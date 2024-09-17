use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;
use serde_json::Value;
use tracing::info;

pub async fn authentication_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Authentication middleware");

    let response = next.run(request).await;
    Ok(response)
}

pub async fn authorization_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Authorization middleware");

    let response = next.run(request).await;
    Ok(response)
}
