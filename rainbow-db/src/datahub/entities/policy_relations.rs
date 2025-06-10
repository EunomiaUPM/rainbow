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

use sea_orm::entity::prelude::*;
use urn::Urn;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "datahub_policy_relations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dataset_id: String,
    pub policy_template_id: String,
    pub extra_content: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relación con el dataset
    #[sea_orm(
        belongs_to = "super::datahub_datasets::Entity",
        from = "Column::DatasetId",
        to = "super::datahub_datasets::Column::Urn"
    )]
    Dataset,

    // Relación con el template
    #[sea_orm(
        belongs_to = "super::policy_templates::Entity",
        from = "Column::PolicyTemplateId",
        to = "super::policy_templates::Column::Id"
    )]
    PolicyTemplate,
}

impl Related<super::datahub_datasets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dataset.def()
    }
}

impl Related<super::policy_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PolicyTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
