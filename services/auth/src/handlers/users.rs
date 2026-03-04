use std::sync::Arc;
use axum::{debug_handler, Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use uuid::Uuid;
use common::errors::AppError;
use common::middleware::AuthClaims;
use common::response::ok;
use crate::dtos::update_user_request::UpdateUserRequest;
use crate::service::service::AuthService;
use crate::service::user_service::UserService;

#[debug_handler]
pub async fn me(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Extension(_id): Extension<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = _auth_service.get_user(_id).await?;
    Ok(Json(user))
}

#[debug_handler]
pub async fn list_users(
    Extension(_user_service): Extension<Arc<UserService>>,
    AuthClaims(_claims): AuthClaims,
) -> Result<impl IntoResponse, AppError> {
    let users = _user_service.list_users(&_claims).await?;
    Ok(ok(users))
}

#[debug_handler]
pub async fn update_user(
    Extension(_user_service): Extension<Arc<UserService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = _user_service.update_user(&_claims, _id, payload).await?;
    Ok(ok(user))
}

#[debug_handler]
pub async fn delete_user(
    Extension(_user_service): Extension<Arc<UserService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    _user_service.delete_user(_id).await?;
    Ok(StatusCode::NO_CONTENT)
}