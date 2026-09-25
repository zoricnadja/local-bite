use crate::dtos::update_user_request::UpdateUserRequest;
use crate::service::service::AuthService;
use crate::service::user_service::UserService;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use common::errors::AppError;
use common::middleware::AuthClaims;
use common::response::ok;
use http::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

#[debug_handler]
pub async fn me(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Extension(_id): Extension<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = _auth_service.get_user(_id).await?;
    let token = _auth_service.issue_token(user.id, &user.email, &user.role, user.business_id)?;
    Ok(([("x-session-token", token)], Json(user)))
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
    if _claims.role == "WORKER" || (_claims.sub != _id && _claims.role != "SYSTEM_ADMIN") {
        return Err(AppError::Forbidden("You cannot delete this account".into()));
    }
    _user_service.delete_user(_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
