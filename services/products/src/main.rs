use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod routes;
mod db;
mod service;
mod repository;
mod handlers;
mod models;
mod dtos;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "product=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Ensure uploads directories exist
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into());
    std::fs::create_dir_all(format!("{}/images", uploads_dir))?;
    std::fs::create_dir_all(format!("{}/qr", uploads_dir))?;
    tracing::info!("Uploads directory: {}", uploads_dir);

    let pool = db::create_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/products", routes::product_routes(pool))
        .merge(routes::static_routes())
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3004".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Product service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

