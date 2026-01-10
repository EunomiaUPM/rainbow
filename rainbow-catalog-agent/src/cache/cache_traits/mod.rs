use crate::CatalogDto;
use urn::Urn;

pub(crate) mod entity_cache_trait;
pub(crate) mod lookup_cache_trait;
pub(crate) mod peer_catalog_cache_trait;
pub(crate) mod redis_cache_connector_trait;
pub(crate) mod utils_trait;

const ONE_DAY_TTL: i32 = 86400;
pub(crate) const DESIRED_CACHE_TTL: i32 = ONE_DAY_TTL * 2;

pub(crate) const PEER_CATALOG_DESIRED_CACHE_TTL: i32 = ONE_DAY_TTL * 10;
