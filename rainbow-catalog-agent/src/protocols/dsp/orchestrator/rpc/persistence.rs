use crate::entities::peer_catalogs::PeerCatalogTrait;
use crate::protocols::dsp::types::catalog_definition::Catalog;
use anyhow::anyhow;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;

pub struct OrchestrationPersistenceForProtocolForRPC {
    peer_catalog_entity_service: Arc<dyn PeerCatalogTrait>,
}

impl OrchestrationPersistenceForProtocolForRPC {
    pub fn new(peer_catalog_entity_service: Arc<dyn PeerCatalogTrait>) -> Self {
        Self { peer_catalog_entity_service }
    }

    pub async fn get_catalog(&self, peer_id: &String) -> anyhow::Result<Option<Catalog>> {
        let catalog =
            self.peer_catalog_entity_service.get_peer_catalog(peer_id).await.map_err(|e| {
                let err = CommonErrors::database_new("Not able to fetch catalog from caché");
                error!("{}", err.log());
                anyhow!(err)
            })?;
        Ok(catalog)
    }

    pub async fn set_catalog(&self, peer_id: &String, catalog: &Catalog) -> anyhow::Result<()> {
        let _ = self.peer_catalog_entity_service.set_peer_catalog(peer_id, catalog).await.map_err(
            |e| {
                let err = CommonErrors::database_new("Not able to set catalog in caché");
                error!("{}", err.log());
                anyhow!(err)
            },
        )?;
        Ok(())
    }
}
