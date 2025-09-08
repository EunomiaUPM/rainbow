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
use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use crate::common::IntoActiveSet;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_token_requirements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub r#type: String,                  // REQUEST
    pub actions: Vec<String>,            // REQUEST
    pub locations: Option<Vec<String>>,  // REQUEST
    pub datatypes: Option<Vec<String>>,  // REQUEST
    pub identifier: Option<String>,      // REQUEST
    pub privileges: Option<Vec<String>>, // REQUEST
    pub label: Option<String>,           // REQUEST
    pub flags: Option<Vec<String>>,      // REQUEST
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            r#type: ActiveValue::Set(self.r#type),
            actions: ActiveValue::Set(self.actions),
            locations: ActiveValue::Set(self.locations),
            datatypes: ActiveValue::Set(self.datatypes),
            identifier: ActiveValue::Set(self.identifier),
            privileges: ActiveValue::Set(self.privileges),
            label: ActiveValue::Set(self.label),
            flags: ActiveValue::Set(self.flags),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth_request::Entity")]
    AuthRequest,
    #[sea_orm(has_one = "super::auth_interaction::Entity")]
    AuthInteraction,
    #[sea_orm(has_one = "super::auth_verification::Entity")]
    AuthVerification,
}

impl Related<super::auth_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthRequest.def()
    }
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
