use crate::dtos::login_request::LoginRequest;
use crate::dtos::login_response::LoginResponse;
use crate::dtos::register_request::RegisterRequest;
use crate::service::service::AuthService;
use axum::{
    debug_handler,
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use common::errors::AppError;
use std::sync::Arc;

#[debug_handler]
pub async fn register(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = _auth_service.register_user(payload).await?;
    Ok((StatusCode::CREATED, Json(LoginResponse { token })))
}

#[debug_handler]
pub async fn login(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(_payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = _auth_service.login(_payload).await?;
    Ok(Json(LoginResponse { token }))
}
