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

use chrono;
use sea_orm::entity::prelude::*;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub consumer: String,
    pub actions: JsonValue, // IT IS A VEC!!
    pub status: Status,
    pub created_at: chrono::NaiveDateTime,
    pub ended_at: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth_interaction::Entity")]
    AuthInteraction,
    #[sea_orm(has_one = "super::auth_verification::Entity")]
    AuthVerification,
}

impl Related<super::auth_interaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthInteraction.def()
    }
}

impl Related<super::auth_verification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthVerification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Ongoing")]
    Ongoing,

    #[sea_orm(string_value = "Completed")]
    Completed,

    #[sea_orm(string_value = "Failed")]
    Failed,

    #[sea_orm(string_value = "Expired")]
    Expired,
}
