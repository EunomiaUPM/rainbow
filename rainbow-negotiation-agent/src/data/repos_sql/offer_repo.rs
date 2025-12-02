use crate::data::entities::offer::{Model, NewOfferModel};
use crate::data::repo_traits::offer_repo::{OfferRepoErrors, OfferRepoTrait};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub struct OfferRepoForSql {
    db_connection: DatabaseConnection,
}

impl OfferRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl OfferRepoTrait for OfferRepoForSql {
    async fn get_all_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn get_batch_offers(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn get_offers_by_negotiation_process(&self, id: &Urn) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn get_offer_by_id(&self, id: &Urn) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn get_offer_by_negotiation_message(&self, id: &Urn) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn get_offer_by_offer_id(&self, id: &Urn) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        todo!()
    }

    async fn create_offer(&self, new_model: &NewOfferModel) -> anyhow::Result<Model, OfferRepoErrors> {
        todo!()
    }

    async fn delete_offer(&self, id: &Urn) -> anyhow::Result<(), OfferRepoErrors> {
        todo!()
    }
}
