use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub static CONTEXT: &str = "https://w3id.org/dspace/2025/1/context.jsonld";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextField {
    Single(String),
    Multiple(Vec<String>),
}

impl ContextField {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self {
            ContextField::Single(s) => {
                if s == CONTEXT {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
            ContextField::Multiple(v) => {
                if v.iter().any(|s| s == CONTEXT) {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
        }
    }
}

impl Display for ContextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(CONTEXT)
    }
}

impl Default for ContextField {
    fn default() -> Self {
        ContextField::Multiple(vec![CONTEXT.to_string()])
    }
}
