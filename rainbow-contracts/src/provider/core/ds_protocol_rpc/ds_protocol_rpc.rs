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

use crate::provider::core::catalog_odrl_facade::CatalogOdrlFacadeTrait;
use crate::provider::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest, SetupFinalizationResponse,
    SetupOfferRequest, SetupOfferResponse, SetupTerminationRequest, SetupTerminationResponse,
};
use crate::provider::core::ds_protocol_rpc::DSRPCContractNegotiationProviderTrait;
use crate::provider::core::rainbow_entities::rainbow_entities_errors::CnErrorProvider;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_odrl::{ContractRequestMessageOfferTypes, OdrlAgreement, OdrlOffer, OdrlTypes};
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_provider::entities::cn_process;
use rainbow_db::contracts_provider::repo::{
    AgreementRepo, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo, ContractNegotiationProcessRepo,
    EditContractNegotiationProcess, NewAgreement, NewContractNegotiationMessage, NewContractNegotiationOffer,
    NewContractNegotiationProcess, Participant,
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

pub struct DSRPCContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
    client: Client,
    catalog_facade: Arc<V>,
}

impl<T, U, V> DSRPCContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>, catalog_facade: Arc<V>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { repo, notification_service, client, catalog_facade }
    }
    async fn get_consumer_base_url(&self, consumer_participant_id: &Urn) -> anyhow::Result<String> {
        let participant = self
            .repo
            .get_participant_by_p_id(consumer_participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or_else(|| CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Participant".to_string(),
            })?;
        Ok(participant.base_url)
    }
    async fn validate_and_get_correlated_provider_process(
        &self,
        consumer_pid_urn: &Urn,
        provider_pid_urn: &Urn,
    ) -> anyhow::Result<cn_process::Model> {
        let consumer_process = self
            .repo
            .get_cn_processes_by_consumer_id(consumer_pid_urn.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or_else(|| CnErrorProvider::NotFound {
                id: consumer_pid_urn.clone(),
                entity: "ConsumerProcess".to_string(),
            })?;

        let provider_process = self
            .repo
            .get_cn_processes_by_provider_id(provider_pid_urn)
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or_else(|| CnErrorProvider::NotFound {
                id: provider_pid_urn.clone(),
                entity: "ProviderProcess".to_string(),
            })?;

        if consumer_process.provider_id != provider_process.provider_id {
            bail!(
                DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                    provider_pid: get_urn_from_string(&provider_process.provider_id)?,
                    consumer_pid: get_urn_from_string(&consumer_process.provider_id)?,
                }
            );
        }
        Ok(provider_process)
    }

    async fn send_protocol_message_to_consumer<M: serde::Serialize>(
        &self,
        target_url: String,
        message_payload: &M,
        error_context_provider_pid: Option<Urn>,
        error_context_consumer_pid: Option<Urn>,
    ) -> anyhow::Result<ContractAckMessage> {
        let response = self.client.post(&target_url).json(message_payload).send().await.map_err(|_| {
            DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                provider_pid: error_context_provider_pid.clone(),
                consumer_pid: error_context_consumer_pid.clone(),
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            bail!(
                DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                    provider_pid: error_context_provider_pid.clone(),
                    consumer_pid: error_context_consumer_pid.clone(),
                    consumer_error: response.json().await.unwrap_or_else(|e| json!({"error": e.to_string()})),
                }
            );
        }

        let ack_message = response.json::<ContractAckMessage>().await.map_err(|_| {
            DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
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
impl<T, U, V> DSRPCContractNegotiationProviderTrait for DSRPCContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    async fn setup_offer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse> {
        let SetupOfferRequest { odrl_offer, consumer_participant_id, .. } = input;
        // 1. fetch participant id
        let consumer_base_url = self.get_consumer_base_url(&consumer_participant_id).await?;
        // 2. validate correlation
        // protocol validation??
        // No need of validation since there is no provider or consumer pid at this point
        // 2. Validate ODRL policy...
        let resolved_offer = match self.catalog_facade.resolve_odrl_offers(odrl_offer.id.clone()).await {
            Ok(resolver) => resolver,
            Err(_) => bail!(
                DSRPCContractNegotiationProviderErrors::DSProtocolContractNegotiationError(
                    IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "Id not found".to_string()
                    }
                )
            ),
        };
        if odrl_offer.target.clone() != resolved_offer.target.clone().unwrap() {
            bail!(
                DSRPCContractNegotiationProviderErrors::DSProtocolContractNegotiationError(
                    IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "target not coincide".to_string()
                    }
                )
            )
        }
        // 3. create message
        let provider_pid = get_urn(None);
        let contract_offer_message = ContractOfferMessage {
            provider_pid: provider_pid.to_string().parse()?,
            odrl_offer: ContractRequestMessageOfferTypes::OfferMessage(odrl_offer.clone()),
            ..Default::default()
        };
        // 4. send message
        let target_url = format!("{}/negotiations/offers", consumer_base_url);
        let response = self
            .send_protocol_message_to_consumer(
                target_url,
                &contract_offer_message,
                Some(provider_pid.clone()),
                None,
            )
            .await?;
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .create_cn_process(NewContractNegotiationProcess {
                consumer_id: Some(get_urn_from_string(&response.consumer_pid)?),
                state: response.state,
                initiated_by: ConfigRoles::Provider,
            })
            .await
            .map_err(CnErrorProvider::DbErr)?;
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.provider_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractOfferMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_offer_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        let offer = self
            .repo
            .create_cn_offer(
                cn_process.provider_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewContractNegotiationOffer {
                    offer_id: None,
                    offer_content: serde_json::to_value(odrl_offer.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // 6. create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupOfferResponse {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: Some(cn_ack.consumer_pid.clone().parse()?),
            provider_pid: Some(cn_ack.provider_pid.clone().parse()?),
            odrl_offer: odrl_offer.clone(),
            message: cn_ack,
        };

        // 7. notification service
        self.notify_subscribers(
            "ContractOfferMessage".to_string(),
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

    async fn setup_reoffer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse> {
        let SetupOfferRequest { odrl_offer, consumer_participant_id, provider_pid, consumer_pid } = input;
        // 1. fetch participant id
        let consumer_base_url = self.get_consumer_base_url(&consumer_participant_id).await?;
        // 2. validate correlation
        let cn_process = self
            .validate_and_get_correlated_provider_process(
                &consumer_pid.clone().unwrap(),
                &provider_pid.clone().unwrap(),
            )
            .await?;
        // 2. Validate ODRL policy...
        let resolved_offer = match self.catalog_facade.resolve_odrl_offers(odrl_offer.id.clone()).await {
            Ok(resolver) => resolver,
            Err(_) => bail!(
                DSRPCContractNegotiationProviderErrors::DSProtocolContractNegotiationError(
                    IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "Id not found".to_string()
                    }
                )
            ),
        };
        if odrl_offer.target.clone() != resolved_offer.target.clone().unwrap() {
            bail!(
                DSRPCContractNegotiationProviderErrors::DSProtocolContractNegotiationError(
                    IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "target not coincide".to_string()
                    }
                )
            )
        }
        // 3. create message
        let contract_offer_message = ContractOfferMessage {
            consumer_pid: cn_process.consumer_id.map(|a| a.parse().unwrap()),
            provider_pid: cn_process.provider_id.parse()?,
            odrl_offer: ContractRequestMessageOfferTypes::OfferMessage(odrl_offer.clone()),
            ..Default::default()
        };
        // 4. send message
        let target_url = format!(
            "{}/negotiations/{}/offers",
            &consumer_base_url,
            &consumer_pid.clone().unwrap()
        );
        let response = self
            .send_protocol_message_to_consumer(
                target_url,
                &contract_offer_message,
                provider_pid.clone(),
                consumer_pid.clone(),
            )
            .await?;
        // 5. persist process, message and offer
        let cn_process = self
            .repo
            .put_cn_process(
                cn_process.provider_id.parse()?,
                EditContractNegotiationProcess { consumer_id: None, state: Some(response.state) },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.provider_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractOfferMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_offer_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        let offer = self
            .repo
            .create_cn_offer(
                cn_process.provider_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewContractNegotiationOffer {
                    offer_id: None,
                    offer_content: serde_json::to_value(odrl_offer.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // 6. create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupOfferResponse {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid,
            provider_pid: Option::from(provider_pid),
            odrl_offer: odrl_offer.clone(),
            message: cn_ack,
        };

        // 7. notification service
        self.notify_subscribers(
            "ContractOfferMessage".to_string(),
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

    async fn setup_agreement(&self, input: SetupAgreementRequest) -> anyhow::Result<SetupAgreementResponse> {
        let SetupAgreementRequest { consumer_participant_id, consumer_pid, provider_pid } = input;
        // 1. fetch participant id
        let consumer_base_url = self.get_consumer_base_url(&consumer_participant_id).await?;
        // 2. validate correlation
        let cn_process =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;


        // 3. Create and validate agreement
        // 3.1. fetch last valid offer in process
        let cn_process_id = get_urn_from_string(&cn_process.provider_id)?;
        let last_offer_model = self.repo
            .get_last_cn_offers_by_cn_process(cn_process_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or_else(|| CnErrorProvider::NotFound {
                id: cn_process_id.clone(),
                entity: "Offer".to_string(),
            })?;
        let last_offer = serde_json::from_value::<OdrlOffer>(last_offer_model.offer_content)?;

        // 3.2 fetch participants
        let provider_participant = self.repo.get_provider_participant().await?.unwrap();
        let provider_participant_id = get_urn_from_string(&provider_participant.participant_id)?;

        // 3.3 arrange agreement
        let agreement_id = get_urn(None);
        let final_agreement = OdrlAgreement {
            id: agreement_id.clone(),
            profile: last_offer.profile,
            permission: last_offer.permission,
            obligation: last_offer.obligation,
            _type: OdrlTypes::Agreement,
            target: last_offer.target.unwrap(),
            assigner: provider_participant_id.clone(),
            assignee: consumer_participant_id.clone(),
            timestamp: Option::from(chrono::Utc::now().to_rfc3339().to_string()),
            prohibition: last_offer.prohibition,
        };

        debug!("{:?}", final_agreement);

        // 4. create message
        let contract_agreement_message = ContractAgreementMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            odrl_agreement: final_agreement.clone(),
            ..Default::default()
        };
        // 5. send message
        let target_url = format!(
            "{}/negotiations/{}/agreement",
            &consumer_base_url,
            &consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                target_url,
                &contract_agreement_message,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
            )
            .await?;

        // 6. persist process, message and agreement
        let process_id = get_urn_from_string(&cn_process.provider_id)?;
        let cn_process = self
            .repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        // persist cn_message
        let cn_message = self
            .repo
            .create_cn_message(
                cn_process.provider_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractAgreementMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_agreement_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        // persist agreement
        let _ = self
            .repo
            .create_agreement(
                cn_process.provider_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewAgreement {
                    agreement_id: Some(agreement_id.clone()),
                    consumer_participant_id,
                    provider_participant_id,
                    agreement_content: final_agreement.clone(),
                    active: false,
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // 7. Create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupAgreementResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            odrl_agreement: final_agreement.clone(),
            message: cn_ack,
        };

        // 8. notification service
        self.notify_subscribers(
            "ContractAgreementMessage".to_string(),
            json!({
             "process": cn_process,
                        "message": cn_message,
                        "agreement": final_agreement
            }),
        )
            .await?;

        Ok(response)
    }

    async fn setup_finalization(&self, input: SetupFinalizationRequest) -> anyhow::Result<SetupFinalizationResponse> {
        let SetupFinalizationRequest { consumer_participant_id, consumer_pid, provider_pid, .. } = input;
        // 1. fetch participant id
        let consumer_base_url = self.get_consumer_base_url(&consumer_participant_id).await?;
        // 2. validate correlation
        let cn_process =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;
        // 3. create message
        let contract_verification_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            event_type: NegotiationEventType::Finalized,
            ..Default::default()
        };
        // 5. send message
        let target_url = format!(
            "{}/negotiations/{}/events",
            &consumer_base_url,
            &consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                target_url,
                &contract_verification_message,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
            )
            .await?;
        // 6. persist process, message
        let process_id = get_urn_from_string(&cn_process.provider_id.clone())?;
        let cn_process = self
            .repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        // persist cn_message
        let message = self
            .repo
            .create_cn_message(
                cn_process.provider_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationEventMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_verification_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;


        // TODO load into PDP

        // 7. Create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupFinalizationResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            message: cn_ack,
        };

        // 8. notification service
        self.notify_subscribers(
            "ContractNegotiationEventMessage:finalized".to_string(),
            json!({
             "process": cn_process,
                     "process": cn_process,
                    "message": message
            }),
        )
            .await?;

        Ok(response)
    }

    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse> {
        let SetupTerminationRequest { consumer_participant_id, consumer_pid, provider_pid, .. } = input;
        // 1. fetch participant id
        let consumer_base_url = self.get_consumer_base_url(&consumer_participant_id).await?;
        // 2. validate correlation
        let cn_process =
            self.validate_and_get_correlated_provider_process(&consumer_pid.clone(), &provider_pid.clone()).await?;
        // 3. create message
        let contract_termination_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };
        // 4. send message
        let target_url = format!(
            "{}/negotiations/{}/termination",
            &consumer_base_url,
            &consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                target_url,
                &contract_termination_message,
                Option::from(provider_pid.clone()),
                Option::from(consumer_pid.clone()),
            )
            .await?;

        // 5. persist cn_process
        let process_id = get_urn_from_string(&cn_process.provider_id.clone())?;
        let cn_process = self
            .repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        // persist cn_message
        let message = self
            .repo
            .create_cn_message(
                cn_process.provider_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationTerminationMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_termination_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;
        // 6. create response
        let cn_ack: ContractAckMessage = cn_process.clone().into();
        let response = SetupTerminationResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            message: cn_ack,
        };
        // 7. notification service
        self.notify_subscribers(
            "ContractNegotiationTerminationMessage".to_string(),
            json!({
             "process": cn_process,
                     "process": cn_process,
                    "message": message
            }),
        )
            .await?;

        Ok(response)
    }
}
