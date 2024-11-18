use anyhow::bail;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! config_field {
    ($args:expr, $field:ident, $env_var:expr, $default:expr) => {{
        // Check if TEST=true in the environment variables
        if std::env::var("TEST").as_deref() == Ok("true") {
            // Do not read from environment variables
            $args.$field.clone().unwrap_or_else(|| $default.to_string())
        } else {
            // Original behavior: attempt to read from environment variables
            $args
                .$field
                .clone()
                .or_else(|| std::env::var($env_var).ok())
                .unwrap_or_else(|| $default.to_string())
        }
    }};
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigRoles {
    Catalog,
    Contracts,
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

pub fn get_provider_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let url = match config.role {
        ConfigRoles::Consumer => format!(
            "{}:{}",
            config.provider_url.clone().unwrap(),
            config.provider_port.clone().unwrap()
        ),
        ConfigRoles::Auth => format!("{}:{}", config.host_url, config.host_port),
        _ => format!("{}:{}", config.host_url, config.host_port),
    };
    Ok(url)
}

pub fn get_local_database_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let protocol = match config.db_type.as_str() {
        "postgres" => "postgres",
        // "memory" => "memory",
        _ => bail!("Unsupported Persistence Provider Type"),
    };

    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        protocol,
        config.db_user,
        config.db_password,
        config.db_url,
        config.db_port,
        config.db_database
    );

    Ok(url)
}

pub fn get_consumer_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let url = format!("{}:{}", config.host_url, config.host_port);
    Ok(url)
}