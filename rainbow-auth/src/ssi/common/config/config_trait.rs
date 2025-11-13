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

use serde_json::Value;
use rainbow_common::config::global_config::{extract_env, option_extract_env, DatabaseConfig, HostConfig};
use crate::ssi::common::config::CommonAuthConfig;
use crate::ssi::common::types::wallet::WalletConfig;


pub trait CommonConfigTrait: Send + Sync + 'static {
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn get_full_db_url(&self) -> String;
    fn get_raw_wallet_config(&self) -> WalletConfig;
    fn get_wallet_api_url(&self) -> String;
    fn get_wallet_register_data(&self) -> Value;
    fn get_wallet_login_data(&self) -> Value;
    fn get_cert(&self) -> anyhow::Result<String>;
    fn get_priv_key(&self) -> anyhow::Result<String>;
    fn get_pub_key(&self) -> anyhow::Result<String>;
    fn get_host(&self) -> String;
    fn is_local(&self) -> bool;
    fn get_weird_port(&self) -> String;
}