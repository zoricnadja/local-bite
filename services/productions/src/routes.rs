use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::handlers::{batches, materials, steps};

pub fn production_routes() -> Router {
    Router::new()
        // Batch collection
        .route("/",
            get(batches::list).post(batches::create))

        // Single batch
        .route("/{id}",
            get(batches::get_one)
            .put(batches::update)
            .delete(batches::delete))

        // Process steps
        .route("/{id}/steps",
            get(steps::list_steps).post(steps::add_step))
        .route("/{id}/steps/{step_id}",
            put(steps::update_step).delete(steps::delete_step))

        // Raw materials within a batch
        .route("/{id}/materials",
            post(materials::add_material))
        .route("/{id}/materials/{material_id}",
            delete(materials::remove_material))
}
