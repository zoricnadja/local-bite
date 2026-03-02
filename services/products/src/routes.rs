use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::handlers::{products_handler, public_handler, qr_handler, image_handler};

pub fn product_routes() -> Router {
    Router::new()
        // Public QR scan endpoint — no auth (must be before /:id to avoid conflict)
        .route("/public/{qr_token}",         get(public_handler::scan))

        // Collection
        .route("/",                          get(products_handler::list).post(products_handler::create))

        // Single product CRUD
        .route("/{id}",                       get(products_handler::get_one)
            .put(products_handler::update)
            .delete(products_handler::delete))

        // Provenance chain
        .route("/{id}/provenance",            get(products_handler::provenance))

        // Image upload
        .route("/{id}/image",                 post(image_handler::upload_image))

        // QR code
        .route("/{id}/qr",                    get(qr_handler::get_qr))
        .route("/{id}/qr/regenerate",         post(qr_handler::regenerate_qr))
}

/// Static file serving for uploaded images and QR PNGs.
/// Mounted at /uploads  → serves ./uploads directory.
pub fn static_routes() -> Router {
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into());
    Router::new().nest_service("/uploads", ServeDir::new(uploads_dir))
}
