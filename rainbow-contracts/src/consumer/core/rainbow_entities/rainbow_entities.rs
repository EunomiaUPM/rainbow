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

use crate::common::CNControllerTypes;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use crate::consumer::core::rainbow_entities::rainbow_entities_types::{EditAgreementRequest, EditContractNegotiationMessageRequest, EditContractNegotiationOfferRequest, EditContractNegotiationRequest, NewAgreementRequest, NewContractNegotiationMessageRequest, NewContractNegotiationOfferRequest, NewContractNegotiationRequest};
use crate::consumer::core::rainbow_entities::RainbowEntitiesContractNegotiationConsumerTrait;
use axum::async_trait;
use rainbow_common::protocol::contract::cn_consumer_process::CnConsumerProcess;
use rainbow_db::contracts_consumer::entities::{agreement, cn_message, cn_offer, cn_process};
use rainbow_db::contracts_consumer::repo::{AgreementConsumerRepo, CnErrors, ContractNegotiationConsumerMessageRepo, ContractNegotiationConsumerOfferRepo, ContractNegotiationConsumerProcessRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowEntitiesContractNegotiationConsumerService<T, U>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowEntitiesContractNegotiationConsumerService<T, U>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
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
impl<T, U> RainbowEntitiesContractNegotiationConsumerTrait for RainbowEntitiesContractNegotiationConsumerService<T, U>
where
    T: ContractNegotiationConsumerProcessRepo
    + ContractNegotiationConsumerMessageRepo
    + ContractNegotiationConsumerOfferRepo
    + AgreementConsumerRepo
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_cn_processes(&self) -> anyhow::Result<Vec<CnConsumerProcess>> {
        let processes = self.repo.get_all_cn_processes(None, None).await.map_err(CnErrorConsumer::DbErr)?;
        let processes = processes.iter().map(|p| CnConsumerProcess::from(p.to_owned())).collect();
        Ok(processes)
    }

    async fn get_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<cn_process::Model> {
        let process = self
            .repo
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() })?;
        Ok(process)
    }

    async fn get_cn_process_by_provider(&self, provider_id: Urn) -> anyhow::Result<CnConsumerProcess> {
        let process =
            self.repo.get_cn_process_by_provider_id(provider_id.clone()).await.map_err(CnErrorConsumer::DbErr)?.ok_or(
                CnErrorConsumer::ProviderNotFound { provider_id, entity: CNControllerTypes::Process.to_string() },
            )?;
        let process: CnConsumerProcess = CnConsumerProcess::from(process);
        Ok(process)
    }

    async fn get_cn_process_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<CnConsumerProcess> {
        let process =
            self.repo.get_cn_process_by_consumer_id(consumer_id.clone()).await.map_err(CnErrorConsumer::DbErr)?.ok_or(
                CnErrorConsumer::ConsumerNotFound { consumer_id, entity: CNControllerTypes::Process.to_string() },
            )?;
        let process: CnConsumerProcess = CnConsumerProcess::from(process);
        Ok(process)
    }

    async fn post_cn_process(&self, input: NewContractNegotiationRequest) -> anyhow::Result<cn_process::Model> {
        let process = self.repo.create_cn_process(input.into()).await.map_err(CnErrorConsumer::DbErr)?;
        Ok(process)
    }

    async fn put_cn_process(&self, process_id: Urn, input: EditContractNegotiationRequest) -> anyhow::Result<CnConsumerProcess> {
        let process = self.repo.put_cn_process(process_id.clone(), input.into()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
        })?;
        let process: CnConsumerProcess = CnConsumerProcess::from(process);
        Ok(process)
    }

    async fn delete_cn_process(&self, process_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_cn_process(process_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
        })?;
        Ok(())
    }

    async fn get_cn_messages(
        &self,
    ) -> anyhow::Result<Vec<cn_message::Model>> {
        let cn_messages = self.repo.get_all_cn_messages(None, None).await.map_err(CnErrorConsumer::DbErr)?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_cn_process_id(process_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<cn_message::Model> {
        let cn_message = self
            .repo
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() })?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_cn_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_provider_id(provider_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorConsumer::ProviderNotFound { provider_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_cn_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>> {
        let cn_messages =
            self.repo.get_cn_messages_by_consumer_id(consumer_id.clone()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorConsumer::ConsumerNotFound { consumer_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?;
        Ok(cn_messages)
    }

    async fn post_cn_message_by_cn_process(
        &self,
        process_id: Urn,
        input: NewContractNegotiationMessageRequest,
    ) -> anyhow::Result<cn_message::Model> {
        let cn_message =
            self.repo.create_cn_message(process_id.clone(), input.into()).await.map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
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
    ) -> anyhow::Result<cn_message::Model> {
        let cn_message =
            self.repo.put_cn_message(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorConsumer::DbErr(err),
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
                CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            CnErrors::CNMessageNotFound => {
                CnErrorConsumer::NotFound { id: message_id.clone(), entity: CNControllerTypes::Message.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
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
    ) -> anyhow::Result<Vec<cn_offer::Model>> {
        let offers = self.repo.get_all_cn_offers_by_cn_process(process_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Offer.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
        })?;
        Ok(offers)
    }

    async fn get_last_cn_offers_by_cn_process_id(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<cn_offer::Model> {
        let offer = self
            .repo
            .get_last_cn_offers_by_cn_process(process_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => CnErrorConsumer::ProcessNotFound {
                    process_id: process_id.clone(),
                    entity: CNControllerTypes::Offer.to_string(),
                },
                _ => CnErrorConsumer::DbErr(err),
            })?
            .ok_or(CnErrorConsumer::LastNotFound { id: process_id, entity: CNControllerTypes::Offer.to_string() })?;
        Ok(offer)
    }

    async fn get_cn_offer_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<cn_offer::Model> {
        let offer = self
            .repo
            .get_all_cn_offers_by_message_id(message_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::NotFound { id: message_id.clone(), entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?
            .ok_or(CnErrorConsumer::NotFound {
                id: message_id.clone(),
                entity: CNControllerTypes::Message.to_string(),
            })?;
        Ok(offer)
    }

    async fn get_cn_offer_by_offer_id(
        &self,
        offer_id: Urn,
    ) -> anyhow::Result<cn_offer::Model> {
        let offer = self
            .repo
            .get_cn_offer_by_id(offer_id.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: offer_id, entity: CNControllerTypes::Offer.to_string() })?;
        Ok(offer)
    }

    async fn post_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewContractNegotiationOfferRequest,
    ) -> anyhow::Result<cn_offer::Model> {
        let offer =
            self.repo.create_cn_offer(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorConsumer::DbErr(err),
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
    ) -> anyhow::Result<cn_offer::Model> {
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
                    CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                }
                CnErrors::CNOfferNotFound => {
                    CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
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
                    CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                }
                CnErrors::CNOfferNotFound => {
                    CnErrorConsumer::NotFound { id: offer_id.clone(), entity: CNControllerTypes::Offer.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
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
    ) -> anyhow::Result<agreement::Model> {
        let agreement = self
            .repo
            .get_agreement_by_process_id(process_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNProcessNotFound => {
                    CnErrorConsumer::NotFound { id: process_id.clone(), entity: CNControllerTypes::Process.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?
            .ok_or(CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() })?;
        Ok(agreement)
    }

    async fn get_agreement_by_cn_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<agreement::Model> {
        let agreement = self
            .repo
            .get_agreement_by_message_id(message_id.clone())
            .await
            .map_err(|err| match err {
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::NotFound { id: message_id.clone(), entity: CNControllerTypes::Message.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
            })?
            .ok_or(CnErrorConsumer::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() })?;
        Ok(agreement)
    }

    async fn get_agreements(&self) -> anyhow::Result<Vec<agreement::Model>> {
        let agreements = self.repo.get_all_agreements(None, None).await.map_err(CnErrorConsumer::DbErr)?;
        Ok(agreements)
    }

    async fn get_agreement_by_agreement_id(
        &self,
        agreement_id: Urn,
    ) -> anyhow::Result<agreement::Model> {
        let agreement =
            self.repo.get_agreement_by_ag_id(agreement_id.clone()).await.map_err(CnErrorConsumer::DbErr)?.ok_or(
                CnErrorConsumer::NotFound { id: agreement_id, entity: CNControllerTypes::Agreement.to_string() },
            )?;
        Ok(agreement)
    }

    async fn post_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewAgreementRequest,
    ) -> anyhow::Result<agreement::Model> {
        let agreement =
            self.repo.create_agreement(process_id.clone(), message_id.clone(), input.into()).await.map_err(|err| {
                match err {
                    CnErrors::CNProcessNotFound => {
                        CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
                    }
                    CnErrors::CNMessageNotFound => {
                        CnErrorConsumer::NotFound { id: message_id, entity: CNControllerTypes::Message.to_string() }
                    }
                    _ => CnErrorConsumer::DbErr(err),
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
    ) -> anyhow::Result<agreement::Model> {
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
                    CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::AgreementNotFound => {
                    CnErrorConsumer::NotFound { id: agreement_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                _ => CnErrorConsumer::DbErr(err),
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
                    CnErrorConsumer::ProcessNotFound { process_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::CNMessageNotFound => {
                    CnErrorConsumer::MessageNotFound { message_id, entity: CNControllerTypes::Agreement.to_string() }
                }
                CnErrors::AgreementNotFound => CnErrorConsumer::NotFound {
                    id: agreement_id.clone(),
                    entity: CNControllerTypes::Agreement.to_string(),
                },
                _ => CnErrorConsumer::DbErr(err),
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
