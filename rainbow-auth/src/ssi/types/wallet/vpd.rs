/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Vpd {
    pub id: String,
    pub input_descriptors: Vec<InputDescriptor>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputDescriptor {
    pub id: String,
    pub constraints: Option<Constraints>,
    pub format: Option<Format>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Constraints {
    pub fields: Vec<Field>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Field {
    pub path: Vec<String>,
    pub filter: Option<Filter>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Filter {
    pub pattern: String,
    #[serde(rename = "type")]
    pub ty: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Format {
    pub jwt_vc_json: Option<JwtVcJson>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JwtVcJson {
    alg: Vec<String>
}
