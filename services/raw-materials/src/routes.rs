use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::materials;

pub fn raw_material_routes(pool: PgPool) -> Router {
    Router::new()
        // Collection
        .route("/",           get(materials::list).post(materials::create))
        // Low-stock alert — must come BEFORE /:id to avoid route conflict
        .route("/low-stock",  get(materials::low_stock))
        // Single item
        .route("/:id",        get(materials::get_one)
            .put(materials::update)
            .delete(materials::delete))
        // Adjust quantity (e.g. after using in production)
        .route("/:id/adjust", post(materials::adjust_quantity))
        .with_state(pool)
}
