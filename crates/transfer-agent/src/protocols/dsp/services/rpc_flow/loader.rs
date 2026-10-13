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

//! The outbound flow's load step: the process, peer and target of a message, and the DSP
//! message itself, built in JSON-LD and read back through RDF.

use std::sync::Arc;

use common::dsp_common::data_address::DataAddress;
use common::dsp_common::DspActor;
use common::oauth::{Owner, OwnerScope, UserInfo};
use common::rdf::RdfMessage;
use serde_json::{json, Map, Value};
use url::Url;
use ymir::errors::{Errors, Outcome};

use crate::data::repo::transfer_process::TransferProcessRepoTrait;
use crate::entities::ids::TransferProcessId;
use crate::entities::protocol::{TransferDirection, TransferRole};
use crate::protocols::dsp::entities::context_common::DIRECTION_PROPERTY;
use crate::protocols::dsp::entities::context_rpc::{TransferRpcDomain, TransferRpcTyped};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::rdf_extractor_dsp::DspTransfer;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::facades::dataplane_facade::DataPlaneFacadeTrait;

/// Resolves what an outbound message needs before it can be checked and sent.
pub struct TransferRpcLoader {
    process_repo: Arc<dyn TransferProcessRepoTrait>,
    dataplane: Arc<dyn DataPlaneFacadeTrait>,
    /// This agent's DSP base, the default callback a provider answers to.
    own_dsp_base: String,
}

impl TransferRpcLoader {
    pub fn new(
        process_repo: Arc<dyn TransferProcessRepoTrait>,
        dataplane: Arc<dyn DataPlaneFacadeTrait>,
        own_dsp_base: String,
    ) -> Self {
        Self {
            process_repo,
            dataplane,
            own_dsp_base,
        }
    }

    /// Resolves the domain stage of a typed RPC body.
    pub async fn load(&self, caller: &UserInfo, typed: TransferRpcTyped) -> Outcome<TransferRpcDomain> {
        match typed.message {
            TransferDSPMessageType::TransferRequestMessage => self.load_request(caller, typed).await,
            _ => self.load_continuation(caller, typed).await,
        }
    }

    /// A request opens a new consumer process; nothing is stored until the provider accepts.
    async fn load_request(&self, caller: &UserInfo, typed: TransferRpcTyped) -> Outcome<TransferRpcDomain> {
        let body = typed.body.clone();
        let process_id = TransferProcessId::generate().as_urn().clone();
        let consumer_pid = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        let transfer_direction = match body.data_address {
            Some(_) => TransferDirection::Push,
            None => TransferDirection::Pull,
        };
        let callback = body
            .callback_address
            .clone()
            .unwrap_or_else(|| self.own_dsp_base.clone());
        let message = typed.message.clone();
        let mut domain = TransferRpcDomain {
            process: None,
            process_id,
            role: TransferRole::Consumer,
            transfer_direction,
            owner: Owner::for_new(caller, None, None),
            peer_id: body.associated_agent_peer.clone().unwrap_or_default(),
            target_url: format!(
                "{}/transfers/request",
                Self::base(body.provider_address.as_deref().unwrap_or_default())
            ),
            outgoing: RdfMessage::<DspTransfer>::build(&message, Map::new()).await?,
            is_restart: false,
            typed,
        };
        // A push consumer sets its sink up first: the request carries its address.
        let sink = body.data_address.clone().map(DataAddress::from);
        let sink = self
            .dataplane
            .on_transfer_request_pre(&domain.hooks(sink.clone()))
            .await?
            .map(DataAddress::from)
            .or(sink);
        domain.outgoing = RdfMessage::<DspTransfer>::build(
            &message,
            Self::members(json!({
                "consumerPid": consumer_pid,
                "agreementId": body.agreement_id,
                "format": body.format,
                "callbackAddress": callback,
                "dataAddress": sink,
            })),
        )
        .await?;
        Ok(domain)
    }

    /// Any other message moves a process the caller may act on, addressed to the peer's pid.
    async fn load_continuation(
        &self,
        caller: &UserInfo,
        typed: TransferRpcTyped,
    ) -> Outcome<TransferRpcDomain> {
        let pid = typed.pid().unwrap_or_default().to_string();
        let process = self
            .process_repo
            .get_transfer_process_by_key_value(&OwnerScope::All, &pid)
            .await?
            .ok_or_else(|| {
                Errors::missing_resource(pid.clone(), "transfer process not found", None)
            })?;
        let correlation = process.correlation().clone();
        let peer_id = correlation
            .peer_participant_id
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default();
        DspActor::user(caller).authorize(process.owner(), &peer_id, process.id().as_urn())?;

        let role = process.role();
        let (consumer_pid, provider_pid) = (
            correlation.consumer_pid.clone().unwrap_or_default(),
            correlation.provider_pid.clone().unwrap_or_default(),
        );
        let peer_pid = match role {
            TransferRole::Provider => &consumer_pid,
            _ => &provider_pid,
        };
        let base = correlation
            .callback_address
            .as_ref()
            .map(Url::to_string)
            .unwrap_or_default();
        let message = typed.message.clone();
        let is_restart = process.state().0 == TransferDSPState::SUSPENDED.to_string()
            && message == TransferDSPMessageType::TransferStartMessage;
        let (code, reason) = (typed.body.code.clone(), typed.body.reason.clone());
        let mut domain = TransferRpcDomain {
            target_url: format!(
                "{}/transfers/{peer_pid}/{}",
                Self::base(&base),
                Self::segment(&message)
            ),
            process_id: process.id().as_urn().clone(),
            transfer_direction: process
                .properties()
                .get(DIRECTION_PROPERTY)
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or(TransferDirection::Pull),
            owner: process.owner().clone(),
            role,
            peer_id,
            outgoing: RdfMessage::<DspTransfer>::build(&message, Map::new()).await?,
            is_restart,
            process: Some(process),
            typed,
        };
        // A provider starting a pull hands the consumer its endpoint in the start message.
        let endpoint = match (&message, role) {
            (TransferDSPMessageType::TransferStartMessage, TransferRole::Provider) => self
                .dataplane
                .on_transfer_start_pre(&domain.hooks(None))
                .await?
                .map(DataAddress::from),
            _ => None,
        };
        domain.outgoing = RdfMessage::<DspTransfer>::build(
            &message,
            Self::members(json!({
                "consumerPid": consumer_pid,
                "providerPid": provider_pid,
                "dataAddress": endpoint,
                "code": code,
                "reason": reason,
            })),
        )
        .await?;
        Ok(domain)
    }

    fn members(value: Value) -> Map<String, Value> {
        match value {
            Value::Object(map) => map,
            _ => Map::new(),
        }
    }

    fn base(url: &str) -> &str {
        url.trim_end_matches('/')
    }

    /// DSP 10.2 path segment of a state change.
    fn segment(message: &TransferDSPMessageType) -> &'static str {
        match message {
            TransferDSPMessageType::TransferStartMessage => "start",
            TransferDSPMessageType::TransferCompletionMessage => "completion",
            TransferDSPMessageType::TransferSuspensionMessage => "suspension",
            _ => "termination",
        }
    }
}
