use std::path::PathBuf;
use std::sync::OnceLock;

pub static CF: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub host_url: String,
    pub host_port: String,
    pub crt: Option<PathBuf>,
    pub key: Option<PathBuf>,
}
