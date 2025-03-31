use axum::extract::rejection::JsonRejection;
use rainbow_db::events::repo::EventRepoErrors;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum SubscriptionErrors {
    #[error("Error from database: {0}")]
    DbErr(EventRepoErrors),
    #[error("{entity} with id {} not found", id.as_str())]
    NotFound { id: Urn, entity: String },
    #[error("Error from deserializing JSON: {0}")]
    JsonRejection(JsonRejection),
    #[error("Error from deserializing path. {0}")]
    UrnUuidSchema(String),
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionErrorMessage {
    pub code: String,
    pub title: String,
    pub message: String,
}