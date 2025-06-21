use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{RegisterInput, LoginInput};
use crate::services::user_service::{register_user, login_user};
use crate::db::get_pg_pool;

pub async fn register(
    data: web::Json<RegisterInput>,
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    match register_user(&pool, data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn login(
    data: web::Json<LoginInput>,
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    match login_user(&pool, data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user), // Later, replace with JWT
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}
