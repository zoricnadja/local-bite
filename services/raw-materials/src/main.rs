use crate::repository::repository::RawMaterialRepository;
use crate::service::service::RawMaterialService;
use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod dtos;
mod handlers;
mod models;
mod repository;
mod routes;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "raw_materials=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Starting raw materials server");
    let pool = db::create_pool().await?;
    common::events::start_outbox(pool.clone(), "raw-materials");
    let repository = Arc::new(RawMaterialRepository::new(pool.clone()));
    let service = Arc::new(RawMaterialService::new(repository.clone()));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/internal/production-consumption", axum::routing::post(handlers::consumption::consume))
        .route("/internal/production-consumption/release", axum::routing::post(handlers::consumption::release))
        .nest("/raw_materials", routes::raw_material_routes())
        .layer(CorsLayer::permissive())
        .layer(Extension(pool))
        .layer(Extension(service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Raw materials service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
