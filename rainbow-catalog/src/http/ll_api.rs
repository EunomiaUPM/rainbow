use crate::core::ll_api::{catalog_request, dataset_request, CatalogRequestMessage};
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

pub async fn catalog_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/catalog/request", post(handle_catalog_request))
        .route("/catalog/datasets/:id", get(handle_get_dataset));
    Ok(router)
}

async fn handle_catalog_request(
    result: Result<Json<CatalogRequestMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /catalog/request");

    match result {
        Ok(Json(input)) => match catalog_request().await {
            Ok(res) => {
                println!("{:#?}", Json(&res));
                (StatusCode::OK, Json(res)).into_response()
            }
            Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        },
        Err(err) => match err {
            JsonRejection::JsonDataError(_) => {
                (StatusCode::BAD_REQUEST, err.to_string()).into_response()
            }
            _ => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        },
    }
}

async fn handle_get_dataset(result: Result<Path<Uuid>, PathRejection>) -> impl IntoResponse {
    info!("POST /catalog/datasets/:id");

    match result {
        Ok(id) => match dataset_request(id.0).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
        },
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
