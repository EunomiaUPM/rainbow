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
use crate::ssi::common::config::{CommonAuthConfig, CommonConfigTrait};
use crate::ssi::common::types::enums::VcDataModelVersion;
use crate::ssi::provider::config::AuthProviderConfigTrait;
use rainbow_common::config::global_config::extract_env;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Serialize, Debug)]
pub struct AuthProviderConfig {
    #[serde(flatten)]
    pub common_config: CommonAuthConfig,
    pub field_extra: bool,
}

impl AuthProviderConfig {
    pub fn new(common_config: CommonAuthConfig) -> Self {
        AuthProviderConfig { common_config, field_extra: true }
    }
}

impl From<ApplicationProviderConfig> for AuthProviderConfig {
    fn from(config: ApplicationProviderConfig) -> Self {
        Self {
            common_config: CommonAuthConfig {
                host: config.auth_host.unwrap(),
                role: config.role.to_string(),
                database_config: config.database_config,
                ssi_wallet_config: config.ssi_wallet_config,
                client: config.client_config,
                keys_path: config.keys_path,
                is_local: config.is_local,
                openapi_path: config.openapi_path,
                api_version: config.api_version,
                self_issuer: VcDataModelVersion::V1,
            },
            field_extra: false,
        }
    }
}

impl AuthProviderConfig {
    pub fn merge_dotenv_configuration(env_file: Option<String>) -> Self {
        let common_default = CommonAuthConfig::default_4_provider();
        let common_config = CommonAuthConfig::merge_dotenv_configuration(env_file, common_default);

        let config = AuthProviderConfig::new(common_config);
        AuthProviderConfig {
            common_config: config.common_config,
            field_extra: extract_env("EXTRA", config.field_extra.to_string()).parse().unwrap(),
        }
    }
}

impl AuthProviderConfigTrait for AuthProviderConfig {
    fn get_full_db_url(&self) -> String {
        self.common_config.get_full_db_url()
    }
    fn get_wallet_api_url(&self) -> String {
        self.common_config.get_wallet_api_url()
    }
    fn get_wallet_register_data(&self) -> Value {
        self.common_config.get_wallet_register_data()
    }
    fn get_wallet_login_data(&self) -> Value {
        self.common_config.get_wallet_login_data()
    }

    fn get_cert(&self) -> anyhow::Result<String> {
        self.common_config.get_cert()
    }
    fn get_priv_key(&self) -> anyhow::Result<String> {
        self.common_config.get_priv_key()
    }
    fn get_pub_key(&self) -> anyhow::Result<String> {
        self.common_config.get_pub_key()
    }
    fn get_host(&self) -> String {
        self.common_config.get_host()
    }

    fn is_local(&self) -> bool {
        self.common_config.is_local()
    }

    fn get_weird_port(&self) -> String {
        self.common_config.get_weird_port()
    }
    fn get_openapi_json(&self) -> anyhow::Result<String> {
        self.common_config.get_openapi_json()
    }
    fn get_api_path(&self) -> String {
        self.common_config.get_api_path()
    }
    fn gaia(&self) -> bool {
        // TODO
        true
    }
}
