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

//! Where a JSON-LD context comes from: the cache first, then local copies (compiled in or in a
//! folder), and only then the network. Whatever is found is cached.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::http::header::ACCEPT;
use axum::http::{HeaderMap, HeaderValue};
use json_syntax::{Parse, Value as JsonSyntaxValue};
use locspan::Location;
use sophia_iri::Iri;
use sophia_jsonld::vocabulary::ArcIri;
use ymir::services::client::ClientTrait;
use ymir::utils::http_client;

use crate::rdf::context_cache::RdfContextCache;
use crate::rdf::loader::{MetaVal, RdfLoaderError};

/// Resolves context URLs through cache, local copies and the network, in that order.
#[derive(Clone, Debug, Default)]
pub struct RdfContextResolver {
    cache: RdfContextCache,
    /// Contexts registered in code, by URL and by file name.
    assets: Arc<HashMap<String, MetaVal>>,
    /// Folder searched for `<file name>` or `<file name>.jsonld` of the URL.
    folder: Option<PathBuf>,
}

impl RdfContextResolver {
    pub fn new(cache: RdfContextCache) -> Self {
        Self {
            cache,
            ..Self::default()
        }
    }

    /// Registers a context under its URL (and its file name) as a local copy.
    pub fn register_asset(&mut self, url: &str, json_doc: &str) -> Result<(), RdfLoaderError> {
        let iri = if url.contains("://") {
            url.to_string()
        } else {
            format!("file://local/{url}")
        };
        let parsed = Self::parse_doc(&iri, json_doc)?;
        let mut map = (*self.assets).clone();
        map.insert(url.to_string(), parsed.clone());
        if let Some(name) = url.rsplit('/').next().filter(|name| *name != url) {
            map.insert(name.to_string(), parsed);
        }
        self.assets = Arc::new(map);
        Ok(())
    }

    /// Folder of local copies, read on demand; a missing folder is simply skipped.
    pub fn set_folder(&mut self, folder: impl Into<PathBuf>) {
        self.folder = Some(folder.into());
    }

    /// The context for `url`: cached, local, or fetched; local and fetched ones are cached.
    pub async fn resolve(&self, url: &ArcIri) -> Result<MetaVal, RdfLoaderError> {
        let key = url.as_str();
        if let Some(doc) = self.cache.get(key).await {
            return Ok(doc);
        }
        let doc = match self.local(key)? {
            Some(doc) => doc,
            None => self.remote(url).await?,
        };
        self.cache.put(key, doc.clone()).await;
        Ok(doc)
    }

    /// Whether `url` would resolve without going to the network.
    pub async fn is_local_or_cached(&self, url: &str) -> bool {
        self.cache.contains(url).await || matches!(self.local(url), Ok(Some(_)))
    }

    /// Parses a context document, located at `url` for error reporting.
    pub fn parse_doc(url: &str, doc: &str) -> Result<MetaVal, RdfLoaderError> {
        let iri: ArcIri = Iri::new_unchecked(Arc::from(url));
        JsonSyntaxValue::parse_str(doc, |span| Location::new(iri.clone(), span))
            .map_err(|e| RdfLoaderError::Parse(url.to_string(), e.to_string()))
    }

    /// A registered asset by URL or file name, else a file of that name in the folder.
    fn local(&self, url: &str) -> Result<Option<MetaVal>, RdfLoaderError> {
        let name = url.rsplit('/').next().unwrap_or(url);
        if let Some(doc) = self.assets.get(url).or_else(|| self.assets.get(name)) {
            return Ok(Some(doc.clone()));
        }
        let Some(folder) = &self.folder else {
            return Ok(None);
        };
        for candidate in [folder.join(name), folder.join(format!("{name}.jsonld"))] {
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                return Self::parse_doc(url, &text).map(Some);
            }
        }
        Ok(None)
    }

    async fn remote(&self, url: &ArcIri) -> Result<MetaVal, RdfLoaderError> {
        let key = url.as_str();
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/ld+json, application/json"),
        );
        let response = http_client()
            .get(key, Some(headers))
            .await
            .map_err(|e| RdfLoaderError::Network(key.to_string(), e.to_string()))?;
        if !response.status().is_success() {
            return Err(RdfLoaderError::Network(
                key.to_string(),
                format!("HTTP {}", response.status()),
            ));
        }
        let text = response
            .text()
            .await
            .map_err(|e| RdfLoaderError::Network(key.to_string(), e.to_string()))?;
        Self::parse_doc(key, &text)
    }
}
