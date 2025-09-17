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

use crate::data::repo_factory::traits::IntoActiveSet;
use chrono;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub participant_slug: String, // REQUEST
    pub cert: Option<String>,
    pub vc_uri: Option<String>,                  // RESPONSE
    pub vc_issuing: Option<String>,              // RESPONSE
    pub status: String,                          // DEFAULT
    pub created_at: chrono::NaiveDateTime,       // DEFAULT
    pub ended_at: Option<chrono::NaiveDateTime>, // COMPLETION
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,               // REQUEST
    pub participant_slug: String, // REQUEST
    pub cert: Option<String>,
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            participant_slug: ActiveValue::Set(self.participant_slug),
            cert: ActiveValue::Set(self.cert),
            vc_uri: ActiveValue::Set(None),
            vc_issuing: ActiveValue::Set(None),
            status: ActiveValue::Set("Pending".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            participant_slug: ActiveValue::Set(self.participant_slug),
            cert: ActiveValue::Set(self.cert),
            vc_uri: ActiveValue::Set(self.vc_uri),
            vc_issuing: ActiveValue::Set(self.vc_issuing),
            status: ActiveValue::Set(self.status),
            created_at: ActiveValue::Set(self.created_at),
            ended_at: ActiveValue::Set(self.ended_at),
        }
    }
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
