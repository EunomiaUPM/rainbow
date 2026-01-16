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
        Router::new()
            .route("/health", get(Self::get_ok))
            .route("/healthz", get(Self::get_ok))
            .route("/liveness", get(Self::get_ok))
            .route("/readiness", get(Self::get_ok))
    }
    async fn get_ok() -> impl IntoResponse {
        (StatusCode::OK, "OK").into_response()
    }
}
