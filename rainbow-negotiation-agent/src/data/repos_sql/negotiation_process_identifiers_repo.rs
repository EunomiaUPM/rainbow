use crate::data::entities::negotiation_process_identifier::{
    EditNegotiationIdentifierModel, Model, NewNegotiationIdentifierModel,
};
use crate::data::repo_traits::negotiation_process_identifiers_repo::{
    NegotiationIdentifierRepoErrors, NegotiationIdentifierRepoTrait,
};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub struct NegotiationProcessIdentifierRepoForSql {
    db_connection: DatabaseConnection,
}

impl NegotiationProcessIdentifierRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl NegotiationIdentifierRepoTrait for NegotiationProcessIdentifierRepoForSql {
    async fn get_all_identifiers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn get_identifiers_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<Model>, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn get_identifier_by_id(&self, id: &Urn) -> anyhow::Result<Option<Model>, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn get_identifier_by_key(
        &self,
        process_id: &Urn,
        key: &str,
    ) -> anyhow::Result<Option<Model>, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn create_identifier(
        &self,
        new_model: &NewNegotiationIdentifierModel,
    ) -> anyhow::Result<Model, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn put_identifier(
        &self,
        id: &Urn,
        edit_model: &EditNegotiationIdentifierModel,
    ) -> anyhow::Result<Model, NegotiationIdentifierRepoErrors> {
        todo!()
    }

    async fn delete_identifier(&self, id: &Urn) -> anyhow::Result<(), NegotiationIdentifierRepoErrors> {
        todo!()
    }
}
