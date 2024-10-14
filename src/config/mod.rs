use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod consumer;
pub mod provider;

#[macro_export]
macro_rules! config_field {
    ($args:expr, $field:ident, $env_var:expr, $default:expr) => {
        $args
            .$field
            .clone()
            .or_else(|| std::env::var($env_var).ok())
            .unwrap_or_else(|| $default.to_string())
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigRoles {
    Consumer,
    Provider,
    Auth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host_url: String,
    pub host_port: String,
    pub db_type: String,
    pub db_url: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
    pub provider_url: Option<String>,
    pub provider_port: Option<String>,
    pub auth_url: Option<String>,
    pub auth_port: Option<String>,
    pub role: ConfigRoles,
}

pub static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();
