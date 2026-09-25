//! Each service owns its catalogue in its own database.
use crate::{
    errors::{AppError, AppResult},
    middleware::{require_business, require_role, AuthClaims},
    response::{created, ok},
};
use axum::{response::Response, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct NewType {
    pub name: String,
}

pub async fn list(
    AuthClaims(claims): AuthClaims,
    Extension(pool): Extension<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business = require_business(&claims)?;
    let names = sqlx::query_scalar::<_, String>("SELECT name FROM type_catalog WHERE business_id IS NULL OR business_id = $1 ORDER BY lower(name), name")
        .bind(business).fetch_all(&pool).await?;
    Ok(ok(names))
}

pub async fn create(
    AuthClaims(claims): AuthClaims,
    Extension(pool): Extension<PgPool>,
    Json(req): Json<NewType>,
) -> AppResult<Response> {
    require_role(&claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business = require_business(&claims)?;
    let name = req.name.trim();
    if name.is_empty() || name.chars().count() > 100 {
        return Err(AppError::BadRequest(
            "Type name must contain 1–100 characters".into(),
        ));
    }
    if let Some(existing) = sqlx::query_scalar::<_, String>("SELECT name FROM type_catalog WHERE (business_id IS NULL OR business_id = $1) AND lower(name) = lower($2) LIMIT 1")
        .bind(business).bind(name).fetch_optional(&pool).await? {
        return Ok(ok(existing));
    }
    let saved = sqlx::query_scalar::<_, String>("INSERT INTO type_catalog (business_id, name) VALUES ($1, $2) ON CONFLICT (business_id, lower(name)) WHERE business_id IS NOT NULL DO UPDATE SET name = type_catalog.name RETURNING name")
        .bind(business).bind(name).fetch_one(&pool).await?;
    Ok(created(saved))
}
