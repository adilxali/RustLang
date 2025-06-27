use actix_web::{ web,App,HttpResponse, HttpServer, Responder, dev::{ServiceResponse, ServiceRequest} , middleware::{
    ErrorHandlers, ErrorHandlerResponse
} , HttpMessage, body::MessageBody, Error, Result };
use::actix_web::middleware::{Next};
use diesel::pg::PgConnection;
use diesel::r2d2::{self,ConnectionManager};
use dotenv::dotenv;
use std::env;
use serde::{ Deserialize};
use serde::Serialize;
mod models;
mod schema;
use crate::models::{Todo, NewTodo, UpdateTodo, User};
use crate::schema::todos::dsl::*;
use crate::schema::users::dsl::*;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper}; // for data and filter
// use diesel::{RunQueryDsl};
use diesel::ExpressionMethods; // for filter use

// jwt token and encryption and expiration

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use bcrypt::{hash, verify, DEFAULT_COST};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct TodoFilter {
    completed:Option<bool>,
    limit:Option<i64>,
    page:Option<i64>,
}

// JWT Claims
#[derive(Debug, Serialize,  Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}


// user credentials for login
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// In_memory user store
// fn get_users() -> HashMap<String, String> {
//     let mut users = HashMap::new();
//     users.insert(
//         "admin".to_string(),
//         hash("password123", DEFAULT_COST).expect("Failed to hash password123"),
//     );
//     users
//
// }

// JWT Middleware

async fn jwt_middleware<T>(
    req:ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceRequest,Error> {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer") {
                let token = auth_str.trim_start_matches("Bearer");
                let key= env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let validation = Validation::default();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&key.as_bytes()),
                    &validation,
                ){
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data.claims);
                        let _ = next.call(req).await;
                    }
                    Err(_) => {}
                }
            }

        }

    }
    Err(actix_web::error::ErrorUnauthorized(serde_json::json!({
                "error": "token expired",
            })))
}


async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy!!!" , "message": "Server is running!"}))
}

async fn login(pool: web::Data<DbPool>,cred:web::Json<LoginRequest>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get db connection from pool");
    let user: Result<User, diesel::result::Error> = users
        .filter(username.eq(&cred.username))
        .first(&mut conn);

    match user {
        Ok(user) => {
            if verify(&cred.password,&user.password).unwrap() {
                let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let claims = Claims {
                    sub: user.username,
                    exp: (Utc::now() + Duration::days(1)).timestamp(),
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(key.as_ref()),
                )
                    .expect("Failed to generate token");
                return HttpResponse::Ok().json(serde_json::json!(
                {
                    "status": "success",
                    "token": token
                }
            ));

            }
        }
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "status": "error",
            }))
        }
    }

    // if let Some(hashed_password) = users.get(&cred.username) {
    //     if verify(&cred.password, hashed_password).unwrap_or(false) {
    //         let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    //         let claims = Claims {
    //             sub : cred.username.clone(),
    //             exp : (Utc::now() + Duration::hours(1)).timestamp(),
    //         };
    //         let token = encode(
    //             &Header::default(),
    //             &claims,
    //             &EncodingKey::from_secret(key.as_ref()),
    //         )
    //             .expect("Failed to generate token");
    //         return HttpResponse::Ok().json(serde_json::json!(
    //             {
    //                 "status": "success",
    //                 "token": token
    //             }
    //         ));
    //     }
    // }
    HttpResponse::Unauthorized().json(serde_json::json!({
        "error": "Invalid credentials",
    }))
}

async fn get_todos(db_pool: web::Data<DbPool>, query : web::Query<TodoFilter>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get connection from pool");
    let mut query_builder = todos.into_boxed();

    if let Some(completed_status) = query.completed {
        query_builder = query_builder.filter(completed.eq(completed_status));
    }

    if let Some(limit) = query.limit {
        query_builder = query_builder.limit(limit)
    }

    if let Some(offset) = query.page {
        query_builder = query_builder.offset(offset);
    }

    let results = query_builder
        .load::<Todo>(&mut conn)
        .expect("Error loading todos");
    HttpResponse::Ok().json(results)
}

async fn create_todo(db_pool: web::Data<DbPool>, item:web::Json<NewTodo>) -> impl Responder {
    if item.task.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "Task cannot be empty"}));
    }



    let mut conn = db_pool.get().expect("Failed to get connection from pool");

    let new_todo = NewTodo {
        task: item.task.clone(),
        completed: item.completed.clone(),
    };

    let inserted_todo = diesel::insert_into(todos)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .expect("Error inserting todo");
    HttpResponse::Created().json(inserted_todo)

}

async  fn update_todo(db_pool: web::Data<DbPool>, todo_id:web::Path<i32>,item:web::Json<UpdateTodo>) -> impl Responder {
   if let Some(task_item ) = &item.task {
       if task_item.trim().is_empty() {
           return HttpResponse::BadRequest().json(serde_json::json!({"error": "Task cannot be empty"}));
       }
   }
    let mut conn = db_pool.get().expect("Failed to get connection from pool");

    let update_todo = diesel::update(todos.find(todo_id.into_inner()))
        .set(&*item)
        .returning(Todo:: as_returning())
        .get_result(&mut conn);

    match update_todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Todo Not found"}))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Internal Server Error"})),
    }

}

async fn delete_todo(db_pool: web::Data<DbPool>, todo_id: web::Path<i32>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get connection from pool");

    let deleted_todo = diesel::delete(todos.find(todo_id.into_inner()))
    .execute(&mut conn)
        .expect("Error deleting todos");
    if deleted_todo > 0 {
        HttpResponse::Ok().json(serde_json::json!({"error": "Task deleted"}))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({"error": "Task not found"}))
    }
}

async fn get_todo_by_id(db_pool: web::Data<DbPool>, todo_id: web::Path<i32>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get connection from pool");
    let todo: Result<Todo, diesel::result::Error> = todos
        .find(todo_id.into_inner())
    .first(&mut conn);

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Task not found"}))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Internal Server Error"})),
    }
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {

    dotenv().ok();

    env_logger::init();


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let _pool:DbPool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(_pool.clone()))
            .route("/healthy", web::get().to(health))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/{id}", web::get().to(get_todo_by_id))
            .route("/login", web::post().to(login))
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
