use crate::config::GLOBAL_CONFIG;

pub fn get_auth_url() -> anyhow::Result<String> {
    let config = GLOBAL_CONFIG.get().unwrap();
    let url = format!(
        "{}:{}",
        config.auth_url.clone().unwrap(),
        config.auth_port.clone().unwrap()
    );
    Ok(url)
}
