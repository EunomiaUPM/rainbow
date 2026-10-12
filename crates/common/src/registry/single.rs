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

//! Registry with one implementation per key.

use super::{Entry, EntryInfo, Origin, Ownership, RegistrationId, RegistryKey};
use arc_swap::ArcSwap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ymir::errors::{Errors, Outcome};

/// Immutable view of a [`Registry`] at one moment.
pub type RegistrySnapshot<K, V> = Arc<HashMap<K, Entry<V>>>;

/// One implementation per key; a taken key must be replaced explicitly.
pub struct Registry<K, V: ?Sized> {
    point: &'static str,
    state: ArcSwap<HashMap<K, Entry<V>>>,
    writer: Mutex<()>,
}

impl<K: RegistryKey, V: ?Sized + Send + Sync + 'static> Registry<K, V> {
    /// Empty registry for the extension point `point` (used in errors and logs).
    pub fn new(point: &'static str) -> Self {
        Self {
            point,
            state: ArcSwap::from_pointee(HashMap::new()),
            writer: Mutex::new(()),
        }
    }

    pub fn point(&self) -> &'static str {
        self.point
    }

    /// Adds an entry; fails if the key is already taken.
    pub fn register(&self, key: K, value: Arc<V>, origin: Origin) -> Outcome<RegistrationId> {
        self.write(|map| {
            if map.contains_key(&key) {
                return Err(Errors::crazy(
                    format!("{}: {key:?} is already registered", self.point),
                    None,
                ));
            }
            let entry = Entry::new(value, origin, 0);
            let id = entry.id;
            map.insert(key, entry);
            Ok(id)
        })
    }

    /// Overrides an entry, or adds it if absent; a plugin cannot override a core entry.
    pub fn replace(&self, key: K, value: Arc<V>, origin: Origin) -> Outcome<RegistrationId> {
        self.write(|map| {
            if let Some(existing) = map.get(&key) {
                if !Ownership::may_replace(&existing.origin, &origin) {
                    return Err(Errors::forbidden(
                        format!("{}: {origin} cannot replace core entry {key:?}", self.point),
                        None,
                    ));
                }
                tracing::info!(point = self.point, ?key, from = %existing.origin, to = %origin, "registry entry replaced");
            }
            let entry = Entry::new(value, origin, 0);
            let id = entry.id;
            map.insert(key, entry);
            Ok(id)
        })
    }

    /// Removes one registration; core entries are permanent.
    pub fn unregister(&self, id: RegistrationId) -> Outcome<()> {
        self.write(|map| {
            let Some(key) = map.iter().find(|(_, e)| e.id == id).map(|(k, _)| k.clone()) else {
                return Err(Errors::missing_resource(
                    format!("{id:?}"),
                    format!("{}: no registration {id:?}", self.point),
                    None,
                ));
            };
            if map[&key].origin == Origin::Core {
                return Err(Ownership::core_is_permanent(self.point, id));
            }
            map.remove(&key);
            Ok(())
        })
    }

    /// Removes every entry registered by `origin`; returns how many went.
    pub fn unregister_origin(&self, origin: &Origin) -> usize {
        if *origin == Origin::Core {
            return 0;
        }
        let mut removed = 0;
        self.apply(|map| {
            let before = map.len();
            map.retain(|_, e| e.origin != *origin);
            removed = before - map.len();
        });
        removed
    }

    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.state.load().get(key).map(|e| e.value.clone())
    }

    /// Like [`Registry::get`], but a missing key is an error naming the point.
    pub fn require(&self, key: &K) -> Outcome<Arc<V>> {
        self.get(key).ok_or_else(|| {
            Errors::missing_resource(
                format!("{key:?}"),
                format!("{}: nothing registered for {key:?}", self.point),
                None,
            )
        })
    }

    /// The whole registry as it is now; later writes do not change it.
    pub fn snapshot(&self) -> RegistrySnapshot<K, V> {
        self.state.load_full()
    }

    pub fn entries(&self) -> Vec<(K, EntryInfo)> {
        self.state
            .load()
            .iter()
            .map(|(k, e)| (k.clone(), e.info()))
            .collect()
    }

    /// Serialises writers, applies `change` to a copy of the map and publishes it on success.
    fn write<T>(&self, change: impl FnOnce(&mut HashMap<K, Entry<V>>) -> Outcome<T>) -> Outcome<T> {
        let _guard = self
            .writer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut next = HashMap::clone(&self.state.load());
        let out = change(&mut next)?;
        self.state.store(Arc::new(next));
        Ok(out)
    }

    /// [`Registry::write`] for changes that cannot fail.
    fn apply(&self, change: impl FnOnce(&mut HashMap<K, Entry<V>>)) {
        let _guard = self
            .writer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut next = HashMap::clone(&self.state.load());
        change(&mut next);
        self.state.store(Arc::new(next));
    }
}
