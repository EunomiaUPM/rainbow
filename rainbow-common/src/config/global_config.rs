use crate::config::database::DbType;
use serde::Serialize;
use std::env;

pub fn extract_env(env_var_name: &str, default: String) -> String {
    env::var(env_var_name).unwrap_or(default)
}

pub fn format_host_config_to_url_string(hc: &HostConfig) -> String {
    if hc.port.is_empty() {
        format!("{}://{}", hc.protocol, hc.url)
    } else {
        format!("{}://{}:{}", hc.protocol, hc.url, hc.port)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct HostConfig {
    pub protocol: String,
    pub url: String,
    pub port: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub db_type: DbType,
    pub url: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub name: String,
}