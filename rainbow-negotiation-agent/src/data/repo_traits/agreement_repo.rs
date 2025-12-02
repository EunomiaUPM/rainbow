use crate::data::entities::agreement;
use crate::data::entities::agreement::{EditAgreementModel, NewAgreementModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait AgreementRepoTrait: Send + Sync {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreement::Model>, AgreementRepoErrors>;
    async fn get_batch_agreements(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<agreement::Model>, AgreementRepoErrors>;
    async fn get_agreement_by_id(&self, id: &Urn) -> anyhow::Result<Option<agreement::Model>, AgreementRepoErrors>;
    async fn get_agreement_by_negotiation_process(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<agreement::Model>, AgreementRepoErrors>;
    async fn get_agreement_by_negotiation_message(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<agreement::Model>, AgreementRepoErrors>;
    async fn create_agreement(
        &self,
        new_model: &NewAgreementModel,
    ) -> anyhow::Result<agreement::Model, AgreementRepoErrors>;
    async fn put_agreement(
        &self,
        id: &Urn,
        edit_model: &EditAgreementModel,
    ) -> anyhow::Result<agreement::Model, AgreementRepoErrors>;
    async fn delete_agreement(&self, id: &Urn) -> anyhow::Result<(), AgreementRepoErrors>;
}

#[derive(Debug, Error)]
pub enum AgreementRepoErrors {
    #[error("Agreement not found")]
    AgreementNotFound,
    #[error("Error fetching agreement. {0}")]
    ErrorFetchingAgreement(Error),
    #[error("Error creating agreement. {0}")]
    ErrorCreatingAgreement(Error),
    #[error("Error deleting agreement. {0}")]
    ErrorDeletingAgreement(Error),
}
