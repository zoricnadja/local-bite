use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    Extension,
};
use std::sync::Arc;
use crate::service::service::AuthService;

pub async fn auth_middleware(
    Extension(auth_service): Extension<Arc<AuthService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = auth_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}