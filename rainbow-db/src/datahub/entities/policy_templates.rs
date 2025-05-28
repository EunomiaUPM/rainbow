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
#[sea_orm(table_name = "policy_templates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // A PolicyTemplate can be related to many PolicyRelations (N:N through policy_relations table)
    #[sea_orm(has_many = "super::policy_relations::Entity")]
    PolicyRelations,
}

// Corrected Related implementation for PolicyRelations
impl Related<super::policy_relations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PolicyRelations.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
