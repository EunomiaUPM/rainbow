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
use anyhow::bail;
use axum::http::HeaderMap;
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use serde_json::Value;
use tracing::error;
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;

pub fn create_opaque_token() -> String {
    let mut bytes = [0u8; 32]; // 256 bits
    rand::thread_rng().fill(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

pub fn extract_gnap_token(headers: HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("GNAP "))
        .map(|token| token.to_string())
}

pub fn trim_4_base(input: &str) -> String {
    let slashes: Vec<usize> = input.match_indices('/').map(|(i, _)| i).collect();

    if slashes.len() < 3 {
        return input.to_string();
    }

    let cut_index = slashes[2];

    input[..cut_index].to_string()
}

pub fn split_did(did: &str) -> (&str, Option<&str>) {
    match did.split_once('#') {
        Some((did_kid, id)) => (did_kid, Some(id)),
        None => (did, None),
    }
}

pub fn trim_path(path: &str) -> String {
    if let Some(pos) = path.rfind('/') {
        let trimmed = &path[..pos];
        trimmed.to_string()
    } else {
        path.to_string()
    }
}

pub fn get_claim(claims: &Value, path: Vec<&str>) -> anyhow::Result<String> {
    let mut node = claims;
    let field = path.last().unwrap_or(&"unknown");
    for key in path.iter() {
        node = match node.get(key) {
            Some(data) => data,
            None => {
                let error = Errors::format_new(BadFormat::Received, format!("Missing field '{}'", key));
                error!("{}", error.log());
                bail!(error)
            }
        };
    }
    let data = match node.as_str() {
        Some(data) => data.to_string(),
        None => {
            let error = Errors::format_new(
                BadFormat::Received,
                format!("Field '{}' not a string", field),
            );
            error!("{}", error.log());
            bail!(error)
        }
    };
    Ok(data)
}