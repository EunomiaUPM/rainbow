use crate::data::entities::odrl_offer::NewOdrlOfferModel;
use crate::data::entities::{catalog, dataservice, dataset, distribution, odrl_offer};
use crate::data::repo_traits::catalog_db_errors::{
    CatalogAgentRepoErrors, CatalogRepoErrors, DataServiceRepoErrors, DatasetRepoErrors, DistributionRepoErrors,
    OdrlOfferRepoErrors,
};
use crate::data::repo_traits::odrl_offer_repo::OdrlOfferRepositoryTrait;
use crate::entities::odrl_policies::CatalogEntityTypes;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct OdrlOfferRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl OdrlOfferRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl OdrlOfferRepositoryTrait for OdrlOfferRepositoryForSql {
    async fn get_all_odrl_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogAgentRepoErrors> {
        let page_limit = limit.unwrap_or(25);
        let page_number = page.unwrap_or(1);
        let calculated_offset = (page_number.max(1) - 1) * page_limit;
        let odrl_offers =
            odrl_offer::Entity::find().limit(page_limit).offset(calculated_offset).all(&self.db_connection).await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into()),
            )),
        }
    }

    async fn get_batch_odrl_offers(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogAgentRepoErrors> {
        let odrl_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let odrl_process =
            odrl_offer::Entity::find().filter(odrl_offer::Column::Id.is_in(odrl_ids)).all(&self.db_connection).await;
        match odrl_process {
            Ok(odrl_process) => Ok(odrl_process),
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into()),
            )),
        }
    }

    async fn get_all_odrl_offers_by_entity(
        &self,
        entity: &Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogAgentRepoErrors> {
        let entity = entity.to_string();
        let odrl_offers =
            odrl_offer::Entity::find().filter(odrl_offer::Column::Entity.eq(entity)).all(&self.db_connection).await;
        match odrl_offers {
            Ok(odrl_offers) => match odrl_offers.is_empty() {
                false => Ok(odrl_offers),
                true => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                    OdrlOfferRepoErrors::OdrlOfferNotFound,
                )),
            },
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into()),
            )),
        }
    }

    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: &Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, CatalogAgentRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::find_by_id(odrl_offer_id).one(&self.db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorFetchingOdrlOffer(err.into()),
            )),
        }
    }

    async fn create_odrl_offer(
        &self,
        new_odrl_offer_model: &NewOdrlOfferModel,
    ) -> anyhow::Result<odrl_offer::Model, CatalogAgentRepoErrors> {
        let model: odrl_offer::ActiveModel = new_odrl_offer_model.into();
        let entity_id = new_odrl_offer_model.entity_id.to_string();
        let odrl_offer = match new_odrl_offer_model.entity_type {
            CatalogEntityTypes::Distribution => {
                let _ = distribution::Entity::find_by_id(entity_id)
                    .one(&self.db_connection)
                    .await
                    .map_err(|err| {
                        CatalogAgentRepoErrors::DistributionRepoErrors(
                            DistributionRepoErrors::ErrorFetchingDistribution(err.into()),
                        )
                    })?
                    .ok_or(CatalogAgentRepoErrors::DistributionRepoErrors(
                        DistributionRepoErrors::DistributionNotFound,
                    ))?;
                let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
                odrl_offer
            }
            CatalogEntityTypes::DataService => {
                let _ = dataservice::Entity::find_by_id(entity_id)
                    .one(&self.db_connection)
                    .await
                    .map_err(|err| {
                        CatalogAgentRepoErrors::DataServiceRepoErrors(DataServiceRepoErrors::ErrorFetchingDataService(
                            err.into(),
                        ))
                    })?
                    .ok_or(CatalogAgentRepoErrors::DataServiceRepoErrors(
                        DataServiceRepoErrors::DataServiceNotFound,
                    ))?;
                let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
                odrl_offer
            }
            CatalogEntityTypes::Catalog => {
                let _ = catalog::Entity::find_by_id(entity_id)
                    .one(&self.db_connection)
                    .await
                    .map_err(|err| {
                        CatalogAgentRepoErrors::CatalogRepoErrors(CatalogRepoErrors::ErrorFetchingCatalog(err.into()))
                    })?
                    .ok_or(CatalogAgentRepoErrors::CatalogRepoErrors(
                        CatalogRepoErrors::CatalogNotFound,
                    ))?;
                let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
                odrl_offer
            }
            CatalogEntityTypes::Dataset => {
                let _ = dataset::Entity::find_by_id(entity_id)
                    .one(&self.db_connection)
                    .await
                    .map_err(|err| {
                        CatalogAgentRepoErrors::DatasetRepoErrors(DatasetRepoErrors::ErrorFetchingDataset(err.into()))
                    })?
                    .ok_or(CatalogAgentRepoErrors::DatasetRepoErrors(
                        DatasetRepoErrors::DatasetNotFound,
                    ))?;
                let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
                odrl_offer
            }
        };

        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorCreatingOdrlOffer(err.into()),
            )),
        }
    }

    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<(), CatalogAgentRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_by_id(odrl_offer_id).exec(&self.db_connection).await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                    OdrlOfferRepoErrors::OdrlOfferNotFound,
                )),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorDeletingOdrlOffer(err.into()),
            )),
        }
    }

    async fn delete_odrl_offers_by_entity(&self, entity_id: &Urn) -> anyhow::Result<(), CatalogAgentRepoErrors> {
        let entity_id = entity_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_many()
            .filter(odrl_offer::Column::Entity.eq(entity_id))
            .exec(&self.db_connection)
            .await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                    OdrlOfferRepoErrors::OdrlOfferNotFound,
                )),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogAgentRepoErrors::OdrlOfferRepoErrors(
                OdrlOfferRepoErrors::ErrorDeletingOdrlOffer(err.into()),
            )),
        }
    }
}
