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

//! Registry with an ordered list of entries per key, plus entries that apply to every key.

use super::{Entry, EntryInfo, Origin, Ownership, RegistrationId, RegistryKey};
use arc_swap::ArcSwap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ymir::errors::{Errors, Outcome};

/// Immutable view of a [`MultiRegistry`] at one moment.
pub struct MultiSnapshot<K, V: ?Sized> {
    by_key: HashMap<K, Vec<Entry<V>>>,
    any: Vec<Entry<V>>,
}

impl<K: RegistryKey, V: ?Sized> MultiSnapshot<K, V> {
    /// Entries registered for exactly this key, in order.
    pub fn for_key(&self, key: &K) -> &[Entry<V>] {
        self.by_key.get(key).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Entries registered for every key, in order.
    pub fn wildcard(&self) -> &[Entry<V>] {
        &self.any
    }

    /// Entries for the key and wildcard ones, merged by priority and registration order.
    pub fn all(&self, key: &K) -> Vec<Entry<V>> {
        let mut out: Vec<Entry<V>> = self.for_key(key).iter().chain(&self.any).cloned().collect();
        out.sort_by_key(|e| (e.priority, e.id));
        out
    }

    fn insert_sorted(list: &mut Vec<Entry<V>>, entry: Entry<V>) {
        let at = list.partition_point(|e| (e.priority, e.id) <= (entry.priority, entry.id));
        list.insert(at, entry);
    }
}

impl<K: Clone, V: ?Sized> Clone for MultiSnapshot<K, V> {
    fn clone(&self) -> Self {
        Self {
            by_key: self.by_key.clone(),
            any: self.any.clone(),
        }
    }
}

/// Several entries per key, run in `(priority, registration)` order; nothing is ever replaced.
pub struct MultiRegistry<K, V: ?Sized> {
    point: &'static str,
    state: ArcSwap<MultiSnapshot<K, V>>,
    writer: Mutex<()>,
}

impl<K: RegistryKey, V: ?Sized + Send + Sync + 'static> MultiRegistry<K, V> {
    pub fn new(point: &'static str) -> Self {
        Self {
            point,
            state: ArcSwap::from_pointee(MultiSnapshot {
                by_key: HashMap::new(),
                any: Vec::new(),
            }),
            writer: Mutex::new(()),
        }
    }

    pub fn point(&self) -> &'static str {
        self.point
    }

    /// Appends an entry for `key`; lower priorities run first.
    pub fn register(&self, key: K, value: Arc<V>, origin: Origin, priority: i32) -> RegistrationId {
        let entry = Entry::new(value, origin, priority);
        let id = entry.id;
        self.apply(|s| {
            MultiSnapshot::<K, V>::insert_sorted(s.by_key.entry(key).or_default(), entry)
        });
        id
    }

    /// Appends an entry that applies to every key.
    pub fn register_any(&self, value: Arc<V>, origin: Origin, priority: i32) -> RegistrationId {
        let entry = Entry::new(value, origin, priority);
        let id = entry.id;
        self.apply(|s| MultiSnapshot::<K, V>::insert_sorted(&mut s.any, entry));
        id
    }

    /// Removes one registration; core entries are permanent.
    pub fn unregister(&self, id: RegistrationId) -> Outcome<()> {
        self.write(|s| {
            let origin = s
                .by_key
                .values()
                .flatten()
                .chain(&s.any)
                .find(|e| e.id == id)
                .map(|e| e.origin.clone());
            match origin {
                None => Err(Errors::missing_resource(
                    format!("{id:?}"),
                    format!("{}: no registration {id:?}", self.point),
                    None,
                )),
                Some(Origin::Core) => Err(Ownership::core_is_permanent(self.point, id)),
                Some(_) => {
                    s.by_key.values_mut().for_each(|l| l.retain(|e| e.id != id));
                    s.any.retain(|e| e.id != id);
                    Ok(())
                }
            }
        })
    }

    /// Removes every entry registered by `origin`; returns how many went.
    pub fn unregister_origin(&self, origin: &Origin) -> usize {
        if *origin == Origin::Core {
            return 0;
        }
        let mut removed = 0;
        self.apply(|s| {
            let before = s.any.len() + s.by_key.values().map(Vec::len).sum::<usize>();
            s.by_key
                .values_mut()
                .for_each(|l| l.retain(|e| e.origin != *origin));
            s.by_key.retain(|_, l| !l.is_empty());
            s.any.retain(|e| e.origin != *origin);
            removed = before - s.any.len() - s.by_key.values().map(Vec::len).sum::<usize>();
        });
        removed
    }

    /// Entries for the key and wildcard ones, merged in run order.
    pub fn all(&self, key: &K) -> Vec<Entry<V>> {
        self.state.load().all(key)
    }

    /// The whole registry as it is now; later writes do not change it.
    pub fn snapshot(&self) -> Arc<MultiSnapshot<K, V>> {
        self.state.load_full()
    }

    /// Keyed entries as `(Some(key), info)` and wildcard ones as `(None, info)`.
    pub fn entries(&self) -> Vec<(Option<K>, EntryInfo)> {
        let s = self.state.load();
        s.by_key
            .iter()
            .flat_map(|(k, l)| l.iter().map(move |e| (Some(k.clone()), e.info())))
            .chain(s.any.iter().map(|e| (None, e.info())))
            .collect()
    }

    /// Serialises writers, applies `change` to a copy and publishes it on success.
    fn write<T>(&self, change: impl FnOnce(&mut MultiSnapshot<K, V>) -> Outcome<T>) -> Outcome<T> {
        let _guard = self
            .writer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut next = MultiSnapshot::clone(&self.state.load());
        let out = change(&mut next)?;
        self.state.store(Arc::new(next));
        Ok(out)
    }

    /// [`MultiRegistry::write`] for changes that cannot fail.
    fn apply(&self, change: impl FnOnce(&mut MultiSnapshot<K, V>)) {
        let _guard = self
            .writer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut next = MultiSnapshot::clone(&self.state.load());
        change(&mut next);
        self.state.store(Arc::new(next));
    }
}
