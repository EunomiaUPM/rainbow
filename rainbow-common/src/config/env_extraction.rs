use crate::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use crate::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use log::debug;
use tracing::info;

pub trait EnvExtraction {
    fn extract_provider_config(env_file: Option<String>) -> anyhow::Result<ApplicationProviderConfig> {
        let config = ApplicationProviderConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let mut config_table = config.clone();
        let len = config_table.datahub_token.len();
        debug!("{}", len);
        let max_len = if len > 20 { 20 } else { len };
        debug!("{}", max_len);
        config_table.datahub_token = format!("{}...", config_table.datahub_token[0..max_len].to_string());
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config_table)?).collapse().to_string();
        info!("Current Application Provider Config:\n{}", table);
        Ok(config)
    }
    fn extract_consumer_config(env_file: Option<String>) -> anyhow::Result<ApplicationConsumerConfig> {
        let config = ApplicationConsumerConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current Application Consumer Config:\n{}", table);
        Ok(config)
    }
}