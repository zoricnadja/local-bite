use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::repository::repository::RawMaterialRepository;
use crate::service::service::RawMaterialService;

mod handlers;
mod models;
mod repository;
mod service;
mod db;
mod routes;
mod dtos;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "raw_materials=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::create_pool().await?;
    let repository = Arc::new(RawMaterialRepository::new(pool.clone()));
    let service = Arc::new(RawMaterialService::new(repository.clone()));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/raw-materials", routes::raw_material_routes(pool))
        .layer(CorsLayer::permissive())
        .layer(Extension(service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Raw materials service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
