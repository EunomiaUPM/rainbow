use crate::provider::core::rainbow_entities::rainbow_entities_types::{
    EditAgreementRequest, EditContractNegotiationMessageRequest, EditContractNegotiationOfferRequest,
    EditContractNegotiationRequest, EditParticipantRequest, NewAgreementRequest, NewContractNegotiationMessageRequest,
    NewContractNegotiationOfferRequest, NewContractNegotiationRequest, NewParticipantRequest,
};
use axum::async_trait;
use rainbow_db::contracts_provider::entities::{agreement, cn_message, cn_offer, cn_process, participant};
use urn::Urn;

pub mod rainbow_entities;
pub mod rainbow_entities_errors;
pub mod rainbow_entities_types;


#[mockall::automock]
#[async_trait]
pub trait RainbowEntitiesContractNegotiationProviderTrait: Send + Sync {
    async fn get_cn_processes(&self) -> anyhow::Result<Vec<cn_process::Model>>;
    async fn get_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn get_cn_process_by_provider(&self, provider_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn get_cn_process_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn post_cn_process(&self, input: NewContractNegotiationRequest) -> anyhow::Result<cn_process::Model>;
    async fn put_cn_process(
        &self,
        process_id: Urn,
        input: EditContractNegotiationRequest,
    ) -> anyhow::Result<cn_process::Model>;
    async fn delete_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<()>;
    async fn get_cn_messages(&self) -> anyhow::Result<Vec<cn_message::Model>>;
    async fn get_cn_messages_by_cn_process(&self, process_id: Urn) -> anyhow::Result<Vec<cn_message::Model>>;
    async fn get_cn_messages_by_cn_message_id(&self, message_id: Urn) -> anyhow::Result<cn_message::Model>;
    async fn get_cn_messages_by_cn_provider_id(&self, provider_id: Urn) -> anyhow::Result<Vec<cn_message::Model>>;
    async fn get_cn_messages_by_cn_consumer_id(&self, consumer_id: Urn) -> anyhow::Result<Vec<cn_message::Model>>;
    async fn post_cn_message_by_cn_process(
        &self,
        process_id: Urn,
        input: NewContractNegotiationMessageRequest,
    ) -> anyhow::Result<cn_message::Model>;
    async fn put_cn_message_by_cn_process(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: EditContractNegotiationMessageRequest,
    ) -> anyhow::Result<cn_message::Model>;
    async fn delete_cn_message_by_cn_process(&self, process_id: Urn, message_id: Urn) -> anyhow::Result<()>;
    async fn get_cn_offers_by_cn_process_id(&self, process_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>>;
    async fn get_last_cn_offers_by_cn_process_id(&self, process_id: Urn) -> anyhow::Result<cn_offer::Model>;
    async fn get_cn_offer_by_cn_message_id(&self, message_id: Urn) -> anyhow::Result<cn_offer::Model>;
    async fn get_cn_offer_by_offer_id(&self, offer_id: Urn) -> anyhow::Result<cn_offer::Model>;
    async fn post_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewContractNegotiationOfferRequest,
    ) -> anyhow::Result<cn_offer::Model>;
    async fn put_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
        input: EditContractNegotiationOfferRequest,
    ) -> anyhow::Result<cn_offer::Model>;
    async fn delete_cn_offer_by_cn_process_id_and_message_id(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
    ) -> anyhow::Result<()>;
    async fn get_agreement_by_cn_process_id(&self, process_id: Urn) -> anyhow::Result<agreement::Model>;
    async fn get_agreement_by_cn_message_id(&self, message_id: Urn) -> anyhow::Result<agreement::Model>;
    async fn get_agreements(&self) -> anyhow::Result<Vec<agreement::Model>>;
    async fn get_agreement_by_agreement_id(&self, agreement_id: Urn) -> anyhow::Result<agreement::Model>;
    async fn post_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        input: NewAgreementRequest,
    ) -> anyhow::Result<agreement::Model>;
    async fn put_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
        input: EditAgreementRequest,
    ) -> anyhow::Result<agreement::Model>;
    async fn delete_agreement(&self, process_id: Urn, message_id: Urn, agreement_id: Urn) -> anyhow::Result<()>;
    async fn get_participants(&self) -> anyhow::Result<Vec<participant::Model>>;
    async fn get_participant_by_id(&self, participant_id: Urn) -> anyhow::Result<participant::Model>;
    async fn get_participant_agreements(&self, participant_id: Urn) -> anyhow::Result<Vec<agreement::Model>>;
    async fn post_participant(&self, input: NewParticipantRequest) -> anyhow::Result<participant::Model>;
    async fn put_participant(
        &self,
        participant_id: Urn,
        input: EditParticipantRequest,
    ) -> anyhow::Result<participant::Model>;
    async fn delete_participant(&self, participant_id: Urn) -> anyhow::Result<()>;
}
