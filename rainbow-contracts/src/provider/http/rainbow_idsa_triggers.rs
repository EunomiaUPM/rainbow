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
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::{OdrlAgreement, OdrlOffer};
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::protocol::contract::{CNValidate, ContractNegotiationMessages};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_provider::entities::cn_process;
use rainbow_db::contracts_provider::repo::EditContractNegotiationProcess;
use rainbow_db::contracts_provider::repo::{
    NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess,
    CONTRACT_PROVIDER_REPO,
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/negotiations/rpc/setup-offer", post(setup_offer))
        .route(
            "/api/v1/negotiations/rpc/setup-agreement",
            post(setup_agreement),
        )
        .route(
            "/api/v1/negotiations/rpc/setup-finalization",
            post(setup_finalization),
        )
        .route(
            "/api/v1/negotiations/rpc/setup-termination",
            post(setup_termination),
        )
}

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client")
});

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupOfferRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OdrlOffer,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupOfferResponse {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OdrlOffer,
    pub message: ContractAckMessage,
}

async fn setup_offer(input: Result<Json<SetupOfferRequest>, JsonRejection>) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    let is_reoffer = input.provider_pid.clone().is_some() && input.consumer_pid.clone().is_some();

    // validate consumer_participant_id
    let participant_id = input.consumer_participant_id.clone();
    let consumer = CONTRACT_PROVIDER_REPO.get_participant_by_p_id(participant_id).await.unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    let consumer_base_url = consumer.unwrap().base_url;

    if is_reoffer {
        // validate consumer
        let consumer = CONTRACT_PROVIDER_REPO
            .get_cn_processes_by_consumer_id(input.consumer_pid.clone().unwrap())
            .await
            .unwrap(); // errors;
        if consumer.is_none() {
            return (StatusCode::NOT_FOUND, "consumer not found").into_response();
        }

        // validate provider
        let provider = CONTRACT_PROVIDER_REPO
            .get_cn_processes_by_provider_id(&input.provider_pid.clone().unwrap())
            .await
            .unwrap(); // errors;
        if provider.is_none() {
            return (StatusCode::NOT_FOUND, "provider not found").into_response();
        }

        // validate correlation
        if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
            return (
                StatusCode::BAD_REQUEST,
                "no correlation between consumer and provider",
            )
                .into_response();
        }
    }

    // validate offer
    if input.odrl_offer.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "offer bad").into_response();
    }

    // create message
    let provider_pid = get_urn(None);
    let contract_offer_message = ContractOfferMessage {
        provider_pid: provider_pid.to_string(),
        odrl_offer: input.odrl_offer.clone(),
        ..Default::default()
    };

    let req: reqwest::Response;
    if is_reoffer == false {
        // send message to consumer
        req = HTTP_CLIENT
            .post(&format!("{}/negotiations/offers", consumer_base_url))
            .json(&contract_offer_message)
            .send()
            .await
            .unwrap();
    } else {
        // send message to consumer
        req = HTTP_CLIENT
            .post(&format!(
                "{}/negotiations/{}/offers",
                consumer_base_url,
                input.consumer_pid.clone().unwrap()
            ))
            .json(&contract_offer_message)
            .send()
            .await
            .unwrap();
    }

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal error in consumer",
        )
            .into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();
    println!("{:?}", response);

    // persist cn_process
    let cn_process: cn_process::Model;
    if is_reoffer == false {
        cn_process = CONTRACT_PROVIDER_REPO
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: Some(get_urn_from_string(&response.provider_pid).unwrap()),
                consumer_id: Some(get_urn_from_string(&response.consumer_pid).unwrap()),
                state: response.state,
                initiated_by: ConfigRoles::Provider,
            })
            .await
            .unwrap(); // errors
    } else {
        cn_process = CONTRACT_PROVIDER_REPO
            .get_cn_processes_by_consumer_id(input.consumer_pid.clone().unwrap())
            .await
            .unwrap()
            .unwrap(); // errors
    }

    // persist cn_message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            cn_process.cn_process_id.parse().unwrap(),
            NewContractNegotiationMessage {
                _type: ContractNegotiationMessages::ContractOfferMessage.to_string(),
                from: ConfigRoles::Provider.to_string(),
                to: ConfigRoles::Consumer.to_string(),
                content: serde_json::to_value(contract_offer_message).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    // persist cn_offer
    let cn_offer = CONTRACT_PROVIDER_REPO
        .create_cn_offer(
            cn_process.cn_process_id.parse().unwrap(),
            cn_message.cn_message_id.parse().unwrap(),
            NewContractNegotiationOffer {
                offer_id: get_urn(None),
                offer_content: serde_json::to_value(input.odrl_offer.clone()).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    let cn_ack: ContractAckMessage = cn_process.into();

    let response: SetupOfferResponse;
    if is_reoffer == false {
        response = SetupOfferResponse {
            consumer_participant_id: input.consumer_participant_id.clone(),
            consumer_pid: Some(cn_ack.consumer_pid.clone().parse().unwrap()),
            provider_pid: Some(cn_ack.provider_pid.clone().parse().unwrap()),
            odrl_offer: input.odrl_offer.clone(),
            message: cn_ack,
        };
    } else {
        response = SetupOfferResponse {
            consumer_participant_id: input.consumer_participant_id.clone(),
            consumer_pid: input.consumer_pid,
            provider_pid: input.provider_pid,
            odrl_offer: input.odrl_offer.clone(),
            message: cn_ack,
        };
    }

    (status, Json(response)).into_response()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAgreementRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:agreement")]
    pub odrl_agreement: OdrlAgreement,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAgreementResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:agreement")]
    pub odrl_agreement: OdrlAgreement,
    pub message: ContractAckMessage,
}

async fn setup_agreement(
    input: Result<Json<SetupAgreementRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };

    // validate consumerParticipant
    let participant_id = input.consumer_participant_id.clone();
    let participant = CONTRACT_PROVIDER_REPO.get_participant_by_p_id(participant_id).await.unwrap(); // errors;
    if participant.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }
    let participant_base_url = participant.unwrap().base_url;

    // validate consumer
    let consumer = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider =
        CONTRACT_PROVIDER_REPO.get_cn_processes_by_provider_id(&input.provider_pid).await.unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (
            StatusCode::BAD_REQUEST,
            "no correlation between consumer and provider",
        )
            .into_response();
    }

    // validate agreement
    if input.odrl_agreement.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "agreement bad").into_response();
    }

    // create message
    let contract_agreement_message = ContractAgreementMessage {
        provider_pid: input.provider_pid.clone().to_string(),
        consumer_pid: input.consumer_pid.clone().to_string(),
        odrl_agreement: input.odrl_agreement.clone(),
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(&format!(
            "{}/negotiations/{}/agreement",
            participant_base_url, input.consumer_pid
        ))
        .json(&contract_agreement_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal error in consumer",
        )
            .into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();
    println!("{:?}", response);

    // persist cn_process
    let process_id = get_urn_from_string(&provider.unwrap().cn_process_id.clone()).unwrap();
    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            process_id,
            EditContractNegotiationProcess {
                provider_id: None,
                consumer_id: None,
                state: Option::from(response.state),
            },
        )
        .await
        .unwrap(); // errors

    // persist cn_message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            cn_process.cn_process_id.parse().unwrap(),
            NewContractNegotiationMessage {
                _type: ContractNegotiationMessages::ContractAgreementMessage.to_string(),
                from: ConfigRoles::Provider.to_string(),
                to: ConfigRoles::Consumer.to_string(),
                content: serde_json::to_value(contract_agreement_message).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    // persist cn_offer
    let cn_offer = CONTRACT_PROVIDER_REPO
        .create_cn_offer(
            cn_process.cn_process_id.parse().unwrap(),
            cn_message.cn_message_id.parse().unwrap(),
            NewContractNegotiationOffer {
                offer_id: get_urn(None),
                offer_content: serde_json::to_value(input.odrl_agreement.clone()).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    let cn_ack: ContractAckMessage = cn_process.into();
    let response = SetupAgreementResponse {
        consumer_pid: input.consumer_pid.clone(),
        provider_pid: input.provider_pid.clone(),
        odrl_agreement: input.odrl_agreement.clone(),
        message: cn_ack,
    };

    (status, Json(response)).into_response()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

async fn setup_finalization(
    input: Result<Json<SetupFinalizationRequest>, JsonRejection>,
) -> impl IntoResponse {
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };

    // validate consumerParticipant
    let participant_id = input.consumer_participant_id.clone();
    let participant = CONTRACT_PROVIDER_REPO.get_participant_by_p_id(participant_id).await.unwrap(); // errors;
    if participant.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }
    let participant_base_url = participant.unwrap().base_url;

    // validate consumer
    let consumer = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider =
        CONTRACT_PROVIDER_REPO.get_cn_processes_by_provider_id(&input.provider_pid).await.unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (
            StatusCode::BAD_REQUEST,
            "no correlation between consumer and provider",
        )
            .into_response();
    }

    // create message
    let contract_verification_message = ContractNegotiationEventMessage {
        provider_pid: input.provider_pid.clone(),
        consumer_pid: input.consumer_pid.clone(),
        event_type: NegotiationEventType::Finalized,
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(&format!(
            "{}/negotiations/{}/events",
            participant_base_url, input.consumer_pid
        ))
        .json(&contract_verification_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal error in consumer",
        )
            .into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();
    println!("{:?}", response);

    // persist cn_process
    let process_id = get_urn_from_string(&provider.unwrap().cn_process_id.clone()).unwrap();
    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            process_id,
            EditContractNegotiationProcess {
                provider_id: None,
                consumer_id: None,
                state: Option::from(response.state),
            },
        )
        .await
        .unwrap(); // errors

    // persist cn_message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            cn_process.cn_process_id.parse().unwrap(),
            NewContractNegotiationMessage {
                _type: ContractNegotiationMessages::ContractNegotiationEventMessage.to_string(),
                from: ConfigRoles::Provider.to_string(),
                to: ConfigRoles::Consumer.to_string(),
                content: serde_json::to_value(contract_verification_message).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    let cn_ack: ContractAckMessage = cn_process.into();
    let response = SetupFinalizationResponse {
        consumer_pid: input.consumer_pid.clone(),
        provider_pid: input.provider_pid.clone(),
        message: cn_ack,
    };

    (status, Json(response)).into_response()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
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

    // validate consumerParticipant
    let participant_id = input.consumer_participant_id.clone();
    let participant = CONTRACT_PROVIDER_REPO.get_participant_by_p_id(participant_id).await.unwrap(); // errors;
    if participant.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }
    let participant_base_url = participant.unwrap().base_url;

    // validate consumer
    let consumer = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_consumer_id(input.consumer_pid.clone())
        .await
        .unwrap(); // errors;
    if consumer.is_none() {
        return (StatusCode::NOT_FOUND, "consumer not found").into_response();
    }

    // validate provider
    let provider =
        CONTRACT_PROVIDER_REPO.get_cn_processes_by_provider_id(&input.provider_pid).await.unwrap(); // errors;
    if provider.is_none() {
        return (StatusCode::NOT_FOUND, "provider not found").into_response();
    }

    // validate correlation
    if consumer.unwrap().cn_process_id != provider.clone().unwrap().cn_process_id {
        return (
            StatusCode::BAD_REQUEST,
            "no correlation between consumer and provider",
        )
            .into_response();
    }

    // create message
    let contract_termination_message = ContractTerminationMessage {
        provider_pid: input.provider_pid.clone(),
        consumer_pid: input.consumer_pid.clone(),
        ..Default::default()
    };

    // send message to consumer
    let req = HTTP_CLIENT
        .post(&format!(
            "{}/negotiations/{}/termination",
            participant_base_url, input.consumer_pid
        ))
        .json(&contract_termination_message)
        .send()
        .await
        .unwrap();

    // if status is not CREATED, return error
    let status = req.status();
    if status.clone() != StatusCode::CREATED {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal error in consumer",
        )
            .into_response();
    }

    // response as json
    let response = req.json::<ContractAckMessage>().await.unwrap();
    println!("{:?}", response);

    // persist cn_process
    let process_id = get_urn_from_string(&provider.unwrap().cn_process_id.clone()).unwrap();
    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            process_id,
            EditContractNegotiationProcess {
                provider_id: None,
                consumer_id: None,
                state: Option::from(response.state),
            },
        )
        .await
        .unwrap(); // errors

    // persist cn_message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            cn_process.cn_process_id.parse().unwrap(),
            NewContractNegotiationMessage {
                _type: ContractNegotiationMessages::ContractNegotiationTerminationMessage
                    .to_string(),
                from: ConfigRoles::Provider.to_string(),
                to: ConfigRoles::Consumer.to_string(),
                content: serde_json::to_value(contract_termination_message).unwrap(),
            },
        )
        .await
        .unwrap(); // errors

    let cn_ack: ContractAckMessage = cn_process.into();
    let response = SetupFinalizationResponse {
        consumer_pid: input.consumer_pid.clone(),
        provider_pid: input.provider_pid.clone(),
        message: cn_ack,
    };

    (status, Json(response)).into_response()
}
