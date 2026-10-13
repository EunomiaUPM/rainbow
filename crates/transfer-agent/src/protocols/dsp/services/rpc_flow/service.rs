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

//! The outbound transfer flow: load with the loader, send to the peer, apply with the
//! message's handler.

use std::sync::Arc;

use common::facades::grants_facade::{send_with_peer_token, GrantsFacadeTrait};
use common::flow::{FlowCx, FlowTrait};
use common::oauth::UserInfo;
use common::rdf::RdfMessage;
use serde_json::{json, Value};
use ymir::errors::{Errors, Outcome, PetitionFailure};
use ymir::services::client::ClientTrait;
use ymir::types::http::HttpBody;
use ymir::utils::http_client;

use crate::entities::protocol::TransferRole;
use crate::protocols::dsp::entities::context_rpc::{
    TransferRpcDomain, TransferRpcOutcome, TransferRpcTyped,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::rdf_extractor_dsp::DspTransfer;
use crate::protocols::dsp::services::rpc_flow::handlers::completion::CompletionHandler;
use crate::protocols::dsp::services::rpc_flow::handlers::request::RequestHandler;
use crate::protocols::dsp::services::rpc_flow::handlers::start::StartHandler;
use crate::protocols::dsp::services::rpc_flow::handlers::suspension::SuspensionHandler;
use crate::protocols::dsp::services::rpc_flow::handlers::termination::TerminationHandler;
use crate::protocols::dsp::services::rpc_flow::loader::TransferRpcLoader;
use crate::protocols::dsp::services::transfer_writer::service::TransferWriter;

/// Sends the five transfer messages to the peer and moves the local process with them.
pub struct TransferRpcFlow {
    loader: TransferRpcLoader,
    writer: TransferWriter,
    grants: Arc<dyn GrantsFacadeTrait>,
}

impl TransferRpcFlow {
    pub fn new(
        loader: TransferRpcLoader,
        writer: TransferWriter,
        grants: Arc<dyn GrantsFacadeTrait>,
    ) -> Self {
        Self {
            loader,
            writer,
            grants,
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
}

#[async_trait::async_trait]
impl FlowTrait for TransferRpcFlow {
    type Key = TransferDSPMessageType;
    type Caller = UserInfo;
    type Input = TransferRpcTyped;
    type Facts = TransferRpcDomain;
    type Output = TransferRpcOutcome;

    /// The route's message type; the body's own type is checked against it by a shape rule.
    fn key(&self, typed: &TransferRpcTyped) -> TransferDSPMessageType {
        typed.route().clone()
    }

    async fn load(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        typed: TransferRpcTyped,
    ) -> Outcome<TransferRpcDomain> {
        self.loader.load(cx.caller, typed).await
    }

    /// Sends the message as built; the answer to a request is read back through RDF.
    async fn send(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        domain: &TransferRpcDomain,
    ) -> Outcome<Option<Value>> {
        let (url, body) = (domain.target_url.clone(), domain.outgoing.json.clone());
        let requested = domain.role == TransferRole::Consumer;
        let raw = send_with_peer_token(
            self.grants.as_ref(),
            cx.caller,
            &domain.peer_id,
            requested,
            |headers| Self::post(url.clone(), headers, body.clone()),
        )
        .await?;
        // Only a request is answered with a message (the TransferProcess, DSP 9.3.1); a state
        // change gets a bare 200, whatever body the peer adds.
        let read = match (&domain.typed.message, &raw) {
            (TransferDSPMessageType::TransferRequestMessage, answer) if !answer.is_null() => {
                let message = RdfMessage::<DspTransfer>::read(answer.clone()).await?;
                json!({ "type": message.message.to_string(), "fields": message.fields, "raw": answer })
            }
            _ => json!({ "raw": raw }),
        };
        Ok(Some(read))
    }

    async fn apply(
        &self,
        cx: &FlowCx<'_, UserInfo>,
        domain: TransferRpcDomain,
        reply: Option<Value>,
    ) -> Outcome<TransferRpcOutcome> {
        use TransferDSPMessageType::*;
        let reply = reply.unwrap_or(Value::Null);
        let writer = &self.writer;
        match domain.typed.message {
            TransferRequestMessage => RequestHandler::handle(writer, cx.caller, domain, &reply).await,
            TransferStartMessage => StartHandler::handle(writer, domain, &reply).await,
            TransferCompletionMessage => CompletionHandler::handle(writer, domain, &reply).await,
            TransferSuspensionMessage => SuspensionHandler::handle(writer, domain, &reply).await,
            TransferTerminationMessage => TerminationHandler::handle(writer, domain, &reply).await,
            other => Err(Errors::crazy(format!("{other} is not an outbound message"), None)),
        }
    }
}
