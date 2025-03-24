/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
use crate::common::lib::common_validations::pids_as_urn_validation;
use crate::common::lib::protocol_transition_rules::protocol_transition_rules;
use crate::common::lib::schema_validation::schema_validation;
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use axum::body::{to_bytes, Body, Bytes};
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, info};

async fn _extract_json_body(request: &mut Request<Body>) -> anyhow::Result<(Value, Bytes)> {
    let body = std::mem::take(request.body_mut());
    let body_bytes = to_bytes(body, 2024).await.map_err(|_| StatusCode::BAD_REQUEST);
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
    debug!("Shape validation middleware");

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
        },
    }
}

pub async fn pids_as_urn_validation_middleware(
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    debug!("Pids validation middleware");

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
        },
    }
}

pub async fn protocol_rules_middleware<T>(
    transfer_service: State<Arc<T>>,
    mut request: Request<Body>,
    next: Next,
) -> impl IntoResponse
where
    T: DSProtocolTransferProviderTrait + Send + Sync + 'static,
{
    debug!("Protocol rules middleware");

    let (json_value, body_bytes) = match _extract_json_body(&mut request).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST).into_response();
        }
    };

    match protocol_transition_rules(transfer_service.0, json_value).await {
        Ok(_) => {
            *request.body_mut() = Body::from(body_bytes);
            next.run(request).await
        }
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
        },
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
