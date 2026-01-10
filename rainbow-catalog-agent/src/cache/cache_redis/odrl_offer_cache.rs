use crate::cache::cache_traits::redis_cache_connector_trait::RedisCacheConnectorTrait;
use crate::cache::cache_traits::utils_trait::UtilsCacheTrait;
use crate::{CatalogDto, DataServiceDto, OdrlPolicyDto};
use rainbow_common::dsp_common::odrl::OdrlOffer;
use std::str::FromStr;
use urn::Urn;

pub struct OdrlOfferCacheForRedis {
    redis_connection: redis::aio::MultiplexedConnection,
}

impl OdrlOfferCacheForRedis {
    pub fn new(redis_connection: redis::aio::MultiplexedConnection) -> Self {
        Self { redis_connection }
    }
}

impl UtilsCacheTrait for OdrlOfferCacheForRedis {
    type Dto = OdrlPolicyDto;
}

impl RedisCacheConnectorTrait for OdrlOfferCacheForRedis {
    type Dto = OdrlPolicyDto;
    fn get_conn(&self) -> redis::aio::MultiplexedConnection {
        self.redis_connection.clone()
    }
    fn get_entity_name(&self) -> &str {
        "odrl-offers"
    }
}
