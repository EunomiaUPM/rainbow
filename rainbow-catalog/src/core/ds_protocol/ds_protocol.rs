use crate::core::ds_protocol::DSProtocolCatalogTrait;
// use crate::core::idsa_api::distributions_request_by_dataset;
use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use axum::async_trait;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;
use urn::Urn;

pub struct DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> DSProtocolCatalogTrait for DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn dataset_request(&self, dataset_id: Urn) -> anyhow::Result<Dataset> {
        todo!()
        // let db_connection = get_db_connection().await;
        // let datasets_out: Vec<Dataset> = vec![];
        // // let datasets_from_db = dataset::Entity::find()
        // //     .filter(dataset::Column::Id.eq(dataset_id.to_string()))
        // //     .one(db_connection)
        // //     .await?;
        //
        // // <=============== HERE end here all
        // let datasets_from_db = self.repo.get_all_datasets(None, None).await?;
        //
        // match datasets_from_db {
        //     Some(dataset_from_db) => {
        //         let mut dataset = Dataset::try_from(dataset_from_db.clone()).unwrap();
        //         // odrl
        //         let dataset_odrl_from_db = odrl_offer::Entity::find()
        //             .filter(odrl_offer::Column::Entity.eq(dataset_from_db.id))
        //             .all(db_connection)
        //             .await?;
        //         dataset.odrl_offer = to_value(dataset_odrl_from_db)?;
        //         dataset.distribution = distributions_request_by_dataset(
        //             dataset.id.parse()?,
        //             dataset_from_db.catalog_id.parse()?,
        //         )
        //             .await?;
        //         Ok(dataset)
        //     }
        //     None => bail!("dataset not found"),
        // }
    }

    async fn dataset_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>> {
        todo!()
    }

    async fn data_services_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<DataService>> {
        todo!()
    }

    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>> {
        todo!()
    }

    async fn distributions_request_by_dataset(
        &self,
        dataset_id: Urn,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<Distribution>> {
        todo!()
    }

    async fn catalog_request(&self) -> anyhow::Result<Vec<Catalog>> {
        todo!()
    }

    async fn catalog_request_by_id(&self) -> anyhow::Result<Vec<Catalog>> {
        todo!()
    }
}
