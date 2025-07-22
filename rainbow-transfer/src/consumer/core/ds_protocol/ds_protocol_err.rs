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

use rainbow_db::transfer_consumer::repo::TransferConsumerRepoErrors;
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum DSProtocolTransferConsumerErrors {
    #[error("Error from database: {0}")]
    DbErr(TransferConsumerRepoErrors),
    #[error("Protocol Error. Transfer process not found")]
    TransferProcessNotFound {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("DataAddress field cannot be null or undefined if dct:format is PUSH type")]
    DataAddressCannotBeNullOnPushError {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Uri and body identifiers do not coincide")]
    UriAndBodyIdentifiersDoNotCoincide,
    #[error("Json malformed: {0}")]
    JsonRejection(String),
}