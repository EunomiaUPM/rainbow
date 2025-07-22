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

use crate::ssi_auth::provider::core::types::Claims;
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

pub fn split_did(did: &str) -> (&str, Option<&str>) {
    match did.split_once('#') {
        Some((didkid, id)) => (didkid, Some(id)),
        None => (did, None),
    }
}

pub fn compare_with_margin(iat: i64, issuance_date: &str, margin_seconds: i64) -> (bool, String) {
    let datetime = match DateTime::from_timestamp(iat, 0) {
        Some(dt) => dt,
        None => return (true, "Invalid iat field".to_string()),
    };

    let parsed_date = match DateTime::parse_from_rfc3339(issuance_date) {
        Ok(dt) => dt,
        Err(_) => {
            return (
                true,
                "IssuanceDate is not with the correct format".to_string(),
            )
        }
    };
    let parsed_date_utc = parsed_date.with_timezone(&Utc);

    if parsed_date_utc > Utc::now() {
        return (true, "Issuance date has not reached yet".to_string());
    }

    if (datetime - parsed_date_utc).num_seconds().abs() > margin_seconds {
        return (true, "IssuanceDate & iat field do not match".to_string());
    }

    (false, "Ignore this".to_string())
}

pub fn create_opaque_token() -> String {
    let mut bytes = [0u8; 32]; // 256 bits
    rand::thread_rng().fill(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

pub fn create_token(sub: String, scopes: String) -> anyhow::Result<String> {
    // TODO
    let exp = generate_exp() as usize;
    let claims = Claims::new(
        sub.clone(),
        "ProvProvider".to_string(),
        format!("ProvProvider-{}", sub),
        scopes,
        exp,
    );

    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret("supersecreto".as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(token: String) -> (bool, String) {
    // TODO

    let mut val = Validation::new(Algorithm::HS512);

    val.required_spec_claims = HashSet::new();
    val.required_spec_claims.insert("jti".to_string());
    val.required_spec_claims.insert("sub".to_string());
    val.required_spec_claims.insert("iss".to_string());
    val.required_spec_claims.insert("aud".to_string());
    val.required_spec_claims.insert("scopes".to_string());
    val.required_spec_claims.insert("nbf".to_string());
    val.required_spec_claims.insert("exp".to_string());

    val.validate_aud = true; // VALIDATE AUDIENCE
    val.validate_exp = true;
    val.validate_nbf = true; // VALIDATE NBF

    let token = match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret("supersecreto".as_ref()),
        &val,
    ) {
        Ok(token) => token,
        Err(e) => return (false, "INVALID TOKEN".to_string()),
    };

    if token.claims.iss != "ProvProvider" {
        return (false, "INVALID TOKEN".to_string());
    }

    if token.claims.aud != format!("ProvProvider-{}", token.claims.sub) {
        return (false, "INVALID TOKEN".to_string());
    }

    (true, token.claims.scopes)
}

fn generate_exp() -> u64 {
    let now = SystemTime::now();
    let ten_minutes_from_now = now + Duration::new(600, 0); // 600 segundos = 10 minutos
    let exp = ten_minutes_from_now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_secs();
    exp
}
