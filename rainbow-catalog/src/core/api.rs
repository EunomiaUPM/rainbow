use crate::data::db_connection;
use crate::data::entities::catalog::Entity as CatalogEntity;
use crate::protocol::catalog_definition::Catalog;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogRequestMessage {
    #[serde(rename = "@context")] // TODO Define validators
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:filter")]
    pub filter: Option<Value>, // TODO Define further
}

pub async fn catalog_request() -> anyhow::Result<Vec<Catalog>> {
    let db_connection = db_connection().await?;
    let catalogs = CatalogEntity::find().all(&db_connection).await?;
    let mut catalogs_protocol: Vec<Catalog> = vec![];
    for catalog in catalogs {
        catalogs_protocol.push(Catalog::try_from(catalog).unwrap())
    }
    Ok(catalogs_protocol)
}

