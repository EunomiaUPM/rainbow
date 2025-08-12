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
use sea_orm::ActiveValue;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,                              // REQUEST
    pub provider_id: String,                     // REQUEST
    pub provider_slug: String,                   // REQUEST
    pub grant_endpoint: String,                  // REQUEST
    pub continue_endpoint: Option<String>,       // RESPONSE
    pub continue_wait: Option<i64>,              // RESPONSE
    pub continue_token: Option<String>,          // RESPONSE
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

impl From<NewModel> for ActiveModel {
    fn from(model: NewModel) -> ActiveModel {
        Self {
            id: ActiveValue::Set(model.id),
            provider_id: ActiveValue::Set(model.provider_id),
            provider_slug: ActiveValue::Set(model.provider_slug),
            grant_endpoint: ActiveValue::Set(model.grant_endpoint),
            continue_endpoint: ActiveValue::Set(None),
            continue_wait: ActiveValue::Set(None),
            continue_token: ActiveValue::Set(None),
            assigned_id: ActiveValue::Set(None),
            token: ActiveValue::Set(None),
            status: ActiveValue::Set("Processing".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth_interaction::Entity")]
    AuthInteraction,
    #[sea_orm(has_one = "super::auth_verification::Entity")]
    AuthVerification,
    #[sea_orm(has_one = "super::auth_token_requirements::Entity")]
    AuthTokenRequirements,
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
impl Related<super::auth_token_requirements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthTokenRequirements.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
