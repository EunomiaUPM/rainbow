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

use crate::setup::{AuthorityApplicationConfig, AuthorityApplicationConfigTrait};
use tracing::info;

pub fn extract_env_config(env_file: Option<String>) -> anyhow::Result<AuthorityApplicationConfig> {
    let config = AuthorityApplicationConfig::default();
    let config = config.merge_dotenv_configuration(env_file);
    let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
    info!("Current Application Authority Config:\n{}", table);
    Ok(config)
}
