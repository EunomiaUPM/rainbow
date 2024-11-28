use crate::core::{create_agreement, get_agreement_by_id};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::Value;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/agreements/:agreement_id", get(handle_get_agreement_by_id))
        .route("/agreements", post(handle_create_agreement))
}

async fn handle_create_agreement(Json(input): Json<Value>) -> impl IntoResponse {
    let dataset = input.get("dataset");
    if dataset.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Must provide a {\"dataset\": <dataset>}".to_string(),
        )
            .into_response();
    }

    // TODO maybe create a DTO
    let dataset_as_uuid = Uuid::parse_str(dataset.unwrap().as_str().unwrap());
    if dataset_as_uuid.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            "Dataset must be a valid UUIDv4".to_string(),
        )
            .into_response();
    }

    match create_agreement(dataset_as_uuid.unwrap()).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
            .into_response(),
    }
}
async fn handle_get_agreement_by_id(Path(agreement_id): Path<Uuid>) -> impl IntoResponse {
    let agreement = get_agreement_by_id(agreement_id).await;
    match agreement {
        Ok(d) => match d {
            Some(di) => (StatusCode::OK, Json(di)).into_response(),
            None => (StatusCode::NOT_FOUND, "Agreement not found".to_string()).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
            .into_response(),
    }
}
