use crate::cache::cache_traits::entity_cache_trait::EntityCacheTrait;
use crate::cache::cache_traits::peer_catalog_cache_trait::PeerCatalogCacheTrait;
use crate::{CatalogDto, DataServiceDto, DatasetDto, DistributionDto, OdrlPolicyDto};
use std::sync::Arc;

#[mockall::automock]
pub trait CatalogAgentCacheTrait: Send + Sync + 'static {
    fn get_catalog_cache(&self) -> Arc<dyn EntityCacheTrait<CatalogDto>>;
    fn get_dataservice_cache(&self) -> Arc<dyn EntityCacheTrait<DataServiceDto>>;
    fn get_dataset_cache(&self) -> Arc<dyn EntityCacheTrait<DatasetDto>>;
    fn get_distribution_cache(&self) -> Arc<dyn EntityCacheTrait<DistributionDto>>;
    fn get_odrl_offer_cache(&self) -> Arc<dyn EntityCacheTrait<OdrlPolicyDto>>;
    fn get_peer_catalog_cache(&self) -> Arc<dyn PeerCatalogCacheTrait>;
}
