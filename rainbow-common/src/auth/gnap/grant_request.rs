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

use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrantRequest {
    pub access_token: AccessTokenRequirements4GR,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Subject4GR>, // REQUIRED if requesting subject information
    pub client: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub interact: Option<Interact4GR>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenRequirements4GR {
    pub access: Access4AT,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>, // REQUIRED if used as part of a request for multiple access tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>, // A set of flags that indicate desired attributes or behavior to be attached to the access token by the AS
    // TODO if Bearer is included the token is not binded to a key
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Access4AT {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>, // Actions4Access4AT COMPLETAR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatypes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subject4GR {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_id_formats: Option<Vec<String>>, // REQUIRED if Subject Identifiers are requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_formats: Option<Vec<String>>, // REQUIRED if assertions are requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_ids: Option<Value>, // If omitted assume that subject information requests are about the current user
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interact4GR {
    pub start: Vec<String>,
    pub finish: Finish4Interact, // REQUIRED because DataSpace Protocol is based on redirects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Finish4Interact {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>, // REQUIRED for redirect and push methods
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_method: Option<String>,
}

impl GrantRequest {
    pub fn default4oidc(client: String, method: String) -> Self {
        Self {
            access_token: AccessTokenRequirements4GR::default(),
            subject: None,
            client: Value::String(client),
            user: None,
            interact: Some(Interact4GR::default4oidc(method)),
        }
    }


    pub fn default4await(cert: String, uri: String) -> Self {
        Self {
            access_token: AccessTokenRequirements4GR::request_vc(), // TODO Fix, es none en este caso
            subject: None,
            client: json!({
                "key" : {
                    "proof": "mtls",
                    "cert#S256": cert
                }

            }),
            user: None,
            interact: Some(Interact4GR::default4oidc(uri)),
        }
    }

    pub fn update_callback(&mut self, callback: String) -> &mut Self {
        if let Some(interact) = self.interact.as_mut() {
            interact.finish.uri = Some(callback);
        }
        self
    }

    pub fn update_actions(&mut self, actions: String) -> &mut Self {
        self.access_token.access.actions = Some(actions);
        self
    }
}

impl AccessTokenRequirements4GR {
    pub fn default() -> Self {
        Self {
            access: Access4AT {
                r#type: String::from("api-access"),
                actions: Some(String::from("talk")),
                locations: None,
                datatypes: None,
                identifier: None,
                privileges: None,
            },
            label: None,
            flags: Some("Bearer".to_string()), // TODO
        }
    }

    pub fn request_vc() -> Self {
        Self {
            access: Access4AT {
                r#type: String::from("vc-exchange"),
                actions: Some(String::from("vc-request")),
                locations: None,
                datatypes: None,
                identifier: None,
                privileges: None,
            },
            label: None,
            flags: None, // TODO
        }
    }
}

impl Interact4GR {
    pub fn default4oidc(method: String) -> Self {
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();

        Self {
            start: vec![String::from("oidc4vp")],
            finish: Finish4Interact { method, uri: None, nonce, hash_method: None },
            hints: None,
        }
    }

    pub fn default4await(uri: String) -> Self {
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        Self {
            start: vec![String::from("await")],
            finish: Finish4Interact { method: "await".to_string(), uri: Some(uri), nonce, hash_method: None },
            hints: None,
        }
    }
}

