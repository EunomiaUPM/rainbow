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
use urn::Urn;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transfer_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub state: String,
    pub direction: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::data_plane_field::Entity")]
    DataPlaneFields,
    #[sea_orm(has_many = "super::transfer_event::Entity")]
    TransferEvents,
}

impl Related<super::data_plane_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataPlaneFields.def()
    }
}
impl Related<super::transfer_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferEvents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone)]
pub struct NewDataPlaneProcessModel {
    pub id: Urn,
    pub direction: String,
    pub state: String,
}

impl From<NewDataPlaneProcessModel> for ActiveModel {
    fn from(value: NewDataPlaneProcessModel) -> Self {
        Self {
            id: ActiveValue::Set(value.id.to_string()),
            state: ActiveValue::Set(value.state),
            direction: ActiveValue::Set(value.direction),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
            updated_at: ActiveValue::Set(None),
        }
    }
}

#[derive(Clone)]
pub struct EditDataPlaneProcessModel {
    pub state: Option<String>,
}
