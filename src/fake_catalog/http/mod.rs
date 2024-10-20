use crate::fake_catalog::lib as lib_fake_catalog;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde_json::Value;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/catalogs/datasets/:dataset_id", get(get_dataset_by_id))
        .route("/catalogs/datasets", post(create_dataset))
        .route("/catalogs/datasets/:dataset_id", delete(delete_dataset))
}


async fn get_dataset_by_id(Path(dataset_id): Path<Uuid>) -> impl IntoResponse {
    let dataset = lib_fake_catalog::get_dataset_by_id(dataset_id);
    match dataset {
        Ok(d) => match d {
            Some(ref di) => (StatusCode::OK, serde_json::to_string(&d).unwrap()),
            None => (StatusCode::NOT_FOUND, "Dataset not found".to_string()),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    }
}
async fn create_dataset(Json(input): Json<Value>) -> impl IntoResponse {
    let endpoint = input.get("endpoint");
    if endpoint.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Must provide a {\"endpoint\": <endpoint>}".to_string(),
        );
    }
    let endpoint = endpoint.unwrap().to_string().replace("\"", "");
    match lib_fake_catalog::create_dataset(endpoint) {
        Ok(d) => (StatusCode::OK, serde_json::to_string(&d).unwrap()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    }
}
async fn delete_dataset(Path(dataset_id): Path<Uuid>) -> impl IntoResponse {
    let transaction = lib_fake_catalog::delete_dataset(dataset_id);
    match transaction {
        Ok(d) => (StatusCode::OK, "Ok".to_string()),
        Err(_) => (StatusCode::NOT_FOUND, "Not found".to_string()),
    }
}
