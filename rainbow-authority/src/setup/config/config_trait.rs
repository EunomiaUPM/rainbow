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
use std::format;
use crate::services::access_manager::config::AccessManagerServiceConfig;
use crate::services::oidc::config::OidcServiceConfig;
use crate::services::wallet::WalletServiceConfig;
use crate::setup::database::DatabaseConfig;

pub trait AuthorityApplicationConfigTrait {
    fn get_full_db_url(&self) -> String;
    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self;
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn parse_to_wallet(&self) -> WalletServiceConfig;
    fn parse_to_access(&self) -> AccessManagerServiceConfig;
    fn parse_to_oidc(&self) -> OidcServiceConfig;
    fn get_host(&self) -> String;
    fn is_local(&self) -> bool;
    fn get_weird_port(&self) -> String;
}
