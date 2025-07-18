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

use crate::common::core::mates_facade::MatesFacadeTrait;
use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAcceptanceRequest, SetupAcceptanceResponse, SetupRequestRequest, SetupRequestResponse,
    SetupTerminationRequest, SetupTerminationResponse, SetupVerificationRequest, SetupVerificationResponse,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCContractNegotiationConsumerTrait;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::config::ConfigRoles;
use rainbow_common::mates::Mates;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_consumer::entities::cn_process;
use rainbow_db::contracts_consumer::repo::{
    AgreementConsumerRepo, ContractNegotiationConsumerMessageRepo, ContractNegotiationConsumerOfferRepo,
    ContractNegotiationConsumerProcessRepo, NewContractNegotiationMessage, NewContractNegotiationOffer,
    NewContractNegotiationProcess,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;
use urn::Urn;

pub struct DSRPCContractNegotiationConsumerService<T, U, V>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: MatesFacadeTrait + Send + Sync,
{
    repo: Arc<T>,
    client: Client,
    notification_service: Arc<U>,
    mates_facade: Arc<V>,
}

impl<T, U, V> DSRPCContractNegotiationConsumerService<T, U, V>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: MatesFacadeTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>, mates_facade: Arc<V>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { repo, client, notification_service, mates_facade }
    }

    /// Get provider mate based in id
    async fn get_provider_mate(&self, provider_participant_id: &String) -> anyhow::Result<Mates> {
        let mate = self
            .mates_facade
            .get_mate_by_id(provider_participant_id.clone())
            .await
            .map_err(|e| anyhow!("Error parsing mate: {}", e.to_string()))?;
        Ok(mate)
    }

    async fn _get_provider_base_url(&self, provider_participant_id: &String) -> anyhow::Result<String> {
        let mate = self
            .mates_facade
            .get_mate_by_id(provider_participant_id.clone())
            .await
            .map_err(|e| anyhow!("Error parsing mate: {}", e.to_string()))?;
        match mate.base_url {
            Some(base_url) => Ok(base_url),
            None => bail!("Mate with no base_url".to_string()),
        }
    }

    async fn validate_and_get_correlated_provider_process(
        &self,
        consumer_pid_urn: &Urn,
        provider_pid_urn: &Urn,
    ) -> anyhow::Result<cn_process::Model> {
        let consumer_process = self
            .repo
            .get_cn_process_by_consumer_id(consumer_pid_urn.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid_urn.clone()),
                consumer_pid: Option::from(consumer_pid_urn.clone()),
            })?;
        let provider_process = self
            .repo
            .get_cn_process_by_provider_id(provider_pid_urn.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid_urn.clone()),
                consumer_pid: Option::from(consumer_pid_urn.clone()),
            })?;
        if consumer_process.consumer_id != provider_process.consumer_id {
            bail!(IdsaCNError::ValidationError(
                "ProviderPid and ConsumerPid don't coincide".to_string()
            ));
        }
        Ok(consumer_process.into())
    }

    async fn send_protocol_message_to_provider<M: serde::Serialize>(
        &self,
        target_url: String,
        message_payload: &M,
        token: String,
        error_context_provider_pid: Option<Urn>,
        error_context_consumer_pid: Option<Urn>,
        client_type: Option<String>,
    ) -> anyhow::Result<ContractAckMessage> {
        let mut request_builder = self
            .client
            .post(&target_url)
            .header("Authorization", format!("Bearer {}", token));
        if let Some(value) = &client_type {
            request_builder = request_builder.header("Rainbow-Client-Type", value);
        }
        let response = request_builder
            .json(message_payload)
            .send()
            .await
            .map_err(
                |_| DSRPCContractNegotiationConsumerErrors::ProviderNotReachable {
                    provider_pid: error_context_provider_pid.clone(),
                    consumer_pid: error_context_consumer_pid.clone(),
                },
            )?;

        let status = response.status();
        if !status.is_success() {
            bail!(
                DSRPCContractNegotiationConsumerErrors::ProviderInternalError {
                    provider_pid: error_context_provider_pid.clone(),
                    consumer_pid: error_context_consumer_pid.clone(),
                    error: Option::from(response.json().await.unwrap_or_else(|e| json!({"error": e.to_string()}))),
                }
            );
        }

        let ack_message = response.json::<ContractAckMessage>().await.map_err(|_| {
            DSRPCContractNegotiationConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: error_context_provider_pid,
                consumer_pid: error_context_consumer_pid,
            }
        })?;
        Ok(ack_message)
    }

    ///
    ///
    async fn notify_subscribers(&self, subcategory: String, message: Value) -> anyhow::Result<()> {
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory,
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_content: message,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
            })
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T, U, V> DSRPCContractNegotiationConsumerTrait for DSRPCContractNegotiationConsumerService<T, U, V>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: MatesFacadeTrait + Send + Sync,
{
    async fn setup_request(&self, input: SetupRequestRequest, client_type: String) -> anyhow::Result<SetupRequestResponse> {
        let SetupRequestRequest { provider_pid, odrl_offer, provider_participant_id, .. } = input;
        // 1. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or(anyhow!("No base url"))?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or(anyhow!("No token"))?;
        // 2. validate correlation
        // protocol validation??
        // No need of validation since there is no provider or consumer pid at this point
        // 3. create message
        let consumer_pid = get_urn(None);
        let contract_request_message = ContractRequestMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            odrl_offer: odrl_offer.clone(),
            ..Default::default()
        };
        // 4. send message
        let provider_url = format!("{}/negotiations/request", provider_base_url);
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &contract_request_message,
                provider_token,
                None,
                Some(consumer_pid.clone()),
                Some(client_type.clone()),
            )
            .await?;

        debug!("\n\n10. {:?}\n", response);


        // 5. persist process, message and offer
        let is_business = match client_type.as_str() {
            "business" => true,
            _ => false
        };
        let cn_process = self
            .repo
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: Option::from(get_urn_from_string(&response.provider_pid)?),
                consumer_id: Option::from(get_urn_from_string(&response.consumer_pid)?),
                associated_provider: Some(provider_mate.participant_id.clone()),
                is_business,
            })
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        let cn_message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&response.consumer_pid)?,
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractRequestMessage.to_string(),
                    subtype: None,
                    from: ConfigRoles::Consumer.to_string(),
                    to: ConfigRoles::Provider.to_string(),
                    content: serde_json::to_value(contract_request_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        let offer = self
            .repo
            .create_cn_offer(
                get_urn_from_string(&response.consumer_pid)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer {
                    offer_id: None,
                    offer_content: serde_json::to_value(odrl_offer.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;


        // 6. create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupRequestResponse {
            consumer_pid: Option::from(get_urn_from_string(&response.consumer_pid)?),
            provider_pid: Option::from(get_urn_from_string(&response.provider_pid)?),
            odrl_offer: odrl_offer.clone(),
            message: cn_ack,
        };
        // 7. notification service
        self.notify_subscribers(
            "ContractRequestMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": offer
            }),
        )
            .await?;
        // 8. bye
        Ok(response)
    }

    async fn setup_rerequest(&self, input: SetupRequestRequest) -> anyhow::Result<SetupRequestResponse> {
        let SetupRequestRequest { provider_pid, consumer_pid, odrl_offer, provider_participant_id, .. } = input;
        // 1. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or(anyhow!("No base url"))?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or(anyhow!("No token"))?;
        // 2. validate correlation
        let _consumer = self
            .validate_and_get_correlated_provider_process(
                &consumer_pid.clone().unwrap(),
                &provider_pid.clone().unwrap(),
            )
            .await?;
        // 3. create message
        let contract_offer_message = ContractRequestMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone().unwrap(),
            odrl_offer: odrl_offer.clone(),
            ..Default::default()
        };
        // 4. send message
        let provider_url = format!(
            "{}/negotiations/{}/request",
            provider_base_url,
            provider_pid.clone().unwrap()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &contract_offer_message,
                provider_token,
                provider_pid.clone(),
                consumer_pid.clone(),
                None,
            )
            .await?;
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .get_cn_process_by_consumer_id(consumer_pid.clone().unwrap())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: consumer_pid.clone().unwrap(), entity: "Consumer".to_string() })?; // errors
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.consumer_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractRequestMessage.to_string(),
                    subtype: None,
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_offer_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        let offer = self
            .repo
            .create_cn_offer(
                cn_process.consumer_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewContractNegotiationOffer {
                    offer_id: None,
                    offer_content: serde_json::to_value(odrl_offer.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        // 6. create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupRequestResponse {
            consumer_pid: Option::from(get_urn_from_string(&response.consumer_pid)?),
            provider_pid: Option::from(get_urn_from_string(&response.provider_pid)?),
            odrl_offer: odrl_offer.clone(),
            message: cn_ack,
        };
        // 7. notification service
        self.notify_subscribers(
            "ContractRequestMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": offer
            }),
        )
            .await?;
        // 8. bye
        Ok(response)
    }

    async fn setup_acceptance(&self, input: SetupAcceptanceRequest) -> anyhow::Result<SetupAcceptanceResponse> {
        let SetupAcceptanceRequest { provider_pid, consumer_pid, provider_participant_id, .. } = input;
        // 1. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or(anyhow!("No base url"))?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or(anyhow!("No token"))?;
        // 2. validate correlation
        let _consumer =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;
        // 3. create message
        let contract_acceptance_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            event_type: NegotiationEventType::Accepted,
            ..Default::default()
        };
        // 4. send message to provider
        let provider_url = format!(
            "{}/negotiations/{}/events",
            provider_base_url,
            provider_pid.clone()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &contract_acceptance_message,
                provider_token,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
                None,
            )
            .await?;
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: consumer_pid.clone(), entity: "Consumer".to_string() })?; // errors
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.consumer_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationEventMessage.to_string(),
                    subtype: Some("accepted".to_string()),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_acceptance_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        // 6. create response
        let response = SetupAcceptanceResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        // 7. notification service
        self.notify_subscribers(
            "ContractAcceptanceMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
            }),
        )
            .await?;
        // 8. bye
        Ok(response)
    }

    async fn setup_verification(&self, input: SetupVerificationRequest) -> anyhow::Result<SetupVerificationResponse> {
        let SetupVerificationRequest { provider_pid, consumer_pid, provider_participant_id, .. } = input;
        // 1. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or(anyhow!("No base url"))?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or(anyhow!("No token"))?; // 2. validate correlation
        let _consumer =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;
        // 3. create message
        let contract_verification_message = ContractAgreementVerificationMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };
        // 4. send message to provider
        let provider_url = format!(
            "{}/negotiations/{}/agreement/verification",
            provider_base_url,
            provider_pid.clone()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &contract_verification_message,
                provider_token,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
                None,
            )
            .await?;
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: consumer_pid.clone(), entity: "Consumer".to_string() })?; // errors
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.consumer_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractAgreementVerificationMessage.to_string(),
                    subtype: None,
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_verification_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        // 6. create response
        let response = SetupVerificationResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        // 7. notification service
        self.notify_subscribers(
            "ContractVerificationMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
            }),
        )
            .await?;
        // 8. bye
        Ok(response)
    }

    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse> {
        let SetupTerminationRequest { provider_pid, consumer_pid, provider_participant_id, .. } = input;
        // 1. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or(anyhow!("No base url"))?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or(anyhow!("No token"))?;
        // 2. validate correlation
        let _consumer =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;
        // 3. create message
        let contract_termination_message = ContractTerminationMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };
        // 4. send message to provider
        let provider_url = format!(
            "{}/negotiations/{}/termination",
            provider_base_url,
            provider_pid.clone()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &contract_termination_message,
                provider_token,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
                None,
            )
            .await?;
        // 5. persist process, message and offer
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: consumer_pid.clone(), entity: "Consumer".to_string() })?; // errors
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.consumer_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationTerminationMessage.to_string(),
                    subtype: None,
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_termination_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorConsumer::DbErr)?;
        // 6. create response
        let response = SetupTerminationResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        self.notify_subscribers(
            "ContractTerminationMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
            }),
        )
            .await?;
        // 8. bye
        Ok(response)
    }
}
