use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::materials;

pub fn raw_material_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/",                get(materials::list).post(materials::create))
        .route("/low-stock",       get(materials::low_stock))
        .route("/{id}",            get(materials::get_one)
            .put(materials::update)
            .delete(materials::delete))
        .route("/{id}/adjust",     post(materials::adjust_quantity))
        .with_state(pool)
}