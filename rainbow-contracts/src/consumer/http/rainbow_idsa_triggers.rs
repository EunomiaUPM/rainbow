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
use crate::provider::core::idsa_api_errors::IdsaCNError;
use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use once_cell::sync::Lazy;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{ContractNegotiationEventMessage, NegotiationEventType};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::OfferTypes;
use rainbow_common::protocol::contract::CNValidate;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_consumer::entities::cn_process;
use rainbow_db::contracts_consumer::repo::{NewContractNegotiationProcess, CONTRACT_CONSUMER_REPO};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/negotiations/rpc/setup-request", post(setup_request))
        .route("/api/v1/negotiations/rpc/setup-acceptance", post(setup_acceptance))
        .route("/api/v1/negotiations/rpc/setup-verification", post(setup_verification))
        .route("/api/v1/negotiations/rpc/setup-termination", post(setup_termination))
}

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client")
});

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupRequestRequest {
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OfferTypes,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupRequestResponse {
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OfferTypes,
    pub message: ContractAckMessage,
}

async fn setup_request(
    input: Result<Json<SetupRequestRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    let is_rerequest = input.provider_pid.clone().is_some() && input.consumer_pid.clone().is_some();

    if is_rerequest {
        // validate consumer
        let consumer = CONTRACT_CONSUMER_REPO
            .get_cn_process_by_consumer_id(input.consumer_pid.clone().unwrap())
            .await
            .unwrap(); // errors;
        if consumer.is_none() {
            return (StatusCode::NOT_FOUND, "consumer not found").into_response();
        }

        // validate provider
        let provider = CONTRACT_CONSUMER_REPO
            .get_cn_process_by_provider_id(input.provider_pid.clone().unwrap())
            .await
            .unwrap(); // errors;
        if provider.is_none() {
            return (StatusCode::NOT_FOUND, "provider not found").into_response();
        }

        // validate correlation
        if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
            return (StatusCode::BAD_REQUEST, "no correlation between consumer and provider").into_response();
        }
    }


    let is_offer_err = match input.odrl_offer.clone() {
        OfferTypes::MessageOffer(message_offer) => message_offer.validate().is_err(),
        OfferTypes::Offer(offer) => offer.validate().is_err(),
        OfferTypes::Other(_) => false
    };
    if is_offer_err {
        return (StatusCode::BAD_REQUEST, "offer bad").into_response();
    }


    // create message
    let contract_offer_message = ContractRequestMessage {
        provider_pid: input.provider_pid.clone(),
        consumer_pid: input.consumer_pid.clone().unwrap_or(get_urn(None)),
        odrl_offer: match input.odrl_offer.clone() {
            OfferTypes::MessageOffer(message_offer) => OfferTypes::MessageOffer(message_offer),
            OfferTypes::Offer(offer) => OfferTypes::Offer(offer),
            _ => {
                return (StatusCode::BAD_REQUEST, "offer bad").into_response();
            }
        },
        ..Default::default()
    };

    // send message to provider
    let req: reqwest::Response;
    if is_rerequest == false {
        req = HTTP_CLIENT
            .post("http://127.0.0.1:1234/negotiations/request")
            .json(&contract_offer_message)
            .send()
            .await
            .unwrap();
    } else {
        req = HTTP_CLIENT
            .post(format!("http://127.0.0.1:1234/negotiations/{}/request", input.provider_pid.clone().unwrap().to_string()))
            .json(&contract_offer_message)
            .send()
            .await
            .unwrap();
    }
    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        let body = req.text().await.unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("internal error in provider, {}", body)).into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();
    println!("{:?}", response);

    // persist cn_process
    let cn_process: cn_process::Model;
    if is_rerequest == false {
        cn_process = CONTRACT_CONSUMER_REPO
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: Some(get_urn_from_string(&response.provider_pid).unwrap()),
                consumer_id: Some(get_urn_from_string(&response.consumer_pid).unwrap()),
            })
            .await
            .unwrap(); // errors
    } else {
        cn_process = CONTRACT_CONSUMER_REPO
            .get_cn_process_by_consumer_id(input.consumer_pid.clone().unwrap())
            .await
            .unwrap()
            .unwrap(); // errors
    }

    let cn_ack: ContractAckMessage = cn_process.into();
    let response = SetupRequestResponse {
        consumer_pid: Option::from(get_urn_from_string(&response.consumer_pid).unwrap()),
        provider_pid: Option::from(get_urn_from_string(&response.provider_pid).unwrap()),
        odrl_offer: input.odrl_offer.clone(),
        message: cn_ack,
    };

    (status, Json(response)).into_response()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAcceptanceRequest {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAcceptanceResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

async fn setup_acceptance(
    input: Result<Json<SetupAcceptanceRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    // validate consumer
    let consumer = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_provider_id(input.provider_pid.clone())
        .await
        .unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (StatusCode::BAD_REQUEST, "no correlation between consumer and provider").into_response();
    }

    // create message
    let contract_acceptance_message = ContractNegotiationEventMessage {
        provider_pid: input.provider_pid.clone(),
        consumer_pid: input.consumer_pid.clone(),
        event_type: NegotiationEventType::Accepted,
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(format!("http://127.0.0.1:1234/negotiations/{}/events", input.provider_pid.clone().to_string()))
        .json(&contract_acceptance_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        let body = req.text().await.unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("internal error in provider, {}", body)).into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();

    let response = SetupAcceptanceResponse {
        consumer_pid: get_urn_from_string(&response.consumer_pid).unwrap(),
        provider_pid: get_urn_from_string(&response.provider_pid).unwrap(),
        message: response,
    };

    (status, Json(response)).into_response()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupVerificationRequest {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupVerificationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

async fn setup_verification(
    input: Result<Json<SetupVerificationRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    // validate consumer
    let consumer = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_provider_id(input.provider_pid.clone())
        .await
        .unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (StatusCode::BAD_REQUEST, "no correlation between consumer and provider").into_response();
    }

    // create message
    let contract_verification_message = ContractAgreementVerificationMessage {
        provider_pid: input.provider_pid.clone().to_string(),
        consumer_pid: input.consumer_pid.clone().to_string(),
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(format!("http://127.0.0.1:1234/negotiations/{}/agreement/verification", input.provider_pid.clone().to_string()))
        .json(&contract_verification_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        let body = req.text().await.unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("internal error in provider, {}", body)).into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();

    let response = SetupAcceptanceResponse {
        consumer_pid: get_urn_from_string(&response.consumer_pid).unwrap(),
        provider_pid: get_urn_from_string(&response.provider_pid).unwrap(),
        message: response,
    };

    (status, Json(response)).into_response()
}


#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

async fn setup_termination(
    input: Result<Json<SetupTerminationRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    // validate consumer
    let consumer = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_provider_id(input.provider_pid.clone())
        .await
        .unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (StatusCode::BAD_REQUEST, "no correlation between consumer and provider").into_response();
    }

    // create message
    let contract_verification_message = ContractTerminationMessage {
        provider_pid: input.provider_pid.clone(),
        consumer_pid: input.consumer_pid.clone(),
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(format!("http://127.0.0.1:1234/negotiations/{}/termination", input.provider_pid.clone().to_string()))
        .json(&contract_verification_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        let body = req.text().await.unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("internal error in provider, {}", body)).into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();

    let response = SetupAcceptanceResponse {
        consumer_pid: get_urn_from_string(&response.consumer_pid).unwrap(),
        provider_pid: get_urn_from_string(&response.provider_pid).unwrap(),
        message: response,
    };

    (status, Json(response)).into_response()
}
