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

use crate::common::IntoActiveSet;
use chrono;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "req_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub provider_id: String,                     // REQUEST
    pub provider_slug: String,                   // REQUEST
    pub grant_endpoint: String,                  // REQUEST
    pub assigned_id: Option<String>,             // RESPONSE
    pub token: Option<String>,                   // COMPLETION
    pub status: String,                          // DEFAULT
    pub created_at: chrono::NaiveDateTime,       // DEFAULT
    pub ended_at: Option<chrono::NaiveDateTime>, // COMPLETION
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,             // REQUEST
    pub provider_id: String,    // REQUEST
    pub provider_slug: String,  // REQUEST
    pub grant_endpoint: String, // REQUEST
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            provider_id: ActiveValue::Set(self.provider_id),
            provider_slug: ActiveValue::Set(self.provider_slug),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            assigned_id: ActiveValue::Set(None),
            token: ActiveValue::Set(None),
            status: ActiveValue::Set("Processing".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            provider_id: ActiveValue::Set(self.provider_id),
            provider_slug: ActiveValue::Set(self.provider_slug),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            assigned_id: ActiveValue::Set(self.assigned_id),
            token: ActiveValue::Set(self.token),
            status: ActiveValue::Set(self.status),
            created_at: ActiveValue::Set(self.created_at),
            ended_at: ActiveValue::Set(self.ended_at),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
