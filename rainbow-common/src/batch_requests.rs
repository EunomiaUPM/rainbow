use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchRequests {
    pub ids: Vec<Urn>,
}