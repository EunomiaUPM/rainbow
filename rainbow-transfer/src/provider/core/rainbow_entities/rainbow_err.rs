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

use axum::extract::rejection::JsonRejection;
use rainbow_db::transfer_provider::repo::TransferProviderRepoErrors;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum RainbowTransferProviderErrors {
    #[error("Error from database: {0}")]
    DbErr(TransferProviderRepoErrors),
    #[error("Transfer provider process not found")]
    ProcessNotFound {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Transfer provider message not found")]
    MessageNotFound {
        transfer_id: Option<Urn>,
        message_id: Option<Urn>,
    },
    #[error("Error by parsing. {error}")]
    UUIDParseError {
        provider_pid: Option<String>,
        consumer_pid: Option<String>,
        error: String,
    },
    #[error("Not Checked Error. {error}")]
    NotCheckedError {
        provider_pid: Option<String>,
        consumer_pid: Option<String>,
        error: String,
    },
    #[error("Error from deserializing JSON: {0}")]
    JsonRejection(JsonRejection),
    #[error("Error from deserializing JSON: {0}")]
    ValidationError(String),
    #[error("Error from deserializing path. {0}")]
    UrnUuidSchema(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RainbowTransferProviderOut {
    pub error: RainbowTransferProviderOutDetail,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RainbowTransferProviderOutDetail {
    pub code: String,
    pub title: String,
    pub message: String,
}

impl RainbowTransferProviderOut {
    pub fn new(code: String, title: String, message: String) -> Self {
        RainbowTransferProviderOut { error: RainbowTransferProviderOutDetail { code, title, message } }
    }
}
