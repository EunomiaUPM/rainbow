use crate::transfer::consumer::lib::api::{get_all_callbacks, get_callback_by_id};
use crate::transfer::consumer::lib::callbacks_controller::create_new_callback;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;

// TODO CONTINUE HERE
pub fn router() -> Router {
    Router::new()
        .route("/api/v1/callbacks", get(handle_get_all_callbacks))
        .route(
            "/api/v1/callbacks/:callback_id",
            get(handle_get_callback_by_id),
        )
        .route("/api/v1/callbacks", post(handle_create_callback))
}

async fn handle_get_all_callbacks() -> impl IntoResponse {
    let callbacks = get_all_callbacks().await;
    if callbacks.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    (StatusCode::OK, Json(callbacks.unwrap())).into_response()
}

async fn handle_get_callback_by_id(Path(callback_id): Path<Uuid>) -> impl IntoResponse {
    let callbacks = get_callback_by_id(callback_id).await;
    if callbacks.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let callbacks = callbacks.unwrap();
    if callbacks.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    (StatusCode::OK, Json(callbacks.unwrap())).into_response()
}

async fn handle_create_callback() -> impl IntoResponse {
    let callback = create_new_callback().await;
    if callback.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    (StatusCode::CREATED, callback.unwrap().to_string()).into_response()
}
