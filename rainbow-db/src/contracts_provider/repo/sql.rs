use super::super::entities::agreement;
use super::super::entities::cn_message;
use super::super::entities::cn_offer;
use super::super::entities::cn_process;
use super::super::entities::participant;
use crate::contracts_provider::repo::{
    AgreementRepo, CnErrors, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo,
    ContractNegotiationProcessRepo, EditAgreement, EditContractNegotiationMessage,
    EditContractNegotiationOffer, EditContractNegotiationProcess, EditParticipant, NewAgreement,
    NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess,
    NewParticipant, Participant,
};
use sea_orm_migration::async_trait::async_trait;
use urn::Urn;

pub struct ContractNegotiationRepoForSql {}

#[async_trait]
impl ContractNegotiationProcessRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_processes_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_processes_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        todo!()
    }

    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        todo!()
    }

    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}

#[async_trait]
impl ContractNegotiationMessageRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_messages_by_cn_id(
        &self,
        cn_message_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_messages_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_messages_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        todo!()
    }

    async fn put_cn_message(
        &self,
        cn_message_id: Urn,
        edit_cn_message: EditContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors> {
        todo!()
    }

    async fn create_cn_message(
        &self,
        new_cn_message: NewContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors> {
        todo!()
    }

    async fn delete_cn_message(&self, cn_message_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}

#[async_trait]
impl ContractNegotiationOfferRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        todo!()
    }

    async fn get_all_cn_offers_by_id(
        &self,
        offer_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        todo!()
    }

    async fn get_all_cn_offers_by_provider(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        todo!()
    }

    async fn get_all_cn_offers_by_consumer(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_offers_by_id(
        &self,
        offer_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        todo!()
    }

    async fn put_cn_offer(
        &self,
        offer_id: Urn,
        edit_cn_offer: EditContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors> {
        todo!()
    }

    async fn create_cn_offer(
        &self,
        new_cn_offer: NewContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors> {
        todo!()
    }

    async fn delete_cn_offer(&self, offer_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}

#[async_trait]
impl AgreementRepo for ContractNegotiationRepoForSql {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors> {
        todo!()
    }

    async fn get_agreement_by_ag_id(
        &self,
        agreement_id: Urn,
    ) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        todo!()
    }

    async fn put_agreement(
        &self,
        agreement_id: Urn,
        edit_agreement: EditAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors> {
        todo!()
    }

    async fn create_agreement(
        &self,
        new_agreement: NewAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors> {
        todo!()
    }

    async fn delete_agreement(&self, agreement_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}

#[async_trait]
impl Participant for ContractNegotiationRepoForSql {
    async fn get_all_participants(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<participant::Model>, CnErrors> {
        todo!()
    }

    async fn get_participant_by_p_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Option<participant::Model>, CnErrors> {
        todo!()
    }

    async fn put_participant(
        &self,
        participant_id: Urn,
        edit_participant: EditParticipant,
    ) -> anyhow::Result<participant::Model, CnErrors> {
        todo!()
    }

    async fn create_participant(
        &self,
        new_participant: NewParticipant,
    ) -> anyhow::Result<participant::Model, CnErrors> {
        todo!()
    }

    async fn delete_participant(&self, participant_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}
