mod db;
mod dtos;
mod handlers;
mod middleware;
mod models;
mod repository;
mod service;

use crate::db::create_pool;
use crate::handlers::auth::{login, register};
use crate::handlers::businesses::{
    add_worker, create_business, delete_business, get_business, list_workers, list_businesses, update_business,
};
use crate::handlers::users::{delete_user, list_users, me, update_user};
use crate::middleware::auth_middleware::auth_middleware;
use crate::repository::business_repository::BusinessRepository;
use crate::service::business_service::BusinessService;
use crate::service::service::AuthService;
use crate::service::user_service::UserService;
use axum::middleware::from_fn;
use axum::routing::{delete, put};
use axum::{routing::get, routing::post, Extension, Router};
use dotenvy::dotenv;
use http::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    common::events::start_outbox(pool.clone(), "auth");
    let user_repo = Arc::new(repository::repository::UserRepository::new(pool.clone()));
    let auth_service = Arc::new(AuthService::new(user_repo.clone(), jwt_secret.clone()));
    let business_repo = Arc::new(BusinessRepository::new(pool));
    let business_service = Arc::new(BusinessService::new(
        business_repo.clone(),
        user_repo.clone(),
        jwt_secret.clone(),
    ));
    let user_service = Arc::new(UserService::new(user_repo.clone()));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let protected = Router::new()
        .route("/me", get(me))
        .route("/users", get(list_users))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .route("/businesses", post(create_business).get(list_businesses))
        .route("/businesses/{id}", get(get_business))
        .route("/businesses/{id}/trace", get(crate::handlers::businesses::trace_business))
        .route("/businesses/{id}", put(update_business))
        .route("/businesses/{id}", delete(delete_business))
        .route("/businesses/{id}/workers", post(add_worker))
        .route("/businesses/{id}/workers", get(list_workers))
        .route_layer(from_fn(auth_middleware));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .merge(protected)
        .layer(cors)
        .layer(Extension(auth_service))
        .layer(Extension(user_service))
        .layer(Extension(business_service));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Auth service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
