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

use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::consumer::core::ds_protocol::DSProtocolContractNegotiationConsumerTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{middleware, Extension, Json, Router};
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;

pub struct DSProtocolContractNegotiationConsumerRouter<T>
where
    T: DSProtocolContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

#[derive(Clone)]
struct ExtractedParams {
    callback_id: Option<String>,
    consumer_pid: String,
}

impl<T> DSProtocolContractNegotiationConsumerRouter<T>
where
    T: DSProtocolContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/:callback_id/negotiations/:consumer_pid/offers",
                post(Self::handle_post_consumer_offers),
            )
            .route(
                "/negotiations/:consumer_pid/offers",
                post(Self::handle_post_consumer_offers),
            )
            .route(
                "/:callback_id/negotiations/:consumer_pid/agreement",
                post(Self::handle_post_agreement),
            )
            .route(
                "/negotiations/:consumer_pid/agreement",
                post(Self::handle_post_agreement),
            )
            .route(
                "/:callback_id/negotiations/:consumer_pid/events",
                post(Self::handle_post_events),
            )
            .route(
                "/negotiations/:consumer_pid/events",
                post(Self::handle_post_events),
            )
            .route(
                "/:callback_id/negotiations/:consumer_pid/termination",
                post(Self::handle_post_termination),
            )
            .route(
                "/negotiations/:consumer_pid/termination",
                post(Self::handle_post_termination),
            )
            .route_layer(middleware::from_fn(Self::extract_params))
            .route("/negotiations/offers", post(Self::handle_post_offers))
            .with_state(self.service)
    }

    async fn extract_params(req: Request, next: Next) -> Response {
        let uri = req.uri().path();
        let parts: Vec<&str> = uri.split('/').filter(|s| !s.is_empty()).collect();
        let (callback_id, consumer_pid) = if parts.len() == 3 {
            (None, parts[1].to_string())
        } else {
            (Some(parts[0].to_string()), parts[2].to_string())
        };
        let mut req = req;
        req.extensions_mut().insert(ExtractedParams { callback_id, consumer_pid });
        next.run(req).await
    }

    async fn handle_post_offers(
        State(service): State<Arc<T>>,
        input: Result<Json<ContractOfferMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /negotiations/offers");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        match service.post_offers(input).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_consumer_offers(
        State(service): State<Arc<T>>,
        Extension(params): Extension<ExtractedParams>,
        input: Result<Json<ContractOfferMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /{}/negotiations/{}/offers",
            params.callback_id.unwrap_or("".to_string()),
            params.consumer_pid.to_string()
        );
        let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        match service.post_consumer_offers(consumer_pid, input).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_agreement(
        State(service): State<Arc<T>>,
        Extension(params): Extension<ExtractedParams>,
        input: Result<Json<ContractAgreementMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /{}/negotiations/{}/agreement",
            params.callback_id.unwrap_or("".to_string()),
            params.consumer_pid.to_string()
        );
        let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        match service.post_agreement(consumer_pid, input).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_events(
        State(service): State<Arc<T>>,
        Extension(params): Extension<ExtractedParams>,
        input: Result<Json<ContractNegotiationEventMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /{}/negotiations/{}/events",
            params.callback_id.unwrap_or("".to_string()),
            params.consumer_pid.to_string()
        );
        let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        match service.post_events(consumer_pid, input).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_termination(
        State(service): State<Arc<T>>,
        Extension(params): Extension<ExtractedParams>,
        input: Result<Json<ContractTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /{}/negotiations/{}/termination",
            params.callback_id.unwrap_or("".to_string()),
            params.consumer_pid.to_string()
        );
        let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        match service.post_termination(consumer_pid, input).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }
}
