use crate::data::entities::negotiation_message::{Model, NewNegotiationMessageModel};
use crate::data::repo_traits::negotiation_message_repo::{NegotiationMessageRepoErrors, NegotiationMessageRepoTrait};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub struct NegotiationMessageRepoForSql {
    db_connection: DatabaseConnection,
}

impl NegotiationMessageRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl NegotiationMessageRepoTrait for NegotiationMessageRepoForSql {
    async fn get_all_negotiation_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, NegotiationMessageRepoErrors> {
        todo!()
    }

    async fn get_messages_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<Model>, NegotiationMessageRepoErrors> {
        todo!()
    }

    async fn get_negotiation_message_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, NegotiationMessageRepoErrors> {
        todo!()
    }

    async fn create_negotiation_message(
        &self,
        new_model: &NewNegotiationMessageModel,
    ) -> anyhow::Result<Model, NegotiationMessageRepoErrors> {
        todo!()
    }

    async fn delete_negotiation_message(&self, id: &Urn) -> anyhow::Result<(), NegotiationMessageRepoErrors> {
        todo!()
    }
}
