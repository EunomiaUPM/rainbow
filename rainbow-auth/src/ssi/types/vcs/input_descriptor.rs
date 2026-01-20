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

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptor {
    pub id: String,
    pub format: InputDescriptorFormat,
    pub constraints: InputDescriptorConstraints
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptorFormat {
    jwt_vc_json: InputDescriptorFormatJWTJson
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptorFormatJWTJson {
    pub alg: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptorConstraints {
    pub fields: Vec<InputDescriptorConstraintsFields>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptorConstraintsFields {
    pub path: Vec<String>,
    pub filter: InputDescriptorConstraintsFieldsFilter
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptorConstraintsFieldsFilter {
    pub r#type: String,
    pub pattern: String
}

impl InputDescriptor {
    pub fn new(vc_type: String) -> Self {
        InputDescriptor {
            id: vc_type.clone(),
            format: InputDescriptorFormat {
                jwt_vc_json: InputDescriptorFormatJWTJson { alg: vec!["RSA".to_string()] }
            },
            constraints: InputDescriptorConstraints {
                fields: vec![InputDescriptorConstraintsFields {
                    path: vec!["$.vc.type".to_string()],
                    filter: InputDescriptorConstraintsFieldsFilter {
                        r#type: "string".to_string(),
                        pattern: vc_type
                    }
                }]
            }
        }
    }
}
