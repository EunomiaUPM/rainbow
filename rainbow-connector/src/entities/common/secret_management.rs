use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecretSource {
    Plain(String),
    Base64(String),
    VaultRef { path: String, key: String },
    EnvVar(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretString {
    #[serde(flatten)]
    pub source: SecretSource,
}

impl SecretString {
    pub async fn resolve(&self) -> anyhow::Result<String> {
        // Vault or base decode
        Ok("<fake_resolving>".to_string())
    }
    async fn write_in_vault(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
