use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use serde_json::json;

pub fn ok<T: Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(json!({ "data": data }))).into_response()
}

pub fn created<T: Serialize>(data: T) -> Response {
    (StatusCode::CREATED, Json(json!({ "data": data }))).into_response()
}

pub fn no_content() -> Response {
    StatusCode::NO_CONTENT.into_response()
}
