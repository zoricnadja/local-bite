use axum::{
    routing::{get, put},
    Router,
};

use crate::handlers::orders;

pub fn order_routes() -> Router {
    Router::new()
        // Analytics must be before /:id to avoid route conflict
        .route("/analytics",      get(orders::analytics))

        // Collection
        .route("/",               get(orders::list).post(orders::create))

        // Single order
        .route("/{id}",            get(orders::get_one).delete(orders::delete))

        // Orders by user
        .route("/user/{id}",            get(orders::get_orders_by_user))

        .route("/{id}/status",     put(orders::update_status))
}
