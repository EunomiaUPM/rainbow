use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PolicyTemplateAllowedDefaultValues {
    Stringable(String),
    Numerable(f32),
}
