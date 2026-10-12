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

//! Typed registries that accept entries at runtime and remember who registered each one.
//!
//! An extension point owns one registry: [`Registry`] holds one implementation per key (an
//! auth scheme, a binding), [`MultiRegistry`] an ordered list per key (validation rules,
//! hooks). Reads load an immutable snapshot without locking; writes copy the map and swap it,
//! so a request keeps the view it started with even if a plugin is installed meanwhile.
//!
//! ```rust,ignore
//! let schemes: Registry<ExtensionKey, dyn AuthSchemeTrait> = Registry::new("connector.auth_scheme");
//! schemes.register(ExtensionKey::core("OAUTH2"), Arc::new(OAuth2Scheme), Origin::Core)?;
//! let scheme = schemes.require(&spec.kind)?;
//! ```
//!
//! Registering a taken key is an error; overriding is an explicit [`Registry::replace`]. Core
//! entries can be replaced by a profile but never removed, and a plugin can touch neither.

mod multi;
mod single;

pub use multi::{MultiRegistry, MultiSnapshot};
pub use single::{Registry, RegistrySnapshot};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use ymir::errors::{BadFormat, Errors};

/// Anything usable as a registry key: closed enums, `()`, or an [`ExtensionKey`].
pub trait RegistryKey: Eq + Hash + Clone + Debug + Send + Sync + 'static {}

impl<T> RegistryKey for T where T: Eq + Hash + Clone + Debug + Send + Sync + 'static {}

/// Open key for extensible kinds: `BASIC_AUTH` for core entries, `wasm:sigv4@1.0` for plugins.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionKey(Cow<'static, str>);

impl ExtensionKey {
    /// Key of a built-in entry; the literal must be `[A-Z0-9_]+`.
    pub fn core(key: &'static str) -> Self {
        debug_assert!(Self::is_core_key(key), "invalid core extension key: {key}");
        Self(Cow::Borrowed(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the key names a plugin rather than a built-in entry.
    pub fn is_plugin(&self) -> bool {
        self.0.starts_with("wasm:")
    }

    fn is_core_key(key: &str) -> bool {
        !key.is_empty()
            && key
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    }

    /// `wasm:<name>@<version>`, name in lowercase kebab case.
    fn is_plugin_key(key: &str) -> bool {
        let Some((name, version)) = key
            .strip_prefix("wasm:")
            .and_then(|rest| rest.split_once('@'))
        else {
            return false;
        };
        !name.is_empty()
            && name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && !version.is_empty()
            && version
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
    }
}

impl FromStr for ExtensionKey {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_core_key(s) || Self::is_plugin_key(s) {
            Ok(Self(Cow::Owned(s.to_string())))
        } else {
            Err(Errors::format(
                BadFormat::Received,
                format!("'{s}' is not an extension key (CORE_KEY or wasm:name@version)"),
                None,
            ))
        }
    }
}

impl Display for ExtensionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Debug for ExtensionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for ExtensionKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ExtensionKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        raw.parse().map_err(serde::de::Error::custom)
    }
}

/// Who registered an entry; decides what may replace or remove it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Origin {
    Core,
    Profile { name: Cow<'static, str> },
    Plugin(PluginRef),
}

impl Origin {
    /// Origin of the entries a named profile registers.
    pub fn profile(name: impl Into<Cow<'static, str>>) -> Self {
        Self::Profile { name: name.into() }
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Origin::Core => f.write_str("core"),
            Origin::Profile { name } => write!(f, "profile:{name}"),
            Origin::Plugin(p) => write!(f, "plugin:{}@{}", p.id, p.version),
        }
    }
}

/// Installed plugin an entry comes from; the digest pins the exact module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PluginRef {
    pub id: String,
    pub version: String,
    pub digest: String,
}

/// Handle to one registration, unique across every registry of the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct RegistrationId(u64);

static NEXT_REGISTRATION: AtomicU64 = AtomicU64::new(1);

impl RegistrationId {
    fn next() -> Self {
        Self(NEXT_REGISTRATION.fetch_add(1, Ordering::Relaxed))
    }
}

/// One registered implementation with its bookkeeping.
#[derive(Debug)]
pub struct Entry<V: ?Sized> {
    pub id: RegistrationId,
    pub origin: Origin,
    pub priority: i32,
    pub value: Arc<V>,
}

impl<V: ?Sized> Clone for Entry<V> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            origin: self.origin.clone(),
            priority: self.priority,
            value: self.value.clone(),
        }
    }
}

impl<V: ?Sized> Entry<V> {
    fn new(value: Arc<V>, origin: Origin, priority: i32) -> Self {
        Self {
            id: RegistrationId::next(),
            origin,
            priority,
            value,
        }
    }

    /// The entry without its value, for listings.
    pub fn info(&self) -> EntryInfo {
        EntryInfo {
            id: self.id,
            origin: self.origin.clone(),
            priority: self.priority,
        }
    }
}

/// What a listing shows about an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EntryInfo {
    pub id: RegistrationId,
    pub origin: Origin,
    pub priority: i32,
}

/// Rules on who may override or remove an entry, shared by both registry kinds.
struct Ownership;

impl Ownership {
    /// A plugin never overrides a core entry; profiles and core may.
    fn may_replace(existing: &Origin, incoming: &Origin) -> bool {
        !(matches!(existing, Origin::Core) && matches!(incoming, Origin::Plugin(_)))
    }

    fn core_is_permanent(point: &str, id: RegistrationId) -> Errors {
        Errors::forbidden(
            format!("{point}: registration {id:?} is core and cannot be removed"),
            None,
        )
    }
}

#[cfg(test)]
mod tests;
