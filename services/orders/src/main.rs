use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::repositories::order_item_repository::OrderItemRepository;
use crate::repositories::order_repository::OrderRepository;
use crate::services::order_service::OrderService;

mod db;
mod models;
mod repositories;
mod services;
mod dtos;
mod routes;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "orders=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::create_pool().await?;
    let order_repository = Arc::new(OrderRepository::new(pool.clone()));
    let order_item_repository = Arc::new(OrderItemRepository::new(pool.clone()));
    let order_service = Arc::new(OrderService::new(order_repository.clone(), order_item_repository.clone()));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/orders", routes::order_routes(pool))
        .layer(CorsLayer::permissive())
        .layer(Extension(order_service))
        .layer(Extension(order_repository))
        .layer(Extension(order_item_repository));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3005".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Orders service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
