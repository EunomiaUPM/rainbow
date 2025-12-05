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
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "transfer_processes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub provider_pid: String,
    pub consumer_pid: Option<String>,
    pub agreement_id: String,
    pub data_plane_id: Option<String>,
    pub associated_consumer: Option<String>,
    pub state: String,
    pub state_attribute: Option<String>,
    pub format: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::transfer_message::Entity")]
    TransferMessages,
}

impl Related<super::transfer_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferMessages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
