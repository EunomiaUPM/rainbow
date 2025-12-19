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
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rainbow_common::data::IntoActiveSet;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "recv_interaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // RESPONSE
    pub start: Vec<String>,        // RESPONSE
    pub method: String,            // RESPONSE
    pub uri: String,               // RESPONSE
    pub client_nonce: String,      // RESPONSE
    pub hash_method: String,       // RESPONSE
    pub hints: Option<String>,     // RESPONSE
    pub grant_endpoint: String,    // RESPONSE
    pub continue_endpoint: String, // RESPONSE
    pub continue_id: String,       // RESPONSE
    pub continue_token: String,    // RESPONSE
    pub as_nonce: String,          // RANDOM
    pub interact_ref: String,      // RANDOM
    pub hash: String,              // RANDOM
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
    pub continue_endpoint: String,   // RESPONSE
    pub continue_token: String,      // RESPONSE
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
        let continue_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();

        let hash_method = self.hash_method.unwrap_or_else(|| "sha-256".to_string()); // TODO
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            self.client_nonce, as_nonce, interact_ref, self.grant_endpoint
        );

        let cont_endpoint = format!("{}/{}", self.continue_endpoint, continue_id);
        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let hash = URL_SAFE_NO_PAD.encode(result);

        ActiveModel {
            id: ActiveValue::Set(self.id),
            start: ActiveValue::Set(self.start),
            method: ActiveValue::Set(self.method),
            uri: ActiveValue::Set(self.uri),
            client_nonce: ActiveValue::Set(self.client_nonce),
            hash_method: ActiveValue::Set(hash_method),
            hints: ActiveValue::Set(self.hints),
            grant_endpoint: ActiveValue::Set(self.grant_endpoint),
            continue_endpoint: ActiveValue::Set(cont_endpoint),
            continue_id: ActiveValue::Set(continue_id),
            continue_token: ActiveValue::Set(self.continue_token),
            as_nonce: ActiveValue::Set(as_nonce),
            interact_ref: ActiveValue::Set(interact_ref),
            hash: ActiveValue::Set(hash),
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
            continue_id: ActiveValue::Set(self.continue_id),
            continue_token: ActiveValue::Set(self.continue_token),
            as_nonce: ActiveValue::Set(self.as_nonce),
            interact_ref: ActiveValue::Set(self.interact_ref),
            hash: ActiveValue::Set(self.hash),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
