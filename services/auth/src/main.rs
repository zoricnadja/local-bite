mod models;
mod handlers;
mod service;
mod repository;
mod db;
mod middleware;
mod dtos;

use axum::{Router, routing::post, routing::get, Extension};
use std::sync::Arc;
use dotenvy::dotenv;
use crate::db::create_pool;
use crate::handlers::auth::{login, me, register};
use crate::service::service::AuthService;
use crate::service::farm_service::FarmService;
use crate::repository::farm_repository::FarmRepository;
use tower_http::cors::{CorsLayer, Any};
use http::Method;
use axum::middleware::from_fn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::handlers::farms::{add_worker, create_farm, get_farm, list_workers};
use crate::middleware::auth_middleware::auth_middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET")?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "auth=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = create_pool().await?;
    let user_repo = Arc::new(repository::repository::UserRepository::new(pool.clone()));
    let auth_service = Arc::new(AuthService::new(user_repo, jwt_secret.clone()));
    let farm_repo = Arc::new(FarmRepository::new(pool));
    let farm_service = Arc::new(FarmService::new(farm_repo, jwt_secret));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let protected = Router::new()
        .route("/me", get(me))
        .route("/farms", post(create_farm))
        .route("/farms/{id}", get(get_farm))
        .route("/farms/{id}/workers", post(add_worker))
        .route("/farms/{id}/workers", get(list_workers))
        .route_layer(from_fn(auth_middleware));

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .merge(protected)
        .layer(cors)
        .layer(Extension(auth_service))
        .layer(Extension(farm_service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Auth service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
