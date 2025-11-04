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

use crate::err::transfer_err::TransferErrorType;
use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::TransferMessageTypes;
use axum::body::to_bytes;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferError {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<String>,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<String>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "reason")]
    pub reason: Vec<String>,
}

impl Default for TransferError {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferError.to_string(),
            provider_pid: None,
            consumer_pid: None,
            code: "".to_string(),
            reason: vec![],
        }
    }
}

impl TransferError {
    pub async fn from_async(value: TransferErrorType) -> Self {
        let response = value.into_response();
        let response_data = to_bytes(response.into_parts().1, 2048).await.unwrap();
        serde_json::from_slice::<TransferError>(&response_data).unwrap()
    }
}
