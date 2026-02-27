use axum::{
    extract::{Path, State},
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use common::{
    errors::{AppError, AppResult},
    response::ok,
};
use crate::dtos::clients;
use crate::dtos::provenance_response::ProvenanceResponse;
// ── GET /products/public/:qr_token  — no auth required ───────────────────────
// This is the endpoint that QR codes point to.
// Returns full provenance chain without requiring a JWT.

pub async fn scan(
    Path(qr_token): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    // Look up product by qr_token (public — no farm_id scoping needed)
    let product = sqlx::query_as!(
        crate::models::Product,
        r#"
        SELECT id, farm_id, name, product_type, description, quantity, unit, price,
               batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
               created_at, updated_at
        FROM   products
        WHERE  qr_token  = $1
          AND  is_active = TRUE
          AND  is_deleted = FALSE
        "#,
        qr_token
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Product not found or no longer active".into()))?;

    // Fetch provenance data — no auth token needed for public batch data
    // (Production service's public endpoint doesn't require auth)
    let (farm_name, batch) = tokio::join!(
        clients::fetch_farm_name(product.farm_id, ""),
        async {
            if let Some(bid) = product.batch_id {
                clients::fetch_batch(bid, "").await.ok()
            } else {
                None
            }
        }
    );

    Ok(ok(ProvenanceResponse {
        product,
        farm_name,
        batch,
    }))
}
