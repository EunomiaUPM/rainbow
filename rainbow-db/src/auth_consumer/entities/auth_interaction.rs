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
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_interaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub start: Vec<String>,                // REQUEST
    pub method: String,                    // REQUEST
    pub uri: String,                       // REQUEST
    pub client_nonce: String,              // RANDOM
    pub hash_method: String,               // REQUEST
    pub hints: Option<String>,             // REQUEST
    pub grant_endpoint: String,            // REQUEST
    pub continue_endpoint: Option<String>, // RESPONSE
    pub continue_token: Option<String>,    // RESPONSE
    pub continue_wait: Option<i64>,        // RESPONSE
    pub as_nonce: Option<String>,          // RESPONSE
    pub interact_ref: Option<String>,      // POST-RESPONSE
    pub hash: Option<String>,              // POST-RESPONSE
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,                  // REQUEST
    pub start: Vec<String>,          // REQUEST
    pub method: String,              // REQUEST
    pub uri: String,                 // REQUEST
    pub hash_method: Option<String>, // REQUEST
    pub hints: Option<String>,       // REQUEST
    pub grant_endpoint: String,      // REQUEST
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let hash_method = self.hash_method.unwrap_or_else(|| "sha-256".to_string()); // TODO
        ActiveModel {
            id: ActiveValue::Set(self.id),
            start: ActiveValue::Set(self.start),
            method: ActiveValue::Set(self.method),
            uri: ActiveValue::Set(self.uri),
            client_nonce: ActiveValue::Set(nonce),
            hash_method: ActiveValue::Set(hash_method),
            hints: ActiveValue::Set(self.hints),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            continue_endpoint: ActiveValue::Set(None),
            continue_token: ActiveValue::Set(None),
            continue_wait: ActiveValue::Set(None),
            as_nonce: ActiveValue::Set(None),
            interact_ref: ActiveValue::Set(None),
            hash: ActiveValue::Set(None),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            start: ActiveValue::Set(self.start),
            method: ActiveValue::Set(self.method),
            uri: ActiveValue::Set(self.uri),
            client_nonce: ActiveValue::Set(self.client_nonce),
            hash_method: ActiveValue::Set(self.hash_method),
            hints: ActiveValue::Set(self.hints),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            continue_endpoint: ActiveValue::Set(self.continue_endpoint),
            continue_token: ActiveValue::Set(self.continue_token),
            continue_wait: ActiveValue::Set(self.continue_wait),
            as_nonce: ActiveValue::Set(self.as_nonce),
            interact_ref: ActiveValue::Set(self.interact_ref),
            hash: ActiveValue::Set(self.hash),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth_request::Entity")]
    AuthRequest,
    #[sea_orm(has_one = "super::auth_verification::Entity")]
    AuthVerification,
    #[sea_orm(has_one = "super::auth_token_requirements::Entity")]
    AuthTokenRequirements,
    #[sea_orm(has_one = "super::authority_request::Entity")]
    AuthorityRequest,
}

impl Related<super::auth_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthRequest.def()
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
impl Related<super::authority_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthorityRequest.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
