use anyhow::anyhow;
use sea_orm::Value;
use std::fmt::Display;

pub enum EntityTypes {
    Catalog,
    Dataset,
    DataService,
    Distribution,
}

impl TryFrom<&str> for EntityTypes {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Catalog" => Ok(EntityTypes::Catalog),
            "Dataset" => Ok(EntityTypes::Dataset),
            "DataService" => Ok(EntityTypes::DataService),
            "Distribution" => Ok(EntityTypes::Distribution),
            _ => Err(anyhow!("Invalid Entity Type")),
        }
    }
}

impl Display for EntityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EntityTypes::Catalog => "Catalog".to_string(),
            EntityTypes::Dataset => "Dataset".to_string(),
            EntityTypes::DataService => "DataService".to_string(),
            EntityTypes::Distribution => "Distribution".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<EntityTypes> for Value {
    fn from(value: EntityTypes) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}