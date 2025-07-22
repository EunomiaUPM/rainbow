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

use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urn::Urn;

pub struct TransferEventKafkaPayload {
    pub key: Option<Vec<u8>>,
    pub payload: Vec<u8>,
    pub topic: String,
    pub partition: Option<i32>,
    pub offset: Option<i64>,
    pub headers: Option<HashMap<String, Vec<u8>>>,
}

pub enum TransferEventPayloadTypes {
    HTTP(HeaderMap, Body),
    Kafka(TransferEventKafkaPayload),
    NiFi,
}

pub struct TransferEvent {
    pub transfer_event_id: Urn,
    pub payload: TransferEventPayloadTypes,
    pub created_at: chrono::DateTime<chrono::Utc>,
}