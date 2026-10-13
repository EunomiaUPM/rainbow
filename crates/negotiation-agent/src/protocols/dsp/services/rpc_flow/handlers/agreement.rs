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

//! `ContractAgreementMessage` sent by the provider (DSP 8.3.5).

use serde_json::Value;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::{NegotiationRpcDomain, NegotiationRpcOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::{AgreementRecord, NegotiationAttachment};
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::protocols::dsp::services::rpc_flow::handlers::OutboundTransition;
use crate::setup::context::AppContext;
use std::str::FromStr;
use urn::Urn;
use ymir::errors::{BadFormat, Errors};

/// Moves our process to AGREED and stores the agreement as sent.
pub struct AgreementHandler;

impl AgreementHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        ctx: &AppContext,
        persistence: &NegotiationDspPersistence,
        domain: NegotiationRpcDomain,
        reply: &Value,
    ) -> Outcome<NegotiationRpcOutcome> {
        let agreement = Self::record(&domain)?;
        OutboundTransition::apply(
            ctx,
            persistence,
            domain,
            reply,
            NegotiationState::AGREED,
            NegotiationAttachment::Agreement(agreement),
        )
        .await
    }

    fn record(domain: &NegotiationRpcDomain) -> Outcome<AgreementRecord> {
        let fields = domain.outgoing.fields.agreement.clone().unwrap_or_default();
        Ok(AgreementRecord {
            id: Self::urn(fields.id.as_deref(), "agreement.@id")?,
            target: Self::urn(fields.target.as_deref(), "agreement.target")?,
            provider_id: fields.assigner.unwrap_or_default(),
            consumer_id: fields.assignee.unwrap_or_default(),
            content: domain
                .outgoing
                .json
                .get("agreement")
                .cloned()
                .unwrap_or_default(),
        })
    }

    fn urn(value: Option<&str>, field: &str) -> Outcome<Urn> {
        Urn::from_str(value.unwrap_or_default())
            .map_err(|_| Errors::format(BadFormat::Received, format!("{field} is not a URN"), None))
    }
}
