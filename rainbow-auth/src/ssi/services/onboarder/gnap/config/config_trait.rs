/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use ymir::config::types::CommonHostsConfig;
use ymir::types::gnap::grant_request::Client4GR;

pub trait GnapOnboarderConfigTrait {
    fn get_pretty_client_config(&self, cert: &str) -> anyhow::Result<Client4GR>;
    fn hosts(&self) -> &CommonHostsConfig;
    fn get_api_path(&self) -> String;
}
