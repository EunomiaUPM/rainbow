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

use super::entities::agreement;
use super::entities::cn_message;
use super::entities::cn_offer;
use super::entities::cn_process;
use super::entities::participant;
use anyhow::Error;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::protocol::contract::ContractNegotiationState;
use sea_orm::DatabaseConnection;
use sea_orm_migration::async_trait::async_trait;
use thiserror::Error;
use urn::Urn;

pub mod sql;

pub trait ContractNegotiationProviderRepoFactory:
ContractNegotiationProcessRepo
+ ContractNegotiationMessageRepo
+ ContractNegotiationOfferRepo
+ AgreementRepo
+ Participant
+ Send
+ Sync
+ 'static
{
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewContractNegotiationProcess {
    pub provider_id: Option<Urn>,
    pub consumer_id: Option<Urn>,
    pub state: ContractNegotiationState,
    pub initiated_by: ConfigRoles,
}
pub struct EditContractNegotiationProcess {
    pub provider_id: Option<Urn>,
    pub consumer_id: Option<Urn>,
    pub state: Option<ContractNegotiationState>,
}

#[async_trait]
pub trait ContractNegotiationProcessRepo {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors>;
    async fn get_cn_processes_by_provider_id(
        &self,
        provider_id: &Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn get_cn_processes_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn get_cn_process_by_cn_id(&self, cn_process_id: Urn) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors>;
    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors>;
    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors>;
}

pub struct NewContractNegotiationMessage {
    pub _type: String,
    pub from: String,
    pub to: String,
    pub content: serde_json::Value,
}
pub struct EditContractNegotiationMessage {
    pub _type: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub content: Option<serde_json::Value>,
}

#[async_trait]
pub trait ContractNegotiationMessageRepo {
    async fn get_all_cn_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors>;
    async fn get_cn_messages_by_cn_process_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors>;
    async fn get_cn_messages_by_cn_message_id(
        &self,
        cn_message_id: Urn,
    ) -> anyhow::Result<Option<cn_message::Model>, CnErrors>;
    async fn get_cn_messages_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors>;
    async fn get_cn_messages_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors>;
    async fn put_cn_message(
        &self,
        cn_process_id: Urn,
        cn_message_id: Urn,
        edit_cn_message: EditContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors>;
    async fn create_cn_message(
        &self,
        cn_process_id: Urn,
        new_cn_message: NewContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors>;
    async fn delete_cn_message(&self, cn_process_id: Urn, cn_message_id: Urn) -> anyhow::Result<(), CnErrors>;
}

pub struct NewContractNegotiationOffer {
    pub offer_id: Option<Urn>,
    pub offer_content: serde_json::Value,
}
pub struct EditContractNegotiationOffer {}

#[async_trait]
pub trait ContractNegotiationOfferRepo {
    async fn get_all_cn_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors>;
    async fn get_all_cn_offers_by_message_id(&self, offer_id: Urn)
                                             -> anyhow::Result<Option<cn_offer::Model>, CnErrors>;
    async fn get_all_cn_offers_by_provider(&self, provider_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors>;
    async fn get_all_cn_offers_by_cn_process(&self, process_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors>;
    async fn get_all_cn_offers_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors>;
    async fn get_last_cn_offers_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors>;
    async fn get_cn_offer_by_id(&self, offer_id: Urn) -> anyhow::Result<Option<cn_offer::Model>, CnErrors>;
    async fn put_cn_offer(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
        edit_cn_offer: EditContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors>;
    async fn create_cn_offer(
        &self,
        process_id: Urn,
        message_id: Urn,
        new_cn_offer: NewContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors>;
    async fn delete_cn_offer(&self, process_id: Urn, message_id: Urn, offer_id: Urn) -> anyhow::Result<(), CnErrors>;
}

pub struct NewAgreement {
    pub consumer_participant_id: Urn,
    pub provider_participant_id: Urn,
    pub agreement_content: OdrlAgreement,
    pub active: bool,
}
pub struct EditAgreement {
    pub active: Option<bool>,
}
#[async_trait]
pub trait AgreementRepo {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors>;
    async fn get_agreement_by_ag_id(&self, agreement_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors>;
    async fn get_agreement_by_process_id(&self, process_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors>;
    async fn get_agreement_by_message_id(&self, message_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors>;

    async fn get_agreements_by_participant_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors>;

    async fn put_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
        edit_agreement: EditAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors>;
    async fn create_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        new_agreement: NewAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors>;
    async fn delete_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
    ) -> anyhow::Result<(), CnErrors>;
}

pub struct NewParticipant {
    pub participant_id: Option<Urn>,
    pub identity_token: Option<String>,
    pub _type: String,
    pub base_url: String,
    pub extra_fields: serde_json::Value,
}
pub struct EditParticipant {
    pub identity_token: Option<String>,
    pub base_url: Option<String>,
    pub extra_fields: Option<serde_json::Value>,
}

#[async_trait]
pub trait Participant {
    async fn get_all_participants(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<participant::Model>, CnErrors>;
    async fn get_participant_by_p_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Option<participant::Model>, CnErrors>;
    async fn get_provider_participant(
        &self,
    ) -> anyhow::Result<Option<participant::Model>, CnErrors>;
    async fn put_participant(
        &self,
        participant_id: Urn,
        edit_participant: EditParticipant,
    ) -> anyhow::Result<participant::Model, CnErrors>;
    async fn create_participant(&self, new_participant: NewParticipant)
                                -> anyhow::Result<participant::Model, CnErrors>;
    async fn delete_participant(&self, participant_id: Urn) -> anyhow::Result<(), CnErrors>;
}

#[derive(Error, Debug)]
pub enum CnErrors {
    #[error("Contract Negotiation Process not found")]
    CNProcessNotFound,
    #[error("Contract Negotiation Message not found")]
    CNMessageNotFound,
    #[error("Contract Negotiation Offer not found")]
    CNOfferNotFound,
    #[error("Agreement not found")]
    AgreementNotFound,
    #[error("Participant not found")]
    ParticipantNotFound(String, Urn),

    #[error("Error fetching Contract Negotiation Process. {0}")]
    ErrorFetchingCNProcess(Error),
    #[error("Error fetching Contract Negotiation Message. {0}")]
    ErrorFetchingCNMessage(Error),
    #[error("Error fetching Contract Negotiation Offer. {0}")]
    ErrorFetchingCNOffer(Error),
    #[error("Error fetching Agreement. {0}")]
    ErrorFetchingAgreement(Error),
    #[error("Error fetching Participant. {0}")]
    ErrorFetchingParticipant(Error),

    #[error("Error creating Contract Negotiation Process. {0}")]
    ErrorCreatingCNProcess(Error),
    #[error("Error creating Contract Negotiation Message. {0}")]
    ErrorCreatingCNMessage(Error),
    #[error("Error creating Contract Negotiation Offer. {0}")]
    ErrorCreatingCNOffer(Error),
    #[error("Error creating Agreement. {0}")]
    ErrorCreatingAgreement(Error),
    #[error("Error creating Participant. {0}")]
    ErrorCreatingParticipant(Error),

    #[error("Error deleting Contract Negotiation Process. {0}")]
    ErrorDeletingCNProcess(Error),
    #[error("Error deleting Contract Negotiation Message. {0}")]
    ErrorDeletingCNMessage(Error),
    #[error("Error deleting Contract Negotiation Offer. {0}")]
    ErrorDeletingCNOffer(Error),
    #[error("Error deleting Agreement. {0}")]
    ErrorDeletingAgreement(Error),
    #[error("Error deleting Participant. {0}")]
    ErrorDeletingParticipant(Error),

    #[error("Error updating Contract Negotiation Process. {0}")]
    ErrorUpdatingCNProcess(Error),
    #[error("Error updating Contract Negotiation Message. {0}")]
    ErrorUpdatingCNMessage(Error),
    #[error("Error updating Contract Negotiation Offer. {0}")]
    ErrorUpdatingCNOffer(Error),
    #[error("Error updating Agreement. {0}")]
    ErrorUpdatingAgreement(Error),
    #[error("Error updating Participant. {0}")]
    ErrorUpdatingParticipant(Error),
}
