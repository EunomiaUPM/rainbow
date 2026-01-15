use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum SecretSource {
    Plain(String),
    Base64(String),
    VaultRef { path: String, key: String },
    EnvVar(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretString {
    pub source: SecretSource,
}

impl SecretString {
    pub fn resolve(&self) -> anyhow::Result<String> {
        // Vault or base decode
        Ok("<fake_resolving>".to_string())
    }
}
