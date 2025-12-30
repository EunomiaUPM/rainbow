use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait::async_trait]
pub trait RedisCacheConnectorTrait: Send + Sync {
    type Dto: Serialize + DeserializeOwned + Send + Sync;

    fn get_conn(&mut self) -> &mut redis::aio::MultiplexedConnection;
    fn get_entity_name(&self) -> &str;
}
