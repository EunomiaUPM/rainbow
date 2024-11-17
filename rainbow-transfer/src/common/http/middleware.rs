use crate::common::err::TransferErrorType;
use crate::common::err::TransferErrorType::NotCheckedError;
use crate::common::lib::common_validations::pids_as_urn_validation;
use crate::common::lib::protocol_transition_rules::protocol_transition_rules;
use crate::common::lib::schema_validation::schema_validation;
use axum::body::{to_bytes, Body, Bytes};
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::request::Parts;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use log::error;
use reqwest::StatusCode;
use serde_json::{json, Error, Value};
use tracing::info;

async fn _extract_json_body(request: &mut Request<Body>) -> anyhow::Result<(Value, Bytes)> {
    let body = std::mem::take(request.body_mut());
    let body_bytes = to_bytes(body, 2024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST);
    if body_bytes.is_err() {
        return Err(anyhow::anyhow!("Failed to decode body"));
    }
    let json_value = serde_json::from_slice::<Value>(&body_bytes.clone().unwrap());
    if json_value.is_err() {
        return Err(anyhow::anyhow!("Failed to decode body"));
    }
    Ok((json_value?, body_bytes.unwrap()))
}
pub async fn schema_validation_middleware(
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    info!("Shape validation middleware");

    let (json_value, body_bytes) = match _extract_json_body(&mut request).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST).into_response();
        }
    };

    match schema_validation(json_value).await {
        Ok(_) => {
            *request.body_mut() = Body::from(body_bytes);
            next.run(request).await
        }
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
        }
    }
}

pub async fn pids_as_urn_validation_middleware(
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    info!("Pids validation middleware");

    let (json_value, body_bytes) = match _extract_json_body(&mut request).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST).into_response();
        }
    };
    match pids_as_urn_validation(json_value).await {
        Ok(_) => {
            *request.body_mut() = Body::from(body_bytes);
            next.run(request).await
        }
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
        }
    }
}

pub async fn validate_incoming_pids_middleware(
    mut request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Validate incoming pids middleware");

    let (json_value, body_bytes) = _extract_json_body(&mut request).await.map_err(|_| {
        println!("Invalid JSON format");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    // logic

    *request.body_mut() = Body::from(body_bytes);
    let response = next.run(request).await;
    Ok(response)
}


pub async fn validate_agreement_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Agreement validation middleware");

    let (json_value, body_bytes) = _extract_json_body(&mut request).await.map_err(|_| {
        println!("Invalid JSON format");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    // logic

    *request.body_mut() = Body::from(body_bytes);
    let response = next.run(request).await;
    Ok(response)
}

pub async fn protocol_rules_middleware(
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    info!("Protocol rules middleware");

    let (json_value, body_bytes) = match _extract_json_body(&mut request).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST).into_response();
        }
    };

    match protocol_transition_rules(json_value).await {
        Ok(_) => {
            *request.body_mut() = Body::from(body_bytes);
            next.run(request).await
        }
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
        }
    }
}

pub async fn authentication_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Authentication middleware");
    // Extraemos el header Authorization: Bearer <JWT>
    // Extraemos información del JWT
    // 

    let response = next.run(request).await;
    Ok(response)
}

pub async fn authorization_middleware(
    request: Request,
    next: Next,
) -> anyhow::Result<Response, StatusCode> {
    info!("Authorization middleware");

    // IRENE - de momoneto pass through

    // Tiraríamos contra el motor de políticas
    // KeyRock??? + OPENFGA
    // CLIENT
    // resuelvo el agreement - tu endpoint tiene asocidad una regla que vincula agreement, endpoint y DNI
    // IF OK -> OK()
    // IF NOK -> Error

    let response = next.run(request).await;
    Ok(response)
}
