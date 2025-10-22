/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::common::CNControllerTypes;
use crate::provider::core::rainbow_entities::rainbow_entities_errors::CnErrorProvider;
use crate::provider::core::rainbow_entities::rainbow_entities_types::{
    EditAgreementRequest, EditContractNegotiationMessageRequest, EditContractNegotiationOfferRequest,
    EditContractNegotiationRequest, NewAgreementRequest, NewContractNegotiationMessageRequest,
    NewContractNegotiationOfferRequest, NewContractNegotiationRequest,
};
use crate::provider::core::rainbow_entities::RainbowEntitiesContractNegotiationProviderTrait;
use axum::async_trait;
use rainbow_db::contracts_provider::entities::cn_process::Model;
use rainbow_db::contracts_provider::repo::{
    AgreementRepo, CnErrors, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo,
    ContractNegotiationProcessRepo,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowEntitiesContractNegotiationProviderService<T, U>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowEntitiesContractNegotiationProviderService<T, U>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowEntitiesContractNegotiationProviderTrait for RainbowEntitiesContractNegotiationProviderService<T, U>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_cn_processes(&self, client_type: Option<String>) -> anyhow::Result<Vec<Model>> {
        let processes = self.repo.get_all_cn_processes(None, None, client_type).await.map_err(CnErrorProvider::DbErr)?;
        Ok(processes)
    }

    async fn get_batch_processes(&self, cn_ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>> {
        let processes = self.repo.get_batch_cn_processes(cn_ids).await.map_err(CnErrorProvider::DbErr)?;
        Ok(processes)
    }

    async fn get_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<Model> {
        let process = self
            .repo
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() })?;
        Ok(process)
    }

    async fn get_cn_process_by_provider(&self, provider_id: Urn) -> anyhow::Result<Model> {
        let process =
            self.repo.get_cn_processes_by_provider_id(&provider_id).await.map_err(CnErrorProvider::DbErr)?.ok_or(
                CnErrorProvider::ProviderNotFound { provider_id, entity: CNControllerTypes::Process.to_string() },
            )?;
        Ok(process)
    }

    async fn get_cn_process_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<Model> {
        let process = self
            .repo
            .get_cn_processes_by_consumer_id(consumer_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::ConsumerNotFound { consumer_id, entity: CNControllerTypes::Process.to_string() })?;
        Ok(process)
    }

    async fn get_cn_processes_by_participant(&self, participant_id: String, client_type: Option<String>) -> anyhow::Result<Vec<Model>> {
        let processes = self
            .repo
            .get_cn_processes_by_participant_id(participant_id, client_type)
            .await
            .map_err(CnErrorProvider::DbErr)?;
        Ok(processes)
    }

    async fn post_cn_process(&self, input: NewContractNegotiationRequest) -> anyhow::Result<Model> {
        let process = self.repo.create_cn_process(input.into()).await.map_err(CnErrorProvider::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationProcess".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&process)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        Ok(process)
    }

    async fn put_cn_process(&self, process_id: Urn, input: EditContractNegotiationRequest) -> anyhow::Result<Model> {
        let process = self.repo.put_cn_process(process_id.clone(), input.into()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorProvider::DbErr(err),
        })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationProcess".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&process)?,
                message_operation: RainbowEventsNotificationMessageOperation::Update,
            })
            .await?;
        Ok(process)
    }

    async fn delete_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_cn_process(process_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorProvider::NotFound { id: process_id.clone(), entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorProvider::DbErr(err),
        })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationProcess".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "ContractNegotiationProcess",
                    "@id": process_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_cn_messages(
        &self,
    ) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::cn_message::Model>> {
        let cn_messages = self.repo.get_all_cn_messages(None, None).await.map_err(CnErrorProvider::DbErr)?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_cn_process_id(process_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_message::Model> {
        let cn_message = self
            .repo
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() })?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_cn_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_provider_id(provider_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::ProviderNotFound { provider_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_consumer_id(consumer_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::ConsumerNotFound { consumer_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn post_cn_message_by_cn_process(
        &self,
        process_id: Urn,
        input: NewContractNegotiationMessageRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_message::Model> {
        let cn_message =
            self.repo.create_cn_message(process_id.clone(), input.into()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&cn_message)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        Ok(cn_message)
    }

    async fn put_cn_message_by_cn_process(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: EditContractNegotiationMessageRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_message::Model> {
        let cn_message =
            self.repo.put_cn_message(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorProvider::DbErr(err),
                }
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&cn_message)?,
                message_operation: RainbowEventsNotificationMessageOperation::Update,
            })
            .await?;
        Ok(cn_message)
    }

    async fn delete_cn_message_by_cn_process(&self, process_id: Urn, message_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_cn_message(process_id.clone(), message_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            CnErrors::CNMessageNotFound => {
                CnErrorProvider::NotFound { id: message_id.clone(), entity: CNControllerTypes::Message.to_string() }
            }
            _ => CnErrorProvider::DbErr(err),
        })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "ContractNegotiationMessage",
                    "@id": message_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_cn_offers_by_cn_process_id(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::cn_offer::Model>> {
        let offers = self.repo.get_all_cn_offers_by_cn_process(process_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Offer.to_string() }
            }
            _ => CnErrorProvider::DbErr(err),
        })?;
        Ok(offers)
    }

    async fn get_last_cn_offers_by_cn_process_id(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_offer::Model> {
        let offer = self
            .repo
            .get_last_cn_offers_by_cn_process(process_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                    process_id: process_id.clone(),
                    entity: CNControllerTypes::Offer.to_string(),
                },
                _ => CnErrorProvider::DbErr(err),
            })?
            .ok_or(CnErrorProvider::LastNotFound { id: process_id, entity: CNControllerTypes::Offer.to_string() })?;
        Ok(offer)
    }

    async fn get_cn_offer_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_offer::Model> {
        let offer = self
            .repo
            .get_all_cn_offers_by_message_id(message_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::NotFound { id: message_id.clone(), entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?
            .ok_or(CnErrorProvider::NotFound {
                id: message_id.clone(),
                entity: CNControllerTypes::Message.to_string(),
            })?;
        Ok(offer)
    }

    async fn get_cn_offer_by_offer_id(
        &self,
        offer_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_offer::Model> {
        let offer = self
            .repo
            .get_cn_offer_by_id(offer_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound { id: offer_id, entity: CNControllerTypes::Offer.to_string() })?;
        Ok(offer)
    }

    async fn post_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewContractNegotiationOfferRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_offer::Model> {
        let offer =
            self.repo.create_cn_offer(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorProvider::DbErr(err),
                }
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationOffer".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&offer)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        Ok(offer)
    }

    async fn put_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
        input: EditContractNegotiationOfferRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::cn_offer::Model> {
        let offer = self
            .repo
            .put_cn_offer(
                process_id.clone(),
                message_id.clone(),
                offer_id,
                input.into(),
            )
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                }
                CnErrors::CNOfferNotFound => {
                    CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationOffer".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&offer)?,
                message_operation: RainbowEventsNotificationMessageOperation::Update,
            })
            .await?;
        Ok(offer)
    }

    async fn delete_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
    ) -> anyhow::Result<()> {
        let _ = self.repo.delete_cn_offer(process_id.clone(), message_id.clone(), offer_id.clone()).await.map_err(
            |err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                }
                CnErrors::CNOfferNotFound => {
                    CnErrorProvider::NotFound { id: offer_id.clone(), entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            },
        )?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "ContractNegotiationOffer".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "ContractNegotiationOffer",
                    "@id": offer_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_agreement_by_cn_process_id(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::agreement::Model> {
        let agreement = self
            .repo
            .get_agreement_by_process_id(process_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::NotFound { id: process_id.clone(), entity: CNControllerTypes::Process.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?
            .ok_or(CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() })?;
        Ok(agreement)
    }

    async fn get_agreement_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::agreement::Model> {
        let agreement = self
            .repo
            .get_agreement_by_message_id(message_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::NotFound { id: message_id.clone(), entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?
            .ok_or(CnErrorProvider::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() })?;
        Ok(agreement)
    }

    async fn get_agreements(&self) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::agreement::Model>> {
        let agreements = self.repo.get_all_agreements(None, None).await.map_err(CnErrorProvider::DbErr)?;
        Ok(agreements)
    }

    async fn get_agreement_by_agreement_id(
        &self,
        agreement_id: Urn,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::agreement::Model> {
        let agreement =
            self.repo.get_agreement_by_ag_id(agreement_id.clone()).await.map_err(CnErrorProvider::DbErr)?.ok_or(
                CnErrorProvider::NotFound { id: agreement_id, entity: CNControllerTypes::Agreement.to_string() },
            )?;
        Ok(agreement)
    }

    async fn get_agreements_by_participant_id(&self, participant_id: Urn) -> anyhow::Result<Vec<rainbow_db::contracts_provider::entities::agreement::Model>> {
        let agreements = self
            .repo
            .get_agreements_by_participant_id(participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?;
        Ok(agreements)
    }

    async fn post_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewAgreementRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::agreement::Model> {
        let agreement =
            self.repo.create_agreement(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorProvider::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorProvider::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorProvider::DbErr(err),
                }
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "Agreement".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&agreement)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        Ok(agreement)
    }

    async fn put_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
        input: EditAgreementRequest,
    ) -> anyhow::Result<rainbow_db::contracts_provider::entities::agreement::Model> {
        let agreement = self
            .repo
            .put_agreement(
                process_id.clone(),
                message_id.clone(),
                agreement_id.clone(),
                input.into(),
            )
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::AgreementNotFound => {
                    CnErrorProvider::NotFound { id: agreement_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                _ => CnErrorProvider::DbErr(err),
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "Agreement".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&agreement)?,
                message_operation: RainbowEventsNotificationMessageOperation::Update,
            })
            .await?;
        Ok(agreement)
    }

    async fn delete_agreement(&self, process_id: Urn, message_id: Urn, agreement_id: Urn) -> anyhow::Result<()> {
        let _ = self
            .repo
            .delete_agreement(process_id.clone(), message_id.clone(), agreement_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorProvider::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorProvider::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::AgreementNotFound => CnErrorProvider::NotFound {
                    id: agreement_id.clone(),
                    entity: CNControllerTypes::Agreement.to_string(),
                },
                _ => CnErrorProvider::DbErr(err),
            })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory: "Agreement".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "Agreement",
                    "@id": agreement_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }
}
