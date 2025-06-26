use actix_web::{ web,App,HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{self,ConnectionManager};
use dotenv::dotenv;
use std::env;
mod models;
mod schema;

use crate::models::{Todo, NewTodo, UpdateTodo};
use crate::schema::todos::dsl::*;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper}; // for data and filter
// use diesel::{RunQueryDsl};
// use diesel::ExpressionMethods; // for filter use

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy!!!" , "message": "Server is running!"}))
}

async fn get_todos(db_pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get connection from pool");
    let results = todos
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
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
