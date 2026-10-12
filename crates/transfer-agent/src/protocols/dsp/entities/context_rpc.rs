/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! An outbound transfer message: the eunomia RPC body a local user sends, and everything
//! resolved to send it.

use std::str::FromStr;

use common::dsp_common::data_address::DataAddress;
use common::oauth::Owner;
use common::rdf::{RdfMessage, RdfProtocolTrait};
use serde::Serialize;
use serde_json::{json, Value};
use urn::Urn;
use ymir::errors::Outcome;

use crate::entities::protocol::{TransferDirection, TransferRole};
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::context_common::TransferContextConnectorRole;
use crate::protocols::dsp::entities::data_address::DataAddressDto;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::rdf_extractor_dsp::DspTransfer;
use crate::protocols::dsp::entities::rdf_extractor_rpc::EunomiaTransfer;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::facades::dataplane_facade::DataplaneHookContext;

/// RPC body for any outbound transfer message; which fields matter depends on the message.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRpcBody {
    /// The provider's participant id, for a request.
    pub associated_agent_peer: Option<String>,
    pub agreement_id: Option<String>,
    pub format: Option<String>,
    /// The sink a push transfer delivers to, for a request.
    pub data_address: Option<DataAddressDto>,
    /// The provider's DSP base, for a request.
    pub provider_address: Option<String>,
    /// Our DSP base as the provider will call it back; defaults to this agent's own.
    pub callback_address: Option<String>,
    pub consumer_pid: Option<String>,
    pub provider_pid: Option<String>,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl TransferRpcBody {
    /// The body as a eunomia JSON-LD message of the RPC that sends `message`.
    pub fn to_json_ld(&self, message: &TransferDSPMessageType) -> Value {
        let mut json = json!({
            "@context": EunomiaTransfer::context(),
            "@type": EunomiaTransfer::compact_type(message),
        });
        if let (Value::Object(object), Ok(Value::Object(members))) =
            (&mut json, serde_json::to_value(self))
        {
            object.extend(members.into_iter().filter(|(_, v)| !v.is_null()));
        }
        json
    }
}

/// The message to send, fixed by the route, its body and the JSON-LD it was read from.
#[derive(Debug, Clone)]
pub struct TransferRpcCommand {
    pub message: TransferDSPMessageType,
    pub body: TransferRpcBody,
    pub json: Value,
}

impl TransferRpcCommand {
    /// A command built in process, such as the TCK's, with its JSON-LD rendered from the body.
    pub fn new(message: TransferDSPMessageType, body: TransferRpcBody) -> Self {
        let json = body.to_json_ld(&message);
        Self {
            message,
            body,
            json,
        }
    }

    /// Reads a eunomia RPC body; its `@type` must be the RPC of the route's `message`.
    pub async fn read(message: &TransferDSPMessageType, json: Value) -> Outcome<Self> {
        let read = RdfMessage::<EunomiaTransfer>::read_as(message, json).await?;
        Ok(Self {
            message: read.message,
            body: read.fields,
            json: read.json,
        })
    }

    /// The pid of an existing process the command refers to, ours or the peer's.
    pub fn pid(&self) -> Option<&str> {
        self.body
            .provider_pid
            .as_deref()
            .or(self.body.consumer_pid.as_deref())
    }
}

/// The command plus everything resolved to send it: process, role, peer and the message.
pub struct TransferRpcFacts {
    pub command: TransferRpcCommand,
    pub process: Option<TransferProcess>,
    pub process_id: Urn,
    pub role: TransferRole,
    pub transfer_direction: TransferDirection,
    pub owner: Owner,
    pub peer_id: String,
    pub target_url: String,
    pub outgoing: RdfMessage<DspTransfer>,
    pub is_restart: bool,
}

impl TransferRpcFacts {
    pub fn current_state(&self) -> Option<TransferDSPState> {
        self.process
            .as_ref()
            .and_then(|p| TransferDSPState::from_str(p.state().0.as_str()).ok())
    }

    pub fn hooks(&self, data_address: Option<DataAddress>) -> DataplaneHookContext {
        DataplaneHookContext {
            process_id: self.process_id.clone(),
            owner: self.owner.clone(),
            role: self.role,
            transfer_direction: self.transfer_direction,
            connector_instance: TransferContextConnectorRole::ConsumerNotHavingConnector,
            resolved_data_address: data_address.clone(),
            data_address,
            is_restart: self.is_restart,
        }
    }
}
