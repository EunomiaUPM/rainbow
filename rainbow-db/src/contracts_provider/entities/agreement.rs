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
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "agreements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub agreement_id: String,
    pub consumer_participant_id: String,
    pub provider_participant_id: String,
    pub cn_message_id: String,
    pub agreement_content: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cn_message::Entity",
        from = "Column::CnMessageId",
        to = "super::cn_message::Column::CnMessageId"
    )]
    CnMessage,
}

impl Related<super::cn_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CnMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
