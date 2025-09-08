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

use crate::adv_protocol::interplane::{
    DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneSDPConfigField, DataPlaneSDPRequestField,
    DataPlaneSDPResponseField,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneProvisionRequest {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
    #[serde(rename = "sdpRequest")]
    pub sdp_request: Vec<DataPlaneSDPRequestField>,
    #[serde(rename = "sdpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_config: Option<Vec<DataPlaneSDPConfigField>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneProvisionResponse {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
    #[serde(rename = "sdpResponse")]
    pub sdp_response: Vec<DataPlaneSDPResponseField>,
    #[serde(rename = "sdpRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_request: Option<Vec<DataPlaneSDPRequestField>>,
    #[serde(rename = "sdpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_config: Option<Vec<DataPlaneSDPConfigField>>,
}
