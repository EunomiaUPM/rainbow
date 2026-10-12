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

//! Inbound DSP transfer messages as a flow: the runner validates, this loads and applies.

pub mod codec;
pub mod handlers;
pub mod idempotency_guard;

use std::sync::Arc;

use common::facades::grants_facade::VerifiedPeer;
use common::flow::{FlowCx, FlowTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ymir::errors::Outcome;

use crate::entities::ids::IdempotencyKey;
use crate::entities::protocol::TransferRole;
use crate::protocols::dsp::entities::ack::TransferProcessAck;
use crate::protocols::dsp::entities::context_dsp::{
    TransferDSPContextDomain, TransferDSPContextTyped,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::flow::handlers::request::RequestHandler;
use crate::protocols::dsp::flow::handlers::transition::TransitionHandler;
use crate::protocols::dsp::flow::handlers::TransferHandlerDeps;
use crate::protocols::dsp::services::dsp_domain_loader::dsp_domain_loader::DspDomainLoader;

/// What applying a message produced: the ACK, and what idempotency needs to replay it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDspReply {
    pub ack: TransferProcessAck,
    /// A request opened the process (201); any other message moved it (200).
    pub created: bool,
    pub process_version: u64,
    pub idempotency_key: IdempotencyKey,
    /// What the process runs under and our side of it, for whoever reacts to the change.
    pub agreement_id: String,
    pub role: TransferRole,
}

/// The five inbound transfer messages; `apply` picks the handler by message type.
pub struct TransferDspFlow {
    loader: Arc<DspDomainLoader>,
    deps: TransferHandlerDeps,
}

impl TransferDspFlow {
    pub fn new(loader: Arc<DspDomainLoader>, deps: TransferHandlerDeps) -> Self {
        Self { loader, deps }
    }
}

#[async_trait::async_trait]
impl FlowTrait for TransferDspFlow {
    type Key = TransferDSPMessageType;
    type Caller = VerifiedPeer;
    type Input = TransferDSPContextTyped;
    type Facts = TransferDSPContextDomain;
    type Output = TransferDspReply;

    /// The route's message type; the body's own type is checked against it by a shape rule.
    fn key(&self, input: &TransferDSPContextTyped) -> TransferDSPMessageType {
        input.rdf.parsed.dsp_message_type.clone()
    }

    async fn load(
        &self,
        _cx: &FlowCx<'_, VerifiedPeer>,
        input: TransferDSPContextTyped,
    ) -> Outcome<TransferDSPContextDomain> {
        self.loader.load(input).await
    }

    async fn apply(
        &self,
        _cx: &FlowCx<'_, VerifiedPeer>,
        facts: TransferDSPContextDomain,
        _reply: Option<Value>,
    ) -> Outcome<TransferDspReply> {
        match facts.typed.message {
            TransferDSPMessageType::TransferRequestMessage => {
                RequestHandler::handle(&self.deps, facts).await
            }
            _ => TransitionHandler::handle(&self.deps, facts).await,
        }
    }
}
