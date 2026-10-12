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

//! Cache of resolved JSON-LD contexts. Contexts are versioned documents that barely change, so
//! entries live long and a process fetches each one about once a day.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::rdf::loader::MetaVal;

/// Resolved contexts by URL, each valid for `ttl` after it was stored.
#[derive(Clone, Debug)]
pub struct RdfContextCache {
    entries: Arc<RwLock<HashMap<String, (MetaVal, Instant)>>>,
    ttl: Duration,
}

impl Default for RdfContextCache {
    fn default() -> Self {
        Self::new(Self::DEFAULT_TTL)
    }
}

impl RdfContextCache {
    /// A day: long enough to fetch a context rarely, short enough to pick up a corrected one.
    pub const DEFAULT_TTL: Duration = Duration::from_secs(24 * 60 * 60);

    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    /// The context for `url`, unless it was never stored or has expired.
    pub async fn get(&self, url: &str) -> Option<MetaVal> {
        let entries = self.entries.read().await;
        entries
            .get(url)
            .filter(|(_, stored_at)| stored_at.elapsed() < self.ttl)
            .map(|(doc, _)| doc.clone())
    }

    pub async fn put(&self, url: &str, doc: MetaVal) {
        self.entries
            .write()
            .await
            .insert(url.to_string(), (doc, Instant::now()));
    }

    pub async fn contains(&self, url: &str) -> bool {
        self.get(url).await.is_some()
    }
}
