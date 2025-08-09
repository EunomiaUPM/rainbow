/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use anyhow::anyhow;
use sea_orm::Value;
use std::fmt::Display;

pub mod catalog_definition;
pub mod dataservice_definition;
pub mod dataset_definition;
pub mod distribution_definition;


pub enum EntityTypes {
    Catalog,
    Dataset,
    DataService,
    Distribution,
    CatalogRecord,
    Resource,
    DatasetSeries,
    Reference,
    Keyword,
    Theme,
    Relation,
    QualifiedRelation,
}

impl TryFrom<&str> for EntityTypes {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Catalog" => Ok(EntityTypes::Catalog),
            "Dataset" => Ok(EntityTypes::Dataset),
            "DataService" => Ok(EntityTypes::DataService),
            "Distribution" => Ok(EntityTypes::Distribution),
            "CatalogRecord" => Ok(EntityTypes::CatalogRecord),
            "Resource" => Ok(EntityTypes::Resource),
            "DatasetSeries" => Ok(EntityTypes::DatasetSeries),
            "Reference" => Ok(EntityTypes::Reference),
            "Keyword" => Ok(EntityTypes::Keyword),
            "Theme" => Ok(EntityTypes::Theme),
            "Relation" => Ok(EntityTypes::Relation),
            "QualifiedRelation" => Ok(EntityTypes::QualifiedRelation),
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
            EntityTypes::CatalogRecord => "CatalogRecord".to_string(),
            EntityTypes::Resource => "Resource".to_string(),
            EntityTypes::DatasetSeries => "DatasetSeries".to_string(),
            EntityTypes::Reference => "Reference".to_string(),
            EntityTypes::Keyword => "Keyword".to_string(),
            EntityTypes::Theme => "Theme".to_string(),
            EntityTypes::Relation => "Relation".to_string(),
            EntityTypes::QualifiedRelation => "QualifiedRelation".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<EntityTypes> for Value {
    fn from(value: EntityTypes) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

