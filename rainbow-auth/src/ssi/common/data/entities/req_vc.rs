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
use rainbow_common::data::IntoActiveSet;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "req_vc")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub authority_id: String,   // REQUEST
    pub authority_slug: String, // REQUEST
    pub grant_endpoint: String, // REQUEST
    pub vc_type: String,
    pub assigned_id: Option<String>, // RESPONSE
    pub vc_uri: Option<String>,
    pub status: String,                          // DEFAULT
    pub created_at: chrono::NaiveDateTime,       // DEFAULT
    pub ended_at: Option<chrono::NaiveDateTime>, // COMPLETION
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,             // REQUEST
    pub authority_id: String,   // REQUEST
    pub authority_slug: String, // REQUEST
    pub grant_endpoint: String, // REQUEST
    pub vc_type: String,
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            authority_id: ActiveValue::Set(self.authority_id),
            authority_slug: ActiveValue::Set(self.authority_slug),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            vc_type: ActiveValue::Set(self.vc_type),
            assigned_id: ActiveValue::Set(None),
            vc_uri: ActiveValue::Set(None),
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
            authority_id: ActiveValue::Set(self.authority_id),
            authority_slug: ActiveValue::Set(self.authority_slug),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            vc_type: ActiveValue::Set(self.vc_type),
            assigned_id: ActiveValue::Set(self.assigned_id),
            vc_uri: ActiveValue::Set(self.vc_uri),
            status: ActiveValue::Set(self.status),
            created_at: ActiveValue::Set(self.created_at),
            ended_at: ActiveValue::Set(self.ended_at),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
