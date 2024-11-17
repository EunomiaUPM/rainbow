use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/version", get(get_version))
        .route("/.well-known/version", get(get_version))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    #[serde(rename = "@context")]
    context: String,
    protocol_versions: Vec<ProtocolVersionsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtocolVersionsResponse {
    version: String,
    path: String,
}

async fn get_version() -> impl IntoResponse {
    info!("GET /version");
    let response = VersionResponse {
        context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
        protocol_versions: vec![ProtocolVersionsResponse {
            version: "1.0".to_string(),
            path: "/some/path/v1".to_string(),
        }],
    };
    (StatusCode::OK, Json(response))
}
