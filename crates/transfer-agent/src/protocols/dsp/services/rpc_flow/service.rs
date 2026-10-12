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

//! Outbound DSP transfer messages, sent by a local user or by the agent itself, as a flow:
//! load the process and build the message, check it, send it, then record what changed.

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use common::dsp_common::data_address::DataAddress;
use common::dsp_common::DspActor;
use common::facades::grants_facade::{send_with_peer_token, GrantsFacadeTrait};
use common::flow::{FlowCx, FlowTrait};
use common::oauth::{Owner, OwnerScope, UserInfo};
use serde_json::{json, Map, Value};
use url::Url;
use urn::Urn;
use ymir::errors::{BadFormat, Errors, Outcome, PetitionFailure};
use ymir::services::client::ClientTrait;
use ymir::types::http::HttpBody;
use ymir::utils::http_client;

use crate::entities::commands::{EditTransferProcessCommand, NewTransferProcessCommand};
use crate::entities::ids::{ParticipantId, TransferProcessId};
use crate::entities::protocol::{
    ProtocolId, ProtocolState, StateMetadata, TransferDirection, TransferRole, CONSUMER_PID_KEY,
    PROVIDER_PID_KEY,
};
use crate::entities::transfer_message::Direction;
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::dsp_flow::handlers::TransferHandlerDeps;
use crate::protocols::dsp::entities::context_rpc::{TransferRpcCommand, TransferRpcFacts};
use crate::protocols::dsp::entities::rdf_message::RdfDspMessage;
use crate::protocols::dsp::services::dsp_domain_loader::dsp_domain_loader::DIRECTION_PROPERTY;
use crate::protocols::dsp::services::rpc_flow::views::TransferRpcReply;

/// Sends the five transfer messages to the peer and moves the local process with them.
pub struct TransferRpcFlow {
    deps: TransferHandlerDeps,
    grants: Arc<dyn GrantsFacadeTrait>,
    /// This agent's DSP base, the default callback a provider answers to.
    own_dsp_base: String,
}

impl TransferRpcFlow {
    pub fn new(
        deps: TransferHandlerDeps,
        grants: Arc<dyn GrantsFacadeTrait>,
        own_dsp_base: String,
    ) -> Self {
        Self {
            deps,
            grants,
            own_dsp_base,
        }
    }

    /// A request opens a new consumer process; nothing is stored until the provider accepts.
    async fn load_request(
        &self,
        caller: &UserInfo,
        command: TransferRpcCommand,
    ) -> Outcome<TransferRpcFacts> {
        let body = &command.body;
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
        let mut facts = TransferRpcFacts {
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
            outgoing: RdfDspMessage::build(&command.message, Map::new()).await?,
            is_restart: false,
            command: command.clone(),
        };
        // A push consumer sets its sink up first: the request carries its address.
        let sink = body.data_address.clone().map(DataAddress::from);
        let sink = self
            .deps
            .dataplane
            .on_transfer_request_pre(&facts.hooks(sink.clone()))
            .await?
            .map(DataAddress::from)
            .or(sink);
        facts.outgoing = RdfDspMessage::build(
            &command.message,
            Self::members(json!({
                "consumerPid": consumer_pid,
                "agreementId": body.agreement_id,
                "format": body.format,
                "callbackAddress": callback,
                "dataAddress": sink,
            })),
        )
        .await?;
        Ok(facts)
    }

    /// Any other message moves a process the caller may act on, addressed to the peer's pid.
    async fn load_continuation(
        &self,
        caller: &UserInfo,
        command: TransferRpcCommand,
    ) -> Outcome<TransferRpcFacts> {
        let pid = command.pid().unwrap_or_default().to_string();
        let urn = Urn::from_str(&pid).map_err(|_| {
            Errors::format(
                BadFormat::Received,
                format!("pid is not a URN: {pid}"),
                None,
            )
        })?;
        let process = self
            .deps
            .process_repo
            .get_transfer_process_by_key_value(&OwnerScope::All, &urn)
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
        DspActor::user(caller).authorize(process.owner(), &peer_id, &urn)?;

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
        let is_restart = process.state().0 == TransferDSPState::SUSPENDED.to_string()
            && command.message == TransferDSPMessageType::TransferStartMessage;
        let mut facts = TransferRpcFacts {
            target_url: format!(
                "{}/transfers/{peer_pid}/{}",
                Self::base(&base),
                Self::segment(&command.message)
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
            outgoing: RdfDspMessage::build(&command.message, Map::new()).await?,
            is_restart,
            process: Some(process),
            command: command.clone(),
        };
        // A provider starting a pull hands the consumer its endpoint in the start message.
        let endpoint = match (&command.message, role) {
            (TransferDSPMessageType::TransferStartMessage, TransferRole::Provider) => self
                .deps
                .dataplane
                .on_transfer_start_pre(&facts.hooks(None))
                .await?
                .map(DataAddress::from),
            _ => None,
        };
        facts.outgoing = RdfDspMessage::build(
            &command.message,
            Self::members(json!({
                "consumerPid": consumer_pid,
                "providerPid": provider_pid,
                "dataAddress": endpoint,
                "code": command.body.code,
                "reason": command.body.reason,
            })),
        )
        .await?;
        Ok(facts)
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

    /// POSTs `body` to `url`; any non-2xx answer is an error carrying the peer's body.
    async fn post(url: String, headers: Option<http::HeaderMap>, body: Value) -> Outcome<Value> {
        let response = http_client()
            .post(&url, headers, HttpBody::Json(body))
            .await?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(Errors::petition(
                &url,
                "POST",
                Some(status),
                PetitionFailure::HttpStatus(status),
                text,
                None,
            ));
        }
        Ok(serde_json::from_str(&text).unwrap_or(Value::Null))
    }

    async fn apply_request(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        facts: &TransferRpcFacts,
        reply: &Value,
    ) -> Outcome<TransferProcess> {
        let body = &facts.command.body;
        let consumer_pid = facts
            .outgoing
            .fields
            .consumer_pid
            .clone()
            .unwrap_or_default();
        let provider_pid = reply["fields"]["providerPid"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let requested = ProtocolState(TransferDSPState::REQUESTED.to_string().into());
        let cmd = NewTransferProcessCommand {
            id: Some(TransferProcessId::new(facts.process_id.clone())),
            visibility: None,
            owner: None,
            role: TransferRole::Consumer,
            protocol: ProtocolId::Dsp2025_1,
            initial_state: requested.clone(),
            initial_state_metadata: StateMetadata {
                attribute: Some(TransferDSPStateAttribute::OnRequest.to_string()),
                reason: None,
                code: None,
            },
            callback_address: body
                .provider_address
                .as_deref()
                .and_then(|u| Url::parse(u).ok()),
            connector_instance_id: None,
            agreement_id: Urn::from_str(body.agreement_id.as_deref().unwrap_or_default()).map_err(
                |_| Errors::format(BadFormat::Received, "agreementId is not a URN", None),
            )?,
            peer_participant_id: ParticipantId::new(facts.peer_id.clone()),
            identifiers: Some(HashMap::from([
                (CONSUMER_PID_KEY.to_string(), consumer_pid),
                (PROVIDER_PID_KEY.to_string(), provider_pid),
            ])),
            properties: Some(json!({ DIRECTION_PROPERTY: facts.transfer_direction })),
        };
        self.deps.processes.create(cx.caller, &cmd).await?;
        let process = self.deps.reload(&facts.process_id).await?;
        self.deps
            .record(
                &process,
                Direction::Outbound,
                &facts.command.message,
                facts.outgoing.envelope(),
                (&requested, &requested),
            )
            .await?;
        self.deps
            .dataplane
            .on_transfer_request_post(&facts.hooks(facts.outgoing.fields.data_address.clone()))
            .await?;
        Ok(process)
    }

    async fn apply_transition(&self, facts: &TransferRpcFacts) -> Outcome<TransferProcess> {
        let current = facts
            .process
            .as_ref()
            .ok_or_else(|| Errors::crazy("a state change needs a process", None))?;
        let hooks = facts.hooks(facts.outgoing.fields.data_address.clone());
        let dataplane = &self.deps.dataplane;
        use TransferDSPMessageType::*;
        match facts.command.message {
            TransferCompletionMessage => dataplane.on_transfer_completion_pre(&hooks).await?,
            TransferSuspensionMessage => dataplane.on_transfer_suspension_pre(&hooks).await?,
            TransferTerminationMessage => dataplane.on_transfer_termination_pre(&hooks).await?,
            _ => None,
        };
        let target = match facts.command.message {
            TransferStartMessage => TransferDSPState::STARTED,
            TransferCompletionMessage => TransferDSPState::COMPLETED,
            TransferSuspensionMessage => TransferDSPState::SUSPENDED,
            _ => TransferDSPState::TERMINATED,
        };
        let (from, to) = (
            current.state().clone(),
            ProtocolState(target.to_string().into()),
        );
        let attribute = match facts.role {
            TransferRole::Provider => TransferDSPStateAttribute::ByProvider,
            _ => TransferDSPStateAttribute::ByConsumer,
        };
        let edit = EditTransferProcessCommand {
            state: Some(to.clone()),
            state_metadata: Some(StateMetadata {
                attribute: Some(attribute.to_string()),
                reason: facts.command.body.reason.clone(),
                code: facts.command.body.code.clone(),
            }),
            identifiers: None,
            properties: None,
            error_details: None,
        };
        self.deps
            .processes
            .edit(&TransferHandlerDeps::actor(), &facts.process_id, &edit)
            .await?;
        let process = self.deps.reload(&facts.process_id).await?;
        self.deps
            .record(
                &process,
                Direction::Outbound,
                &facts.command.message,
                facts.outgoing.envelope(),
                (&from, &to),
            )
            .await?;
        match facts.command.message {
            TransferStartMessage => dataplane.on_transfer_start_post(&hooks).await?,
            TransferCompletionMessage => dataplane.on_transfer_completion_post(&hooks).await?,
            TransferSuspensionMessage => dataplane.on_transfer_suspension_post(&hooks).await?,
            _ => dataplane.on_transfer_termination_post(&hooks).await?,
        };
        Ok(process)
    }
}

#[async_trait::async_trait]
impl FlowTrait for TransferRpcFlow {
    type Key = TransferDSPMessageType;
    type Caller = UserInfo;
    type Input = TransferRpcCommand;
    type Facts = TransferRpcFacts;
    type Output = TransferRpcReply;

    fn key(&self, input: &TransferRpcCommand) -> TransferDSPMessageType {
        input.message.clone()
    }

    async fn load(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        input: TransferRpcCommand,
    ) -> Outcome<TransferRpcFacts> {
        match input.message {
            TransferDSPMessageType::TransferRequestMessage => {
                self.load_request(cx.caller, input).await
            }
            _ => self.load_continuation(cx.caller, input).await,
        }
    }

    /// Sends the message as built and reads the answer back through RDF.
    async fn send(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        facts: &TransferRpcFacts,
    ) -> Outcome<Option<Value>> {
        let (url, body) = (facts.target_url.clone(), facts.outgoing.json.clone());
        let requested = facts.role == TransferRole::Consumer;
        let raw = send_with_peer_token(
            self.grants.as_ref(),
            cx.caller,
            &facts.peer_id,
            requested,
            |headers| Self::post(url.clone(), headers, body.clone()),
        )
        .await?;
        let read = match &raw {
            Value::Null => json!({ "raw": Value::Null }),
            answer => {
                let message = RdfDspMessage::read(answer.clone()).await?;
                json!({ "type": message.message.to_string(), "fields": message.fields, "raw": answer })
            }
        };
        Ok(Some(read))
    }

    async fn apply(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        facts: TransferRpcFacts,
        reply: Option<Value>,
    ) -> Outcome<TransferRpcReply> {
        let reply = reply.unwrap_or(Value::Null);
        let process = match facts.command.message {
            TransferDSPMessageType::TransferRequestMessage => {
                self.apply_request(cx, &facts, &reply).await?
            }
            _ => self.apply_transition(&facts).await?,
        };
        let model = self
            .deps
            .processes
            .get_one(&TransferHandlerDeps::actor(), process.id().as_urn())
            .await?;
        Ok(TransferRpcReply {
            context: TransferRpcReply::CONTEXT,
            kind: TransferRpcReply::TYPE,
            request: facts.command.json,
            response: reply["raw"].clone(),
            transfer_agent_model: model,
            agreement_id: process
                .correlation()
                .agreement_id
                .as_ref()
                .map(Urn::to_string)
                .unwrap_or_default(),
            role: facts.role,
            message: facts.command.message,
        })
    }
}
