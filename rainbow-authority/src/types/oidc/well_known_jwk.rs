/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WellKnownJwks {
    pub kty: String,
    pub n: String,
    pub e: String,
    pub d: String,
    pub kid: String,
}

impl WellKnownJwks {
    pub fn new(key: RsaPrivateKey) -> WellKnownJwks {
        WellKnownJwks {
            kty: "RSA".to_string(),
            n: URL_SAFE_NO_PAD.encode(key.n().to_bytes_be()),
            e: URL_SAFE_NO_PAD.encode(key.e().to_bytes_be()),
            d: URL_SAFE_NO_PAD.encode(key.d().to_bytes_be()),
            kid: "0".to_string(),
        }
    }
}
