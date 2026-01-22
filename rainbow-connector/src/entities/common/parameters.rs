use crate::entities::connector_template::validator::{ExpectedType, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ParameterName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ParameterType {
    String,
    Int,
    Boolean,
    #[serde(rename = "VEC<STRING>")]
    VecString,
    #[serde(rename = "MAP<STRING,STRING>")]
    MapStringString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefinition {
    pub name: ParameterName,
    pub title: String,
    pub description: Option<String>,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
}

pub type TemplateString = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateInt {
    Value(i64),
    Template(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateBoolean {
    Value(bool),
    Template(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateVecString {
    Value(Vec<String>),
    Template(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateMapString {
    Value(HashMap<String, String>),
    Template(String),
}

pub trait TemplateVisitable {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error>;
}

impl TemplateVisitable for TemplateString {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        let _ = visitor.scan_template_candidate(self.as_str(), ExpectedType::AnyString);
        Ok(())
    }
}

impl TemplateVisitable for TemplateInt {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            TemplateInt::Value(_) => Ok(()),
            TemplateInt::Template(str) => {
                visitor.scan_template_candidate(str, ExpectedType::StrictInt);
                Ok(())
            }
        }
    }
}

impl TemplateVisitable for TemplateBoolean {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            TemplateBoolean::Value(_) => Ok(()),
            TemplateBoolean::Template(s) => {
                visitor.scan_template_candidate(s, ExpectedType::StrictBool);
                Ok(())
            }
        }
    }
}

impl TemplateVisitable for TemplateVecString {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            TemplateVecString::Value(list) => {
                for (index, item) in list.iter_mut().enumerate() {
                    visitor.enter_scope(&index.to_string());
                    visitor.scan_template_candidate(item, ExpectedType::AnyString);
                    visitor.exit_scope();
                }
            }
            TemplateVecString::Template(s) => {
                visitor.scan_template_candidate(s, ExpectedType::StrictVec);
            }
        }
        Ok(())
    }
}

impl TemplateVisitable for TemplateMapString {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            TemplateMapString::Value(map) => {
                for (key, value) in map.iter_mut() {
                    visitor.enter_scope(key);
                    visitor.scan_template_candidate(value, ExpectedType::AnyString);
                    visitor.exit_scope();
                }
            }
            TemplateMapString::Template(s) => {
                visitor.scan_template_candidate(s, ExpectedType::StrictMap);
            }
        }
        Ok(())
    }
}
