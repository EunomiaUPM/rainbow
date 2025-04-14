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
use rainbow_db::contracts_consumer::repo::CnErrors;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum CnErrorConsumer {
    #[error("{entity} with id {} not found", id.as_str())]
    NotFound { id: Urn, entity: String },
    #[error("Last {entity} with id {} not found", id.as_str())]
    LastNotFound { id: Urn, entity: String },
    #[error("{entity} associated with CN Process provider_pid {} not found", provider_id.as_str())]
    ProviderNotFound { provider_id: Urn, entity: String },
    #[error("{entity} associated with CN Process consumer_pid {} not found", consumer_id.as_str())]
    ConsumerNotFound { consumer_id: Urn, entity: String },
    #[error("{entity} associated with CN Process cn_process_id {} not found", process_id.as_str())]
    ProcessNotFound { process_id: Urn, entity: String },
    #[error("{entity} associated with CN Message cn_message_id {} not found", message_id.as_str())]
    MessageNotFound { message_id: Urn, entity: String },
    #[error("Error from database: {0}")]
    DbErr(CnErrors),
    #[error("Error from deserializing JSON: {0}")]
    JsonRejection(JsonRejection),
    #[error("Error from deserializing path. {0}")]
    UrnUuidSchema(String),
    #[error("{which} Participant id {} not found", participant_id.as_str()
    )]
    ParticipantNotFound { participant_id: Urn, which: String, entity: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CnErrorConsumerErrorOut {
    pub error: CnErrorConsumerOutDetail,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CnErrorConsumerOutDetail {
    pub code: String,
    pub title: String,
    pub message: String,
}

impl CnErrorConsumerErrorOut {
    pub fn new(code: String, title: String, message: String) -> Self {
        CnErrorConsumerErrorOut { error: CnErrorConsumerOutDetail { code, title, message } }
    }
}
