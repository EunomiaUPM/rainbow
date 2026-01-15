use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Int,
    Boolean,
    Secret,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
}

pub type ParameterValues = HashMap<String, String>;
