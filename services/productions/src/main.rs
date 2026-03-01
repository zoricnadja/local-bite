use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::repositories::batch_repository::BatchRepository;
use crate::repositories::raw_materials_repository::RawMaterialsRepository;
use crate::repositories::step_repository::StepRepository;
use crate::services::batch_service::BatchService;
use crate::services::raw_materials_service::RawMaterialsService;
use crate::services::step_service::StepService;

mod dtos;
mod models;
mod services;
mod repositories;
mod handlers;
mod db;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "production=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::create_pool().await?;
    let raw_materials_repository = Arc::new(RawMaterialsRepository::new(pool.clone()));
    let step_repository = Arc::new(StepRepository::new(pool.clone()));
    let batch_repository = Arc::new(BatchRepository::new(pool.clone()));
    let raw_materials_service = Arc::new(
        RawMaterialsService::new(batch_repository.clone(), raw_materials_repository.clone(), step_repository.clone()));
    let batch_service = Arc::new(BatchService::new(
        batch_repository.clone(), step_repository.clone(), raw_materials_repository.clone(), raw_materials_service.clone()));
    let step_service = Arc::new(StepService::new(batch_repository.clone(), batch_service.clone(), step_repository.clone()));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/", routes::production_routes())
        .layer(CorsLayer::permissive())
        .layer(Extension(batch_service))
        .layer(Extension(step_service))
        .layer(Extension(raw_materials_service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3003".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Production service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
