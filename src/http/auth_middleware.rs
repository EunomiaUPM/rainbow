use axum::body::Body;
use axum::extract::{FromRequest, Request};
use axum::extract::rejection::JsonRejection;
use axum::http::HeaderMap;
use axum::Json;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde_json::Value;
use tracing::info;
use tracing::log::debug;

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
