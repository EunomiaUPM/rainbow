use axum::extract::rejection::JsonRejection;
use rainbow_db::catalog::repo::CatalogRepoErrors;
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum DSProtocolCatalogErrors {
    #[error("{entity} with id {} not found", id.as_str())]
    NotFound { id: Urn, entity: String },
    #[error("Error from database: {0}")]
    DbErr(CatalogRepoErrors),
    #[error("Error from deserializing JSON: {0}")]
    JsonRejection(JsonRejection),
    #[error("Error from deserializing path. {0}")]
    UrnUuidSchema(String),
}