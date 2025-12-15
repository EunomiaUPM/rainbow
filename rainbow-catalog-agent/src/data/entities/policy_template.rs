/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use urn::UrnBuilder;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "catalog_policy_templates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
    pub operand_options: Option<serde_json::Value>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::catalog::Entity", from = "Column::Entity", to = "super::catalog::Column::Id")]
    Catalog,
    #[sea_orm(belongs_to = "super::dataset::Entity", from = "Column::Entity", to = "super::dataset::Column::Id")]
    Dataset,
    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::Entity",
        to = "super::dataservice::Column::Id"
    )]
    DataService,
    #[sea_orm(
        belongs_to = "super::distribution::Entity",
        from = "Column::Entity",
        to = "super::distribution::Column::Id"
    )]
    Distribution,
}
impl Related<super::catalog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}
impl Related<super::dataset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dataset.def()
    }
}
impl Related<super::dataservice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataService.def()
    }
}
impl Related<super::distribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Distribution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct NewPolicyTemplateModel {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
    pub operand_options: Option<serde_json::Value>,
}

impl From<NewPolicyTemplateModel> for ActiveModel {
    fn from(dto: NewPolicyTemplateModel) -> Self {
        let new_urn = UrnBuilder::new("policy-template", uuid::Uuid::new_v4().to_string().as_str())
            .build()
            .expect("UrnBuilder failed");
        Self {
            id: ActiveValue::Set(dto.id.clone().unwrap_or(new_urn.clone().to_string()).to_string()),
            title: ActiveValue::Set(dto.title),
            description: ActiveValue::Set(dto.description),
            content: ActiveValue::Set(dto.content),
            operand_options: ActiveValue::Set(dto.operand_options),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
        }
    }
}
