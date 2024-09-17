use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn router() -> Router {
    Router::new().route("/auth", get(get_auth))
}

#[derive(Deserialize, Serialize, Debug)]
struct AuthResponse {
    status: i16,
    message: Option<String>,
}

async fn get_auth() -> impl IntoResponse {
    info!("GET /auth");

    serde_json::to_string(&AuthResponse {
        status: 200,
        message: Some("ok".to_string()),
    })
    .unwrap()
}
