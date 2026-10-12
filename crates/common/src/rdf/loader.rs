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

//! JSON-LD document loader: contexts come from a long-lived cache, local copies, or the network.

use std::path::Path;

use json_syntax::Value as JsonSyntaxValue;
use locspan::{Location, Meta, Span};
use sophia_jsonld::json_ld::future::{BoxFuture, FutureExt};
use sophia_jsonld::json_ld::{Loader, LoadingResult, RemoteDocument};
use sophia_jsonld::vocabulary::ArcIri;

use crate::rdf::context_cache::RdfContextCache;
use crate::rdf::context_resolver::RdfContextResolver;

/// JSON value as parsed by the JSON-LD loader, with source locations.
pub type JsonVal = JsonSyntaxValue<Location<ArcIri, Span>>;
/// `JsonVal` with its location metadata.
pub type MetaVal = Meta<JsonVal, Location<ArcIri, Span>>;

/// Errors that may occur when resolving and loading JSON-LD context documents.
#[derive(thiserror::Error, Debug)]
pub enum RdfLoaderError {
    #[error("Document not found: {0}")]
    NotFound(String),
    #[error("Failed to fetch remote document '{0}': {1}")]
    Network(String, String),
    #[error("Failed to parse JSON-LD document from '{0}': {1}")]
    Parse(String, String),
}

/// JSON-LD document loader over a [`RdfContextResolver`]: cache, local copies, network.
#[derive(Clone, Debug, Default)]
pub struct RdfContextLoader {
    resolver: RdfContextResolver,
}

impl RdfContextLoader {
    /// Loader with an empty long-lived cache and no local copies yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loader whose resolver uses `cache` (to share one cache, or set another TTL).
    pub fn with_cache(cache: RdfContextCache) -> Self {
        Self {
            resolver: RdfContextResolver::new(cache),
        }
    }

    /// Registers a context document on an owned loader (builder style).
    pub fn with_asset(mut self, url: &str, json_doc: &str) -> Result<Self, RdfLoaderError> {
        self.register_asset(url, json_doc)?;
        Ok(self)
    }

    /// Registers a context document under its URL and file name as a local copy.
    pub fn register_asset(&mut self, url: &str, json_doc: &str) -> Result<(), RdfLoaderError> {
        self.resolver.register_asset(url, json_doc)
    }

    /// Looks for local copies of contexts in `dir_path`, read when first needed.
    pub fn preload_dir<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<(), RdfLoaderError> {
        self.resolver.set_folder(dir_path.as_ref());
        Ok(())
    }

    /// Whether a context resolves without going to the network.
    pub async fn is_cached(&self, url: &str) -> bool {
        self.resolver.is_local_or_cached(url).await
    }

    /// Parses string content into a typed JSON syntax tree bound to the URL location.
    pub fn parse_doc(url_str: &str, doc_str: &str) -> Result<MetaVal, RdfLoaderError> {
        RdfContextResolver::parse_doc(url_str, doc_str)
    }

    fn to_remote_document(
        url: ArcIri,
        parsed: MetaVal,
    ) -> RemoteDocument<ArcIri, Location<ArcIri, Span>, JsonVal> {
        RemoteDocument::new(
            Some(url),
            Some("application/ld+json".parse().unwrap()),
            parsed,
        )
    }
}

impl Loader<ArcIri, Location<ArcIri, Span>> for RdfContextLoader {
    type Output = JsonVal;
    type Error = RdfLoaderError;

    fn load_with<'a>(
        &'a mut self,
        _vocabulary: &'a mut (impl Sync + Send + rdf_types::IriVocabularyMut<Iri = ArcIri>),
        url: ArcIri,
    ) -> BoxFuture<'a, LoadingResult<ArcIri, Location<ArcIri, Span>, Self::Output, Self::Error>>
    where
        ArcIri: 'a,
    {
        let resolver = self.resolver.clone();
        async move {
            let doc = resolver.resolve(&url).await?;
            Ok(Self::to_remote_document(url, doc))
        }
        .boxed()
    }
}
