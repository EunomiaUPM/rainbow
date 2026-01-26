use crate::entities::common::parameter_visitor::ParameterVisitor;
use crate::entities::common::parameters::{TemplateString, TemplateVisitable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecretSource {
    Plain(TemplateString),
    Base64(TemplateString),
    VaultRef { path: TemplateString, key: TemplateString },
    EnvVar(TemplateString),
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
}

impl TemplateVisitable for SecretString {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("source");
        match &mut self.source {
            SecretSource::Plain(secret) => {
                visitor.enter_scope("plain");
                secret.accept(visitor)?;
                visitor.exit_scope();
            }
            SecretSource::Base64(secret) => {
                visitor.enter_scope("plain");
                secret.accept(visitor)?;
                visitor.exit_scope();
            }
            SecretSource::VaultRef { path, key } => {
                visitor.enter_scope("plain");
                path.accept(visitor)?;
                key.accept(visitor)?;
                visitor.exit_scope();
            }
            SecretSource::EnvVar(env) => {
                visitor.enter_scope("plain");
                env.accept(visitor)?;
                visitor.exit_scope();
            }
        }
        visitor.exit_scope();
        Ok(())
    }
}
