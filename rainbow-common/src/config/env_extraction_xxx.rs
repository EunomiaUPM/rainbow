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

use crate::config::consumer::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use crate::config::provider::config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use tracing::info;

pub trait EnvExtraction {
    fn extract_provider_config(env_file: Option<String>) -> anyhow::Result<ApplicationProviderConfig> {
        let config = ApplicationProviderConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let mut config_table = config.clone();
        let len = config_table.datahub_token.len();
        let max_len = if len > 20 { 20 } else { len };
        config_table.datahub_token = format!("{}...", config_table.datahub_token[0..max_len].to_string());
        let table = json_to_table::json_to_table(&serde_json::to_value(&config_table)?).collapse().to_string();
        info!("Current Application Provider Config:\n{}", table);
        Ok(config)
    }
    fn extract_consumer_config(env_file: Option<String>) -> anyhow::Result<ApplicationConsumerConfig> {
        let config = ApplicationConsumerConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current Application Consumer Config:\n{}", table);
        Ok(config)
    }
}
