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

use crate::protocol::context_field::ContextField;
use crate::protocol::contract::ContractNegotiationMessages;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct ContractErrorMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    pub provider_pid: Option<String>,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Option<String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
}

impl Default for ContractErrorMessage {
    fn default() -> Self {
        ContractErrorMessage {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractNegotiationError,
            provider_pid: None,
            consumer_pid: None,
            code: None,
            reason: None,
        }
    }
}

impl IntoResponse for ContractErrorMessage {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}