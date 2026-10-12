/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use ymir::errors::{Errors, Outcome};

use crate::config::types::cache::{CacheConfig, CacheType};

/// Access to the cache settings of a config.
pub trait CacheConfigTrait {
    fn cache_config(&self) -> &CacheConfig;
    /// Redis URL with credentials; fails for any other `cache_type`, which has no URL.
    fn get_full_cache_url(&self) -> Outcome<String> {
        let cache = self.cache_config();
        match cache.cache_type {
            CacheType::Redis => Ok(format!(
                "{}://{}:{}@{}:{}",
                cache.cache_type,
                cache.user,
                cache.password.expose(),
                cache.url,
                cache.port
            )),
            ref other => Err(Errors::parse(
                format!("cache_type `{other}` has no URL: only Redis is supported here"),
                None,
            )),
        }
    }
}
