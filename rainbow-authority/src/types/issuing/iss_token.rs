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

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct IssuingToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
}

impl IssuingToken {
    pub fn new(token: String) -> IssuingToken {
        IssuingToken { access_token: token, token_type: "Bearer".to_string(), expires_in: 600 }
    }
}
