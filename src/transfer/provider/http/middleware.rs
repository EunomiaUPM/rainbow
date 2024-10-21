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
    // redirijo al verifier -> Me devuelve un OK más la identidad (JWT)
    // DNI
    // CLIENT

    let response = next.run(request).await;
    Ok(response)
}

pub async fn authorization_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Authorization middleware");

    // Tiraríamos contra el motor de políticas
    // KeyRock??? + OPENFGA
    // CLIENT
    // resuelvo el agreement - tu endpoint tiene asocidad una regla que vincula agreement, endpoint y DNI
    // IF OK -> OK()
    // IF NOK -> Error

    let response = next.run(request).await;
    Ok(response)
}

pub async fn protocol_rules_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Protocol rules middleware");
    // request.

    let response = next.run(request).await;
    Ok(response)
}