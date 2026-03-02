use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::orders;

pub fn order_routes(pool: PgPool) -> Router {
    Router::new()
        // Analytics must be before /:id to avoid route conflict
        .route("/analytics",      get(orders::analytics))

        // Collection
        .route("/",               get(orders::list).post(orders::create))

        // Single order
        .route("/{id}",            get(orders::get_one).delete(orders::delete))
        .route("/{id}/status",     put(orders::update_status))

        .with_state(pool)
}
