use crate::cache::cache_traits::redis_cache_connector_trait::RedisCacheConnectorTrait;
use crate::cache::cache_traits::utils_trait::UtilsCacheTrait;
use crate::protocols::dsp::types::distribution_definition::Distribution;
use crate::DistributionDto;
use std::str::FromStr;
use urn::Urn;

pub struct DistributionCacheForRedis {
    redis_connection: redis::aio::MultiplexedConnection,
}

impl DistributionCacheForRedis {
    pub fn new(redis_connection: redis::aio::MultiplexedConnection) -> Self {
        Self { redis_connection }
    }
}

impl UtilsCacheTrait for DistributionCacheForRedis {
    type Dto = DistributionDto;
}

impl RedisCacheConnectorTrait for DistributionCacheForRedis {
    type Dto = DistributionDto;
    fn get_conn(&mut self) -> &mut redis::aio::MultiplexedConnection {
        &mut self.redis_connection
    }
    fn get_entity_name(&self) -> &str {
        "distributions"
    }
}
