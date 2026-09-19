mod projector;
mod queries;

use axum::{routing::get, Router, Extension};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())).init();
    let pool = PgPoolOptions::new().max_connections(10).connect(&std::env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;
    let connected = Arc::new(AtomicBool::new(false));
    projector::start(pool.clone(), connected.clone());
    let app = Router::new()
        .route("/health", get(move || { let state=connected.clone(); async move { axum::Json(serde_json::json!({"status":"ok","consumer_connected":state.load(Ordering::Relaxed)})) } }))
        .route("/dashboard", get(queries::dashboard))
        .route("/producers", get(queries::producers))
        .route("/internal/provenance/{id}", get(queries::provenance))
        .layer(Extension(pool));
    let port = std::env::var("PORT").unwrap_or_else(|_| "3006".into());
    let listener=tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
