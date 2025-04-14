use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use axum::async_trait;
use urn::Urn;

pub mod ds_protocol;
pub mod ds_protocol_errors;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolCatalogTrait: Sync + Send {
    async fn dataset_request(&self, dataset_id: Urn) -> anyhow::Result<Dataset>;
    async fn dataset_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>>;
    async fn data_services_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<DataService>>;
    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>>;
    async fn distributions_request_by_dataset(
        &self,
        dataset_id: Urn,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<Distribution>>;
    async fn catalog_request(&self) -> anyhow::Result<Vec<Catalog>>;
    async fn catalog_request_by_id(&self, catalog_id: Urn) -> anyhow::Result<Catalog>;
}
