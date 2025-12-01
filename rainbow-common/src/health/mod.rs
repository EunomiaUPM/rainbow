use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use reqwest::StatusCode;

pub struct HealthRouter;
impl HealthRouter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn router(self) -> Router {
        Router::new().route("/health", get(Self::get_health))
    }
    async fn get_health() -> impl IntoResponse {
        (StatusCode::OK, "OK").into_response()
    }
}
