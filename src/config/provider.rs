use crate::config::GLOBAL_CONFIG;
use anyhow::bail;

pub fn get_provider_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let url = format!("{}:{}", config.host_url, config.host_port);
    Ok(url)
}

pub fn get_provider_database_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let protocol = match config.db_type.as_str() {
        "postgres" => "postgres",
        "mongo" => "mongodb",
        "memory" => ":memory:",
        "mysql" => "mysql",
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
