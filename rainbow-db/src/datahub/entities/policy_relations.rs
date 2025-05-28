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

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "policy_relations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dataset_id: String,
    pub domain_id: Option<String>,
    pub policy_template_id: String,
    pub extra_content: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::policy_templates::Entity",
        from = "Column::PolicyTemplateId",
        to = "super::policy_templates::Column::Id"
    )]
    PolicyTemplate,
}


// Corrected Related implementation for PolicyTemplates
impl Related<super::policy_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PolicyTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
