mod config;
// mod models;
mod routes;
// mod handlers;
// mod services;

use actix_web::{App, HttpServer};
use config::init_pg_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = init_pg_pool().await.expect("Failed to init pool");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::init_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
