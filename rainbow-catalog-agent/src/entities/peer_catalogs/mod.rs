pub(crate) mod peer_catalogs;

use crate::protocols::dsp::types::catalog_definition::Catalog;

#[async_trait::async_trait]
pub trait PeerCatalogTrait: Send + Sync {
    async fn get_peer_catalog(&self, peer_id: &String) -> anyhow::Result<Option<Catalog>>;
    async fn set_peer_catalog(&self, peer_id: &String, catalog: &Catalog) -> anyhow::Result<()>;
}
