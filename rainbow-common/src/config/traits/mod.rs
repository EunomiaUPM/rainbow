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

mod api_trait;
mod common_trait;
mod config_loader;
mod database_trait;
mod extra_traits;
mod host_config_trait;
mod extra_hosts_trait;

pub use api_trait::ApiConfigTrait;
pub use common_trait::CommonConfigTrait;
pub use config_loader::ConfigLoader;
pub use database_trait::DatabaseConfigTrait;
pub use extra_traits::*;
pub use host_config_trait::HostConfigTrait;
pub use extra_hosts_trait::ExtraHostsTrait;