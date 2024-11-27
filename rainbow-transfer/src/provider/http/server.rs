use crate::provider::http::api;
use crate::provider::http::router;
use axum::http::Method;
use axum::Router;
use rainbow_common::misc_router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn create_provider_router() -> Router {
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(router::router())
        .merge(api::router())
        .layer(TraceLayer::new_for_http());
    server
}
