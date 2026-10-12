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

//! An inbound DSP transfer message, read off the request through RDF, and the facts the
//! loader resolves for it.

use std::collections::HashMap;
use std::str::FromStr;

use axum::extract::{FromRequestParts, Path, Request};
use common::dsp_common::odrl::OdrlAgreement;
use common::facades::grants_facade::VerifiedPeer;
use common::flow::{IdempotencyKey, IdempotentFactsTrait};
use common::oauth::Owner;
use common::rdf::RdfMessage;
use sha2::{Digest, Sha256};
use urn::Urn;
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::entities::protocol::{ProtocolId, TransferDirection, TransferRole};
use crate::protocols::dsp::entities::context_common::{
    TransferContextConnectorRole, TransferContextProcessSlot,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::rdf_extractor_dsp::DspTransfer;
use crate::protocols::dsp::facades::dataplane_facade::DataplaneHookContext;

/// An inbound message as the flow takes it: who sent it, where, and what it says.
#[derive(Debug)]
pub struct TransferDspInput {
    pub peer: VerifiedPeer,
    /// The message type the route handles; the body's own is `body.message`.
    pub route: TransferDSPMessageType,
    /// The pid in the route, on the routes that carry one.
    pub path_id: Option<String>,
    pub body: RdfMessage<DspTransfer>,
    /// Derived from protocol identity, with the peer's `Idempotency-Key` folded in.
    pub idempotency_key: IdempotencyKey,
}

impl TransferDspInput {
    /// Cap on inbound bodies: control-plane messages are tiny, this only bounds a hostile peer.
    const MAX_BODY_BYTES: usize = 1024 * 1024;

    /// Reads the request: the peer the auth middleware resolved, the route pid, the
    /// idempotency header and the body through RDF.
    pub async fn read(
        version: &ProtocolId,
        route: &TransferDSPMessageType,
        request: Request,
    ) -> Outcome<Self> {
        let (mut parts, body) = request.into_parts();
        let peer = parts.extensions.get::<VerifiedPeer>().cloned().ok_or_else(|| {
            Errors::crazy(
                "auth middleware did not resolve the peer (VerifiedPeer missing)",
                None,
            )
        })?;
        let bytes = axum::body::to_bytes(body, Self::MAX_BODY_BYTES)
            .await
            .map_err(|_| {
                Errors::format(BadFormat::Received, "failed to read request body", None)
            })?;
        let supplied_key = parts
            .headers
            .get("idempotency-key")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let request_path = parts.uri.path().to_string();
        let path_id = Path::<HashMap<String, String>>::from_request_parts(&mut parts, &())
            .await
            .ok()
            .and_then(|Path(params)| params.get("id").cloned());
        let json: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| {
            Errors::format(BadFormat::Received, format!("body is not JSON: {e}"), None)
        })?;
        let body = RdfMessage::<DspTransfer>::read(json).await?;
        let idempotency_key =
            Self::derive_key(&peer, &request_path, version, &body, supplied_key.as_deref());
        Ok(Self {
            peer,
            route: route.clone(),
            path_id,
            body,
            idempotency_key,
        })
    }

    /// Participant, route, version, type, pids and the peer's header, length-prefixed so no
    /// pid can imitate another tuple; pids are unique per peer, so the participant is in.
    fn derive_key(
        peer: &VerifiedPeer,
        request_path: &str,
        version: &ProtocolId,
        body: &RdfMessage<DspTransfer>,
        supplied_key: Option<&str>,
    ) -> IdempotencyKey {
        let mut hasher = Sha256::new();
        let mut field = |bytes: &[u8]| {
            hasher.update((bytes.len() as u64).to_be_bytes());
            hasher.update(bytes);
        };
        field(peer.participant_id.as_bytes());
        field(request_path.as_bytes());
        // A nested router strips the mount prefix, so the path alone cannot tell versions apart.
        field(version.to_string().as_bytes());
        field(body.message.to_string().as_bytes());
        field(body.fields.consumer_pid.as_deref().unwrap_or_default().as_bytes());
        field(body.fields.provider_pid.as_deref().unwrap_or_default().as_bytes());
        field(supplied_key.unwrap_or_default().as_bytes());
        IdempotencyKey::new(format!("dsp-{}", hex::encode(hasher.finalize())))
    }
}

/// An inbound message with the process, agreement, role and connector it refers to.
#[derive(Debug)]
pub struct TransferDspFacts {
    pub input: TransferDspInput,
    pub process: TransferContextProcessSlot,
    pub agreement: OdrlAgreement,
    pub role: TransferRole,
    pub transfer_direction: TransferDirection,
    pub connector_instance: TransferContextConnectorRole,
    pub is_restart: bool,
}

impl TransferDspFacts {
    /// Owner of the existing process, or of a new one a peer opens: seen as its grant says.
    pub fn owner(&self) -> Owner {
        match &self.process {
            TransferContextProcessSlot::Existing(p) => p.owner().clone(),
            TransferContextProcessSlot::New { .. } => self.input.peer.owner(),
        }
    }

    /// What the dataplane hooks need from this message and the process it refers to.
    pub fn dataplane_hooks(&self) -> Outcome<DataplaneHookContext> {
        Ok(DataplaneHookContext {
            process_id: self.process_urn()?,
            owner: self.owner(),
            role: self.role,
            transfer_direction: self.transfer_direction,
            connector_instance: self.connector_instance.clone(),
            data_address: self.input.body.fields.data_address.clone(),
            resolved_data_address: None,
            is_restart: self.is_restart,
        })
    }

    /// Id of the existing process, or the consumer pid of a new one.
    pub fn process_urn(&self) -> Outcome<Urn> {
        match &self.process {
            TransferContextProcessSlot::Existing(p) => Ok(p.id().as_urn().clone()),
            TransferContextProcessSlot::New { consumer_pid } => Urn::from_str(consumer_pid)
                .map_err(|_| Errors::crazy("invalid consumer_pid urn for a new process", None)),
        }
    }
}

impl IdempotentFactsTrait for TransferDspFacts {
    fn idempotency_key(&self) -> &IdempotencyKey {
        &self.input.idempotency_key
    }

    fn canonical_hash(&self) -> &[u8; 32] {
        &self.input.body.canonical_hash
    }

    fn process_version(&self) -> Option<u64> {
        match &self.process {
            TransferContextProcessSlot::Existing(p) => Some(p.version()),
            TransferContextProcessSlot::New { .. } => None,
        }
    }
}
