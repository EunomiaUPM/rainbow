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
use std::{format, fs};
use super::ClientConfig;
use crate::setup::database::DatabaseConfig;
use crate::types::wallet::SSIWalletConfig;

pub trait AuthorityApplicationConfigTrait {
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn get_raw_client_config(&self) -> &ClientConfig;
    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig;
    fn get_wallet_portal_url(&self) -> String;
    fn get_full_db_url(&self) -> String;
    fn get_wallet_data(&self) -> serde_json::Value;
    fn get_cert(&self) -> String;
    fn get_priv_key(&self) -> String;
    fn get_pub_key(&self) -> String;
    fn merge_dotenv_configuration(&self) -> Self;
}
