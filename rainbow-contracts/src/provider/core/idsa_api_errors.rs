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
use rainbow_db::contracts_provider::repo::CnErrors;
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum IdsaCNError {
    #[error("Error from database: {0}")]
    DbErr(CnErrors),
    #[error("Negotiation process not found")]
    ProcessNotFound {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
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
    #[error("Operation not allowed: {error}")]
    NotAllowed {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
        error: String,
    },
}