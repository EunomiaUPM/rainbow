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
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_interaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // RESPONSE
    pub start: Vec<String>,         // RESPONSE
    pub method: String,             // RESPONSE
    pub uri: String,                // RESPONSE
    pub client_nonce: String,       // RESPONSE
    pub hash_method: String,        // RESPONSE
    pub hints: Option<String>,      // RESPONSE
    pub grant_endpoint: String,     // RESPONSE
    pub continue_endpoint: String,  // RESPONSE
    pub continue_id: String,        // RESPONSE
    pub continue_token: String,     // RESPONSE
    pub as_nonce: String,           // RANDOM
    pub interact_ref: String,       // RANDOM
    pub hash: String,               // RANDOM
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,                  // REQUEST
    pub start: Vec<String>,          // REQUEST
    pub method: String,              // REQUEST
    pub uri: String,                 // REQUEST
    pub client_nonce: String,        // REQUEST
    pub hash_method: Option<String>, // REQUEST
    pub hints: Option<String>,       // REQUEST
    pub grant_endpoint: String,      // REQUEST
    pub continue_endpoint: String,  // RESPONSE
    pub continue_token: String,     // RESPONSE
}

impl From<NewModel> for ActiveModel {
    fn from(model: NewModel) -> ActiveModel {
        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
        let continue_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();

        let hash_method = model.hash_method.unwrap_or_else(|| "sha-256".to_string()); // TODO
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            model.client_nonce, as_nonce, interact_ref, model.grant_endpoint
        );

        let cont_endpoint = format!("{}/{}", model.continue_endpoint, continue_id);
        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let hash = URL_SAFE_NO_PAD.encode(result);

        Self {
            id: ActiveValue::Set(model.id),
            start: ActiveValue::Set(model.start),
            method: ActiveValue::Set(model.method),
            uri: ActiveValue::Set(model.uri),
            client_nonce: ActiveValue::Set(model.client_nonce),
            hash_method: ActiveValue::Set(hash_method),
            hints: ActiveValue::Set(model.hints),
            grant_endpoint: ActiveValue::Set(model.grant_endpoint),
            continue_endpoint: ActiveValue::Set(cont_endpoint),
            continue_id: ActiveValue::Set(continue_id),
            continue_token: ActiveValue::Set(model.continue_token),
            as_nonce: ActiveValue::Set(as_nonce),
            interact_ref: ActiveValue::Set(interact_ref),
            hash: ActiveValue::Set(hash),
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
impl ActiveModelBehavior for ActiveModel {}
