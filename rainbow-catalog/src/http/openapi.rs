use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use once_cell::sync::Lazy;
use serde_json::Value;

pub fn route_openapi() -> Router {
    Router::new()
        .route("/api/v1/catalog/openapi.json", get(get_open_api))
}

static OPENAPI_JSON: Lazy<Value> = Lazy::new(|| {
    let openapi_yaml = include_str!("./../../openapi/catalog.json");
    let openapi = serde_json::from_str::<Value>(&openapi_yaml).unwrap();
    openapi
});

async fn get_open_api() -> impl IntoResponse {
    (StatusCode::OK, Json(OPENAPI_JSON.clone())).into_response()
}