use axum::http::Method;
use axum::Router;
use rainbow_catalog::http::hl_api as catalog_hl_api_router;
use rainbow_catalog::http::ll_api as catalog_ll_api_router;
use rainbow_catalog::http::policies_api as catalog_policies_api_router;
use rainbow_common::misc_router as misc_router;
use rainbow_contracts::http as contract_router;
use rainbow_transfer::consumer::http::api as consumer_hl_api_router;
use rainbow_transfer::consumer::http::openapi as consumer_redoc_router;
use rainbow_transfer::consumer::http::router as consumer_ll_api_router;
use rainbow_transfer::provider::http::api as provider_hl_api_router;
use rainbow_transfer::provider::http::proxy as provider_http_dataplane_router;
use rainbow_transfer::provider::http::router as provider_ll_api_router;
use tower_http::cors::{Any, CorsLayer};

fn _create_cors_layer() -> CorsLayer {
    // TODO Cors in env
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
}

pub async fn get_provider_routes() -> Router {


    Router::new()
        .merge(misc_router::router())
        .merge(provider_ll_api_router::router())
        .merge(provider_hl_api_router::router())
        .merge(provider_http_dataplane_router::router())
        .merge(catalog_ll_api_router::catalog_router().await.unwrap())
        .merge(catalog_hl_api_router::catalog_api_router().await.unwrap())
        .merge(catalog_policies_api_router::catalog_policies_api_router().await.unwrap())
        .merge(contract_router::router())
        .layer(_create_cors_layer())
}

pub async fn get_consumer_routes() -> Router {
    Router::new()
        .merge(misc_router::router())
        .merge(consumer_ll_api_router::router())
        .merge(consumer_hl_api_router::router())
        .merge(consumer_redoc_router::open_api_setup().unwrap())
        .layer(_create_cors_layer())
}