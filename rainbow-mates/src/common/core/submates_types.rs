use rainbow_common::mates::BusMates;
use rainbow_common::utils::get_urn;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapMateRequest {
    pub participant_slug: Option<String>,
    pub participant_type: String,
    pub base_url: String,
}

impl Into<BusMates> for BootstrapMateRequest {
    fn into(self) -> BusMates {
        BusMates {
            id: "".to_string(),
            participant_id: get_urn(None).to_string(),
            token: None,
            token_actions: Some("talk".to_string()),
            saved_at: chrono::Utc::now().naive_utc(),
            last_interaction: chrono::Utc::now().naive_utc(),
        }
    }
}