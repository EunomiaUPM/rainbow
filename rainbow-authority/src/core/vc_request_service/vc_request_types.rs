use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VCRequest {
    pub vc_content: serde_json::Value,
}