use crate::db::create_pool;
use crate::repositories::product_repository::ProductRepository;
use crate::services::certificate_service::CertificateService;
use crate::services::image_service::ImageService;
use crate::services::product_service::ProductService;
use crate::services::provenance_service::ProvenanceService;
use crate::services::qr_service::QrService;
use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod reservations;
mod output_consumer;
mod db;
mod dtos;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "product=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into());
    std::fs::create_dir_all(format!("{}/images", uploads_dir))?;
    std::fs::create_dir_all(format!("{}/qr", uploads_dir))?;
    tracing::info!("Uploads directory: {}", uploads_dir);

    let pool = create_pool().await?;
    common::events::start_outbox(pool.clone(), "products");
    output_consumer::start(pool.clone());
    let product_repository = Arc::new(ProductRepository::new(pool.clone()));
    let product_service = Arc::new(ProductService::new(product_repository.clone()));
    let image_service = Arc::new(ImageService::new(
        product_repository.clone(),
        uploads_dir.clone(),
    ));
    let qr_service = Arc::new(QrService::new(
        product_repository.clone(),
        uploads_dir.clone(),
    ));
    let provenance_service = Arc::new(ProvenanceService::new(product_repository.clone()));
    let certificate_service = Arc::new(CertificateService::new(provenance_service.clone()));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/internal/reservations/{id}", axum::routing::post(reservations::reserve))
        .route("/internal/reservations/{id}/release/{farm}", axum::routing::post(reservations::release))
        .layer(Extension(pool.clone()))
        .nest("/products", routes::product_routes())

        .layer(CorsLayer::permissive())
        .layer(Extension(provenance_service))
        .layer(Extension(certificate_service))
        .layer(Extension(product_service))
        .layer(Extension(image_service))
        .layer(Extension(qr_service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3003".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Product service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
