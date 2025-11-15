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

use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use anyhow::bail;
use axum::http::HeaderMap;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{TokenData, Validation};
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use tracing::{error, info};

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

pub fn extract_bearer_token(headers: HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
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
                let error = Errors::format_new(BadFormat::Received, &format!("Missing field '{}'", key));
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
                &format!("Field '{}' not a string", field),
            );
            error!("{}", error.log());
            bail!(error)
        }
    };
    Ok(data)
}

pub fn validate_token<T>(token: &str, audience: Option<&str>) -> anyhow::Result<(TokenData<T>, String)>
where
    T: Serialize + DeserializeOwned,
{
    info!("Validating token");
    let header = jsonwebtoken::decode_header(&token)?;
    let kid_str = get_from_opt(&header.kid, "kid")?;
    let (kid, _) = split_did(kid_str.as_str()); // TODO KID_ID
    let alg = header.alg;

    let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
    let jwk: Jwk = serde_json::from_slice(&vec)?;

    let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

    let mut val = Validation::new(alg);

    val.required_spec_claims = HashSet::new();
    val.validate_exp = false;
    val.validate_nbf = true;

    match audience {
        Some(data) => {
            val.validate_aud = true;
            val.set_audience(&[&(data)]);
        }
        None => {
            val.validate_aud = false;
        }
    };

    let token_data = match jsonwebtoken::decode::<T>(&token, &key, &val) {
        Ok(data) => data,
        Err(e) => {
            let error = Errors::security_new(&format!("VPT signature is incorrect -> {}", e.to_string()));
            error!("{}", error.log());
            bail!(error);
        }
    };

    info!("Token signature is correct");
    Ok((token_data, kid.to_string()))
}

pub fn get_from_opt<T>(value: &Option<T>, field_name: &str) -> anyhow::Result<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    match value {
        Some(v) => Ok(v.clone()),
        None => {
            let error = Errors::format_new(
                BadFormat::Received,
                &format!("Missing field: {}", field_name),
            );
            error!("{}", error.log());
            bail!(error);
        }
    }
}

pub fn is_active(iat: u64) -> anyhow::Result<()> {
    let now = Utc::now().timestamp() as u64;
    if now >= iat {
        Ok(())
    } else {
        let error = Errors::forbidden_new("Token is not yet valid");
        error!("{}", error.log());
        bail!(error);
    }
}

pub fn has_expired(exp: u64) -> anyhow::Result<()> {
    let now = Utc::now().timestamp() as u64;
    if now <= exp {
        Ok(())
    } else {
        let error = Errors::forbidden_new("Token has expired");
        error!("{}", error.log());
        bail!(error);
    }
}

pub fn read(path: &str) -> anyhow::Result<String> {
    match fs::read_to_string(&path) {
        Ok(data) => Ok(data),
        Err(e) => {
            let error = Errors::read_new(path.to_string(), e.to_string());
            error!("{}", error);
            bail!(error)
        }
    }
}
