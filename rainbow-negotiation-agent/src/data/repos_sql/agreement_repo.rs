use crate::data::entities::agreement::{EditAgreementModel, Model, NewAgreementModel};
use crate::data::repo_traits::agreement_repo::{AgreementRepoErrors, AgreementRepoTrait};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub struct AgreementRepoForSql {
    db_connection: DatabaseConnection,
}

impl AgreementRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl AgreementRepoTrait for AgreementRepoForSql {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, AgreementRepoErrors> {
        todo!()
    }

    async fn get_batch_agreements(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>, AgreementRepoErrors> {
        todo!()
    }

    async fn get_agreement_by_id(&self, id: &Urn) -> anyhow::Result<Option<Model>, AgreementRepoErrors> {
        todo!()
    }

    async fn get_agreement_by_negotiation_process(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, AgreementRepoErrors> {
        todo!()
    }

    async fn get_agreement_by_negotiation_message(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, AgreementRepoErrors> {
        todo!()
    }

    async fn create_agreement(&self, new_model: &NewAgreementModel) -> anyhow::Result<Model, AgreementRepoErrors> {
        todo!()
    }

    async fn put_agreement(
        &self,
        id: &Urn,
        edit_model: &EditAgreementModel,
    ) -> anyhow::Result<Model, AgreementRepoErrors> {
        todo!()
    }

    async fn delete_agreement(&self, id: &Urn) -> anyhow::Result<(), AgreementRepoErrors> {
        todo!()
    }
}
