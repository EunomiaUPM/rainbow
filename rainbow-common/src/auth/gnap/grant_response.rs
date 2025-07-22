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

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::auth::gnap::access_token::AccessToken;

#[derive(Serialize, Deserialize, Debug)]
pub struct GrantResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#continue: Option<Continue4GResponse>, // REQUIRED for continuation calls are allowed for this client instance on this grant request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<AccessToken>, // REQUIRED if an access token is included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<Interact4GResponse>, // REQUIRED if interaction is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Subject4GResponse>, // REQUIRED if subject information is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continue4GResponse {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<AccessToken>,
}

impl Continue4GResponse {
    pub fn default4async(uri: String) -> Self {
        Self {
            uri,
            wait: Some(60),
            access_token: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interact4GResponse {
    pub oidc4vp: Option<String>,
    pub redirect: Option<String>, // REQUIRED 4 if redirection
    pub app: Option<String>,      // ...
    pub user_code: Option<String>,
    pub user_code_uri: Option<UserCodeUri4Int>,
    pub finish: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCodeUri4Int {
    pub code: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject4GResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_ids: Option<Vec<Value>>, // REQUIRED if returning Subject Identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion: Option<Vec<Value>>, // REQUIRED if returning assertions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>, // RECOMMENDED
}

impl GrantResponse {
    pub fn default4oidc4vp(id: String, uri: String, continue_uri: String, consumer_nonce: String) -> Self {
        Self {
            r#continue: Some(Continue4GResponse {
                uri: continue_uri, // TODO Get here
                wait: None, // TODO Manage wait time
                access_token: None,
            }),
            access_token: None,
            interact: Some(Interact4GResponse::default4oidc4vp(uri, consumer_nonce)),
            subject: None,
            instance_id: Some(id),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            r#continue: None,
            access_token: None,
            interact: None,
            subject: None,
            instance_id: None,
            error: Some(error),
        }
    }

    pub fn default4async(id: String, uri: String) -> Self {
        Self {
            r#continue: Some(Continue4GResponse::default4async(uri)),
            access_token: None,
            interact: None,
            subject: None,
            instance_id: Some(id),
            error: None,
        }
    }
}

impl Interact4GResponse {
    fn default4oidc4vp(uri: String, consumer_nonce: String) -> Self {
        Self {
            oidc4vp: Some(uri),
            redirect: None,
            app: None,
            user_code: None,
            user_code_uri: None,
            finish: Some(consumer_nonce),
            expires_in: None, // TODO COMPLETAR
        }
    }
}