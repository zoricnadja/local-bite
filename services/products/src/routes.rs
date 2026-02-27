use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::services::ServeDir;

use crate::handlers::{products, public, qr, upload};

pub fn product_routes(pool: PgPool) -> Router {
    Router::new()
        // Public QR scan endpoint — no auth (must be before /:id to avoid conflict)
        .route("/public/:qr_token",         get(public::scan))

        // Collection
        .route("/",                          get(products::list).post(products::create))

        // Single product CRUD
        .route("/:id",                       get(products::get_one)
            .put(products::update)
            .delete(products::delete))

        // Provenance chain
        .route("/:id/provenance",            get(products::provenance))

        // Image upload
        .route("/:id/image",                 post(upload::upload_image))

        // QR code
        .route("/:id/qr",                    get(qr::get_qr))
        .route("/:id/qr/regenerate",         post(qr::regenerate_qr))

        .with_state(pool)
}

/// Static file serving for uploaded images and QR PNGs.
/// Mounted at /uploads  → serves ./uploads directory.
pub fn static_routes() -> Router {
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into());
    Router::new().nest_service("/uploads", ServeDir::new(uploads_dir))
}
