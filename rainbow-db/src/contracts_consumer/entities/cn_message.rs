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
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cn_messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cn_message_id: String,
    pub cn_process_id: String,
    pub _type: String,
    pub subtype: Option<String>,
    pub from: String,
    pub to: String,
    pub created_at: chrono::NaiveDateTime,
    pub content: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cn_process::Entity",
        from = "Column::CnProcessId",
        to = "super::cn_process::Column::ConsumerId"
    )]
    CnProcesses,
    #[sea_orm(has_many = "super::cn_offer::Entity")]
    CnOffers,
    #[sea_orm(has_many = "super::agreement::Entity")]
    Agreements,
}

impl Related<super::cn_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CnProcesses.def()
    }
}

impl Related<super::cn_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CnOffers.def()
    }
}

impl Related<super::agreement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agreements.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
