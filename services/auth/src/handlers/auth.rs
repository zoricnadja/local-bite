use crate::service::service::AuthService;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    debug_handler
};
use common::errors::AppError;
use std::sync::Arc;
use crate::dtos::login_request::LoginRequest;
use crate::dtos::login_response::LoginResponse;
use crate::dtos::register_request::RegisterRequest;

#[debug_handler]
pub async fn register(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    _auth_service.register_user(payload).await?;
    Ok((StatusCode::CREATED, "User registered successfully"))
}

#[debug_handler]
pub async fn login(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(_payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = _auth_service.login(_payload).await?;
    Ok(Json(LoginResponse { token }))
}

#[debug_handler]
pub async fn me(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Extension(user_id): Extension<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = _auth_service.get_user(user_id).await?;
    Ok(Json(user))
}