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

//! Registry and MultiRegistry: conflicts, ownership rules, snapshots and ordering.

use super::*;

fn plugin(id: &str) -> Origin {
    Origin::Plugin(PluginRef {
        id: id.into(),
        version: "1.0".into(),
        digest: "sha256:0".into(),
    })
}

fn key(k: &'static str) -> ExtensionKey {
    ExtensionKey::core(k)
}

/// Core keys are uppercase; plugin keys are `wasm:<name>@<version>`.
#[test]
fn extension_key_accepts_core_and_plugin_forms() {
    assert!("BASIC_AUTH".parse::<ExtensionKey>().is_ok());
    assert!("wasm:aws-sigv4@1.2.0"
        .parse::<ExtensionKey>()
        .unwrap()
        .is_plugin());
    for bad in [
        "",
        "basic_auth",
        "wasm:",
        "wasm:Sig@1",
        "wasm:sig",
        "wasm:sig@",
        "HTTP!",
    ] {
        assert!(
            bad.parse::<ExtensionKey>().is_err(),
            "{bad} should be rejected"
        );
    }
}

/// Extension keys travel as plain strings and are validated when read.
#[test]
fn extension_key_round_trips_as_string() {
    let json = serde_json::to_string(&key("OAUTH2")).unwrap();
    assert_eq!(json, "\"OAUTH2\"");
    assert_eq!(
        serde_json::from_str::<ExtensionKey>(&json).unwrap(),
        key("OAUTH2")
    );
    assert!(serde_json::from_str::<ExtensionKey>("\"oauth2\"").is_err());
}

/// A taken key cannot be registered again.
#[test]
fn register_rejects_a_taken_key() {
    let reg: Registry<ExtensionKey, str> = Registry::new("test.point");
    reg.register(key("A"), Arc::from("first"), Origin::Core)
        .unwrap();
    assert!(reg
        .register(key("A"), Arc::from("second"), plugin("p"))
        .is_err());
    assert_eq!(&*reg.get(&key("A")).unwrap(), "first");
}

/// A profile may replace a core entry; a plugin may not.
#[test]
fn replace_respects_ownership() {
    let reg: Registry<ExtensionKey, str> = Registry::new("test.point");
    reg.register(key("A"), Arc::from("core"), Origin::Core)
        .unwrap();
    assert!(reg
        .replace(key("A"), Arc::from("plugin"), plugin("p"))
        .is_err());
    reg.replace(key("A"), Arc::from("profile"), Origin::profile("catena-x"))
        .unwrap();
    assert_eq!(&*reg.get(&key("A")).unwrap(), "profile");
}

/// A missing key is an error that names the extension point.
#[test]
fn require_names_the_point_when_missing() {
    let reg: Registry<ExtensionKey, str> = Registry::new("connector.auth_scheme");
    let err = reg.require(&key("NOPE")).unwrap_err();
    assert!(err.reason().contains("connector.auth_scheme"));
}

/// Core entries cannot be removed, other origins can.
#[test]
fn unregister_keeps_core_entries() {
    let reg: Registry<ExtensionKey, str> = Registry::new("test.point");
    let core = reg
        .register(key("A"), Arc::from("core"), Origin::Core)
        .unwrap();
    let other = reg
        .register(key("B"), Arc::from("plugin"), plugin("p"))
        .unwrap();
    assert!(reg.unregister(core).is_err());
    reg.unregister(other).unwrap();
    assert!(reg.get(&key("B")).is_none());
    assert!(reg.get(&key("A")).is_some());
}

/// Uninstalling an origin removes only what that origin registered.
#[test]
fn unregister_origin_removes_only_that_origin() {
    let reg: Registry<ExtensionKey, str> = Registry::new("test.point");
    reg.register(key("A"), Arc::from("core"), Origin::Core)
        .unwrap();
    reg.register(key("B"), Arc::from("p1"), plugin("p1"))
        .unwrap();
    reg.register(key("C"), Arc::from("p2"), plugin("p2"))
        .unwrap();
    assert_eq!(reg.unregister_origin(&plugin("p1")), 1);
    assert_eq!(reg.unregister_origin(&Origin::Core), 0);
    let mut keys: Vec<_> = reg.entries().into_iter().map(|(k, _)| k).collect();
    keys.sort();
    assert_eq!(keys, vec![key("A"), key("C")]);
}

/// A snapshot keeps the view it was taken with while others write.
#[test]
fn snapshot_is_stable_across_writes() {
    let reg: Registry<ExtensionKey, str> = Registry::new("test.point");
    reg.register(key("A"), Arc::from("a"), Origin::Core)
        .unwrap();
    let before = reg.snapshot();
    reg.register(key("B"), Arc::from("b"), plugin("p")).unwrap();
    assert_eq!(before.len(), 1);
    assert_eq!(reg.snapshot().len(), 2);
}

/// Concurrent writers never lose an entry.
#[test]
fn concurrent_registrations_are_all_kept() {
    let reg: Arc<MultiRegistry<u8, str>> = Arc::new(MultiRegistry::new("test.point"));
    let handles: Vec<_> = (0..8)
        .map(|i| {
            let reg = reg.clone();
            std::thread::spawn(move || {
                for _ in 0..25 {
                    reg.register(i % 2, Arc::from("x"), plugin("p"), 0);
                }
            })
        })
        .collect();
    handles.into_iter().for_each(|h| h.join().unwrap());
    assert_eq!(reg.all(&0).len() + reg.all(&1).len(), 200);
}

/// Entries run by priority, then by registration order, wildcards included.
#[test]
fn multi_registry_orders_by_priority_then_registration() {
    let reg: MultiRegistry<&'static str, str> = MultiRegistry::new("test.point");
    reg.register("k", Arc::from("late"), Origin::Core, 10);
    reg.register("k", Arc::from("first"), Origin::Core, 0);
    reg.register_any(Arc::from("any"), plugin("p"), 5);
    reg.register("k", Arc::from("second"), Origin::profile("x"), 0);
    reg.register("other", Arc::from("not mine"), Origin::Core, 0);
    let order: Vec<String> = reg.all(&"k").iter().map(|e| e.value.to_string()).collect();
    assert_eq!(order, vec!["first", "second", "any", "late"]);
}

/// The multi registry keeps core entries and drops a plugin's on uninstall.
#[test]
fn multi_registry_uninstall_keeps_core() {
    let reg: MultiRegistry<&'static str, str> = MultiRegistry::new("test.point");
    let core = reg.register("k", Arc::from("core"), Origin::Core, 0);
    reg.register("k", Arc::from("p"), plugin("p"), 0);
    reg.register_any(Arc::from("p-any"), plugin("p"), 0);
    assert!(reg.unregister(core).is_err());
    assert_eq!(reg.unregister_origin(&plugin("p")), 2);
    assert_eq!(reg.all(&"k").len(), 1);
}
