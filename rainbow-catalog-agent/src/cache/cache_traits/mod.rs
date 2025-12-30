use crate::CatalogDto;
use urn::Urn;

pub(crate) mod entity_cache_trait;
pub(crate) mod lookup_cache_trait;
pub(crate) mod redis_cache_connector_trait;
pub(crate) mod utils_trait;

const ONE_DAY_TTL: i32 = 86400;
const DESIRED_CACHE_TTL: i32 = ONE_DAY_TTL * 2;
