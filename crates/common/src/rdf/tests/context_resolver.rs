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

//! RdfContextResolver and RdfContextCache: cache first, then local copies, with expiry.

use std::sync::Arc;
use std::time::Duration;

use sophia_iri::Iri;
use sophia_jsonld::vocabulary::ArcIri;

use crate::rdf::{RdfContextCache, RdfContextResolver};

const URL: &str = "https://example.org/contexts/test.jsonld";
const DOC: &str = r#"{"@context": {"name": "https://schema.org/name"}}"#;

fn iri(url: &str) -> ArcIri {
    Iri::new_unchecked(Arc::from(url))
}

/// A folder of local copies, unique per test.
fn folder() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rdf-contexts-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A context registered in code resolves without touching the network.
#[tokio::test]
async fn registered_context_resolves_locally() {
    let mut resolver = RdfContextResolver::default();
    resolver.register_asset(URL, DOC).unwrap();
    assert!(resolver.is_local_or_cached(URL).await);
    assert!(resolver.resolve(&iri(URL)).await.is_ok());
}

/// A file named like the URL in the folder resolves, with or without `.jsonld`.
#[tokio::test]
async fn folder_copy_resolves_by_file_name() {
    let dir = folder();
    std::fs::write(dir.join("test.jsonld"), DOC).unwrap();
    let mut resolver = RdfContextResolver::default();
    resolver.set_folder(&dir);
    assert!(resolver.resolve(&iri(URL)).await.is_ok());
    assert!(resolver
        .resolve(&iri("https://example.org/contexts/test"))
        .await
        .is_ok());
}

/// Once resolved, the cache answers first: the folder copy is no longer needed.
#[tokio::test]
async fn the_cache_answers_before_the_folder() {
    let dir = folder();
    std::fs::write(dir.join("test.jsonld"), DOC).unwrap();
    let mut resolver = RdfContextResolver::default();
    resolver.set_folder(&dir);
    resolver.resolve(&iri(URL)).await.unwrap();
    std::fs::remove_file(dir.join("test.jsonld")).unwrap();
    assert!(resolver.resolve(&iri(URL)).await.is_ok());
}

/// An expired entry is gone, so the next resolution goes back to the local copy.
#[tokio::test]
async fn expired_entries_are_not_served() {
    let cache = RdfContextCache::new(Duration::from_millis(30));
    let doc = RdfContextResolver::parse_doc(URL, DOC).unwrap();
    cache.put(URL, doc).await;
    assert!(cache.contains(URL).await);
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert!(!cache.contains(URL).await);
}

/// The default cache keeps contexts for a day.
#[test]
fn default_ttl_is_long() {
    assert_eq!(RdfContextCache::DEFAULT_TTL, Duration::from_secs(86_400));
}
