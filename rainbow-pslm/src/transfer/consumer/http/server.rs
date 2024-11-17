use crate::transfer::common::misc_router;
use crate::transfer::consumer::http::openapi::open_api_setup;
use crate::transfer::consumer::http::{api, proxy, router};
use axum::http::Method;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn create_consumer_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let open_api = open_api_setup().unwrap();

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(router::router())
        .merge(proxy::router())
        .merge(api::router())
        .merge(open_api)
        .layer(cors)
        .layer(TraceLayer::new_for_http());
    server
}