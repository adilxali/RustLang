// src/db.rs
pub async fn init_pg_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
