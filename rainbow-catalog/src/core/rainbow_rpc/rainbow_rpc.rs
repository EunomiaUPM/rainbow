use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_rpc::rainbow_rpc_types::RainbowRPCCatalogResolveDataServiceRequest;
use crate::core::rainbow_rpc::RainbowRPCCatalogTrait;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::policies::EntityTypes;
use axum::async_trait;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;

pub struct RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowRPCCatalogTrait for RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn resolve_data_service(
        &self,
        input: RainbowRPCCatalogResolveDataServiceRequest,
    ) -> anyhow::Result<DataService> {
        let data_service = self
            .repo
            .get_data_service_by_id(input.data_service_id.clone())
            .await
            .map_err(|e| CatalogError::DbErr(e.into()))?
            .ok_or(CatalogError::NotFound {
                id: input.data_service_id,
                entity: EntityTypes::DataService.to_string(),
            })?;
        let data_service = DataService::try_from(data_service)?;
        Ok(data_service)
    }
}
