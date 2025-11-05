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

use crate::services::repo::traits::IntoActiveSet;
use chrono;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeriveEntityModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "minions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub participant_id: String, // REQUEST
    pub participant_slug: String,                // REQUEST
    pub participant_type: String,                // REQUEST
    pub base_url: Option<String>,                // REQUEST
    pub vc_uri: Option<String>,                  // REQUEST
    pub is_vc_issued: bool,
    pub saved_at: chrono::NaiveDateTime,         // DEFAULT
    pub last_interaction: chrono::NaiveDateTime, // DEFAULT
    pub is_me: bool,                             // REQUEST
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub participant_id: String,
    pub participant_slug: String,
    pub participant_type: String,
    pub base_url: Option<String>,
    pub vc_uri: Option<String>,
    pub is_vc_issued: bool,
    pub is_me: bool,
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            participant_id: ActiveValue::Set(self.participant_id),
            participant_slug: ActiveValue::Set(self.participant_slug),
            participant_type: ActiveValue::Set(self.participant_type),
            base_url: ActiveValue::Set(self.base_url),
            vc_uri: ActiveValue::Set(self.vc_uri),
            is_vc_issued: ActiveValue::Set(self.is_vc_issued),
            saved_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            last_interaction: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            is_me: ActiveValue::Set(self.is_me),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            participant_id: ActiveValue::Set(self.participant_id),
            participant_slug: ActiveValue::Set(self.participant_slug),
            participant_type: ActiveValue::Set(self.participant_type),
            base_url: ActiveValue::Set(self.base_url),
            vc_uri: ActiveValue::Set(self.vc_uri),
            is_vc_issued: ActiveValue::Set(self.is_vc_issued),
            saved_at: ActiveValue::Set(self.saved_at),
            last_interaction: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            is_me: ActiveValue::Set(self.is_me),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
