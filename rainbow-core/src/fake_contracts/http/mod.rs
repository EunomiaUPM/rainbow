use crate::fake_contracts::lib as lib_fake_contracts;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::Value;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/agreements/:agreement_id", get(get_agreement_by_id))
        .route("/agreements", post(create_agreement))
}


async fn create_agreement(Json(input): Json<Value>) -> impl IntoResponse {
    let dataset = input.get("dataset");
    if dataset.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Must provide a {\"dataset\": <dataset>}".to_string(),
        );
    }

    // TODO maybe create a DTO
    let dataset_as_uuid = Uuid::parse_str(dataset.unwrap().as_str().unwrap());
    if dataset_as_uuid.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            "Dataset must be a valid UUIDv4".to_string(),
        );
    }

    match lib_fake_contracts::create_agreement(dataset_as_uuid.unwrap()) {
        Ok(d) => (StatusCode::OK, serde_json::to_string(&d).unwrap()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    }
}
async fn get_agreement_by_id(Path(agreement_id): Path<Uuid>) -> impl IntoResponse {
    let agreement = lib_fake_contracts::get_agreement_by_id(agreement_id);
    match agreement {
        Ok(d) => match d {
            Some(ref di) => (StatusCode::OK, serde_json::to_string(&d).unwrap()),
            None => (StatusCode::NOT_FOUND, "Agreement not found".to_string()),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    }
}