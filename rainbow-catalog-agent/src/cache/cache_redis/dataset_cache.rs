use crate::cache::cache_traits::redis_cache_connector_trait::RedisCacheConnectorTrait;
use crate::cache::cache_traits::utils_trait::UtilsCacheTrait;
use crate::{CatalogDto, DatasetDto};
use std::str::FromStr;
use urn::Urn;

pub struct DatasetCacheForRedis {
    redis_connection: redis::aio::MultiplexedConnection,
}

impl DatasetCacheForRedis {
    pub fn new(redis_connection: redis::aio::MultiplexedConnection) -> Self {
        Self { redis_connection }
    }
}

impl UtilsCacheTrait for DatasetCacheForRedis {
    type Dto = DatasetDto;
}

impl RedisCacheConnectorTrait for DatasetCacheForRedis {
    type Dto = DatasetDto;
    fn get_conn(&self) -> redis::aio::MultiplexedConnection {
        self.redis_connection.clone()
    }
    fn get_entity_name(&self) -> &str {
        "datasets"
    }
}
