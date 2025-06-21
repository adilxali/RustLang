use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn init_pg_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
