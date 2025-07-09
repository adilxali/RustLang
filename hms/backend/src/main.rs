// backend/src/main.rs
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

mod routes;
mod models;
mod schema;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::auth::Auth)
            .wrap(middleware::rate_limit::RateLimiter)
            .service(routes::auth::login)
            .service(routes::patients::create_patient)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}