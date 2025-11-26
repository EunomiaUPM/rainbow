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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthJwtClaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub jti: String,
    pub iss: String,
    pub aud: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: String, // Optional. Subject (whom token refers to)
    pub iss: String, // Optional. Issuer
    pub aud: String, // Optional. Audience
    pub scopes: String,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub nbf: usize, // Optional. not before
}
