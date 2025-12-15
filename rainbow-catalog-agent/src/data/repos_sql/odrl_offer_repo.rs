use crate::data::entities::odrl_offer;
use crate::data::entities::odrl_offer::NewOdrlOfferModel;
use crate::data::repo_traits::odrl_offer_repo::{OdrlOfferRepoErrors, OdrlOfferRepositoryTrait};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect,
};
use urn::Urn;

pub struct OdrlOfferRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl OdrlOfferRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl OdrlOfferRepositoryTrait for OdrlOfferRepositoryForSql {
    async fn get_all_odrl_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, OdrlOfferRepoErrors> {
        let odrl_offers = odrl_offer::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn get_batch_odrl_offers(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, OdrlOfferRepoErrors> {
        let odrl_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let odrl_process =
            odrl_offer::Entity::find().filter(odrl_offer::Column::Id.is_in(odrl_ids)).all(&self.db_connection).await;
        match odrl_process {
            Ok(odrl_process) => Ok(odrl_process),
            Err(e) => Err(OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(e.into())),
        }
    }

    async fn get_all_odrl_offers_by_entity(
        &self,
        entity: &Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, OdrlOfferRepoErrors> {
        let entity = entity.to_string();
        let odrl_offers =
            odrl_offer::Entity::find().filter(odrl_offer::Column::Entity.eq(entity)).all(&self.db_connection).await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: &Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, OdrlOfferRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::find_by_id(odrl_offer_id).one(&self.db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn create_odrl_offer(
        &self,
        new_odrl_offer_model: &NewOdrlOfferModel,
    ) -> anyhow::Result<odrl_offer::Model, OdrlOfferRepoErrors> {
        let model: odrl_offer::ActiveModel = new_odrl_offer_model.into();
        let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(OdrlOfferRepoErrors::ErrorCreatingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<(), OdrlOfferRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_by_id(odrl_offer_id).exec(&self.db_connection).await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(OdrlOfferRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(OdrlOfferRepoErrors::ErrorDeletingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offers_by_entity(&self, entity_id: &Urn) -> anyhow::Result<(), OdrlOfferRepoErrors> {
        let entity_id = entity_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_many()
            .filter(odrl_offer::Column::Entity.eq(entity_id))
            .exec(&self.db_connection)
            .await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(OdrlOfferRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(OdrlOfferRepoErrors::ErrorUpdatingOdrlOffer(err.into())),
        }
    }

    async fn get_upstream_offers(
        &self,
        entity_id: &Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, OdrlOfferRepoErrors> {
        todo!()
    }
}
