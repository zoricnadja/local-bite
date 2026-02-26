use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use crate::{errors::AppError, jwt::{decode_jwt, Claims}};

pub struct AuthClaims(pub Claims);

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization format — use 'Bearer <token>'".into()))?;

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let data = decode_jwt(token, &secret)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        Ok(AuthClaims(data.claims))
    }
}

/// Helper — assert the caller belongs to a specific farm or is SYSTEM_ADMIN.
pub fn require_farm(claims: &Claims) -> Result<uuid::Uuid, AppError> {
    claims
        .farm_id
        .ok_or_else(|| AppError::Forbidden("No farm associated with this account".into()))
}

/// Asserts role is one of the allowed values.
pub fn require_role(claims: &Claims, allowed: &[&str]) -> Result<(), AppError> {
    if allowed.contains(&claims.role.as_str()) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Role '{}' is not allowed for this action",
            claims.role
        )))
    }
}