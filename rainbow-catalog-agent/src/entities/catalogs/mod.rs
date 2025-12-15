use crate::data::entities::catalog;
use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use serde::{Deserialize, Serialize};
use urn::Urn;

pub(crate) mod catalogs;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatalogDto {
    #[serde(flatten)]
    pub inner: catalog::Model,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewCatalogDto {
    pub id: Option<Urn>,
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditCatalogDto {
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

impl From<NewCatalogDto> for NewCatalogModel {
    fn from(dto: NewCatalogDto) -> Self {
        Self {
            id: dto.id,
            foaf_home_page: dto.foaf_home_page,
            dct_conforms_to: dto.dct_conforms_to,
            dct_creator: dto.dct_creator,
            dct_title: dto.dct_title,
        }
    }
}

impl From<EditCatalogDto> for EditCatalogModel {
    fn from(dto: EditCatalogDto) -> Self {
        Self {
            foaf_home_page: dto.foaf_home_page,
            dct_conforms_to: dto.dct_conforms_to,
            dct_creator: dto.dct_creator,
            dct_title: dto.dct_title,
        }
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait CatalogEntityTrait {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        no_main_catalog: bool,
    ) -> anyhow::Result<Vec<CatalogDto>>;
    async fn get_batch_catalogs(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<CatalogDto>>;
    async fn get_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<Option<CatalogDto>>;
    async fn get_main_catalog(&self) -> anyhow::Result<Option<CatalogDto>>;

    async fn put_catalog_by_id(
        &self,
        catalog_id: &Urn,
        edit_catalog_model: &EditCatalogDto,
    ) -> anyhow::Result<CatalogDto>;
    async fn create_catalog(&self, new_catalog_model: &NewCatalogDto) -> anyhow::Result<CatalogDto>;

    async fn create_main_catalog(&self, new_catalog_model: &NewCatalogDto) -> anyhow::Result<CatalogDto>;

    async fn delete_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<()>;
}
