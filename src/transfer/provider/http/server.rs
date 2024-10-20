use crate::fake_catalog::http as fake_catalog;
use crate::fake_contracts::http as fake_contracts;
use crate::transfer::common::misc_router;
use crate::transfer::provider::http::middleware::{authentication_middleware, authorization_middleware};
use crate::transfer::provider::http::router;
use crate::transfer::provider::http::{api, proxy};
use axum::http::Method;
use axum::{middleware, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn create_provider_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(router::router())
        .merge(proxy::router())
        .merge(api::router())
        .merge(fake_catalog::router())
        .merge(fake_contracts::router())
        // .layer(cors)
        .layer(middleware::from_fn(authorization_middleware)) // TODO put middleware where needed
        .layer(middleware::from_fn(authentication_middleware))
        .layer(TraceLayer::new_for_http());
    server
}