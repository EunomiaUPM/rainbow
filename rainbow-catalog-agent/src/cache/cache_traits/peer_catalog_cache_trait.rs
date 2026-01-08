use crate::protocols::dsp::types::catalog_definition::Catalog;
use urn::Urn;

#[async_trait::async_trait]
pub trait PeerCatalogCacheTrait: Sync + Send {
    async fn get_catalog(&self, participant_id: &String) -> anyhow::Result<Option<Catalog>>;
    async fn set_catalog(&self, participant_id: &String, catalog: &Catalog) -> anyhow::Result<()>;
}
