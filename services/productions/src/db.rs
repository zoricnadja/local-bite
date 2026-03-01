use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool() -> anyhow::Result<PgPool> {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    Ok(pool)
}
