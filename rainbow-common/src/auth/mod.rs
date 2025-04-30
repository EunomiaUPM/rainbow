use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrantRequest {
    pub access_token: AccessTokenRequirements4GR,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Subject4GR>, // REQUIRED if requesting subject information
    pub client: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub interact: Interact4GR,
}

impl GrantRequest {
    pub fn default4oidc() -> Self {
        Self {
            access_token: AccessTokenRequirements4GR::default(),
            subject: None,
            client: "".to_string(),
            user: None,
            interact: Interact4GR::default4oidc(),
        }
    }

    pub fn default_with_nonce(nonce: String) -> Self {
        let mut ret = Self {
            access_token: AccessTokenRequirements4GR::default(),
            subject: None,
            client: "".to_string(),
            user: None,
            interact: Interact4GR::default4oidc(),
        };

        ret.interact.finish.nonce = nonce;
        ret
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenRequirements4GR {
    pub access: Access4AT,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>, // REQUIRED if used as part of a request for multiple access tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>, // A set of flags that indicate desired attributes or behavior to be attached to the access token by the AS
}

impl AccessTokenRequirements4GR {
    pub fn default() -> Self {
        Self {
            access: Access4AT {
                r#type: String::from("provider-api"),
                actions: Some(vec![String::from("talk")]),
                locations: None,
                datatypes: None,
                identifier: None,
                privileges: None,
            },
            label: None,
            flags: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Access4AT {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>, // Actions4Access4AT COMPLETAR
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
#[serde(rename_all = "lowercase")]
pub enum Actions4Access4AT {
    TALK,
    NEGOTIATE,
    EXCHANGE,
    READ,
    WRITE,
    DELETE,
}

impl Actions4Access4AT {
    pub fn to_string(&self) -> &str {
        match *self {
            Actions4Access4AT::TALK => "talk",
            Actions4Access4AT::NEGOTIATE => "negotiate",
            Actions4Access4AT::EXCHANGE => "exchange",
            Actions4Access4AT::READ => "read",
            Actions4Access4AT::WRITE => "write",
            Actions4Access4AT::DELETE => "delete",
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject4GRRes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_ids: Option<Vec<Value>>, // REQUIRED if returning Subject Identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion: Option<Vec<Value>>, // REQUIRED if returning assertions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>, // RECOMENDED
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interact4GR {
    pub start: Vec<String>,
    pub finish: Finish4Interact, // REQUIRED because DataSpace Protocol is based on redirects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Value>,
}

impl Interact4GR {
    pub fn default4oidc() -> Self {
        let nonce: String =
            rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();

        Self {
            start: vec![String::from("oidc4vp")],
            finish: Finish4Interact {
                method: String::from("redirect"),
                uri: Some("".to_string()), // COMPLETAR
                nonce,
                hash_method: None,
            },
            hints: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interact4GRRes {
    pub oidc4vp: Option<String>,
    pub redirect: Option<String>, // REQUIRED 4 if redirection
    pub app: Option<String>,      // ...
    pub user_code: Option<String>,
    pub user_code_uri: Option<UserCodeUri4Int>,
    pub finish: Option<String>,
    pub expires_in: Option<u64>,
}

impl Interact4GRRes {
    fn default4oidc4vp(uri: String, consumer_nonce: String) -> Self {
        Self {
            oidc4vp: Some(uri),
            redirect: None,
            app: None,
            user_code: None,
            user_code_uri: None,
            finish: Some(consumer_nonce),
            expires_in: None, // COMPLETAR
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCodeUri4Int {
    pub code: String,
    pub uri: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GrantRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#continue: Option<Continue4GRRes>, // REQUIRED for continuation calls are allowed for this client instance on this grant request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<AccessToken>, // REQUIRED if an access token is included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<Interact4GRRes>, // REQUIRED if interaction is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Subject4GRRes>, // REQUIRED if subject information is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GrantRequestResponse {
    pub fn default4oidc4vp(id: String, uri: String, consumer_nonce: String) -> Self {
        Self {
            r#continue: None,
            access_token: None,
            interact: Some(Interact4GRRes::default4oidc4vp(uri, consumer_nonce)),
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continue4GRRes {
    uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<AccessToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub value: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage: Option<Value>,
    pub access: Vec<String>,
    pub expires_in: Option<u64>,
    pub key: Value, // DecodingKey
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct COMPLETAR {}
