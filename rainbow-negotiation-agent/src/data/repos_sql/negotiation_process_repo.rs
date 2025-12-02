use crate::data::entities::negotiation_process::{EditNegotiationProcessModel, Model, NewNegotiationProcessModel};
use crate::data::repo_traits::negotiation_process_repo::{NegotiationProcessRepoErrors, NegotiationProcessRepoTrait};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub struct NegotiationProcessRepoForSql {
    db_connection: DatabaseConnection,
}

impl NegotiationProcessRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl NegotiationProcessRepoTrait for NegotiationProcessRepoForSql {
    async fn get_all_negotiation_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn get_batch_negotiation_processes(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<Model>, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn get_negotiation_process_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn get_negotiation_process_by_key_id(
        &self,
        key_id: &str,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn get_negotiation_process_by_key_value(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn create_negotiation_process(
        &self,
        new_model: &NewNegotiationProcessModel,
    ) -> anyhow::Result<Model, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn put_negotiation_process(
        &self,
        id: &Urn,
        edit_model: &EditNegotiationProcessModel,
    ) -> anyhow::Result<Model, NegotiationProcessRepoErrors> {
        todo!()
    }

    async fn delete_negotiation_process(&self, id: &Urn) -> anyhow::Result<(), NegotiationProcessRepoErrors> {
        todo!()
    }
}
