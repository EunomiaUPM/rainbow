use crate::core::api::{catalog_request, CatalogRequestMessage};
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn catalog_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/catalog/request", post(handle_catalog_request))
        .layer(TraceLayer::new_for_http());;
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
            },
            Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        },
        Err(err) => match err {
            JsonRejection::JsonDataError(_) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
            _ => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        }
    }
}