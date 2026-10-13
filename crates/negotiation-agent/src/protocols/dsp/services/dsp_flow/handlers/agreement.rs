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

//! `ContractAgreementMessage` received by the consumer (DSP 8.3.5): the provider agreed.

use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::{AgreementRecord, NegotiationAttachment};
use crate::protocols::dsp::services::dsp_flow::handlers::InboundTransition;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use std::str::FromStr;
use urn::Urn;
use ymir::errors::{BadFormat, Errors};

/// Moves the process to AGREED and stores the agreement with both parties.
pub struct AgreementHandler;

impl AgreementHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        persistence: &NegotiationDspPersistence,
        domain: NegotiationDspDomain,
    ) -> Outcome<NegotiationDspOutcome> {
        let agreement = Self::record(&domain)?;
        InboundTransition::apply(
            persistence,
            &domain,
            NegotiationState::AGREED,
            NegotiationAttachment::Agreement(agreement),
        )
        .await
    }

    fn record(domain: &NegotiationDspDomain) -> Outcome<AgreementRecord> {
        let fields = domain.typed.fields.agreement.clone().unwrap_or_default();
        Ok(AgreementRecord {
            id: Self::stored_id(fields.id.as_deref().unwrap_or_default())?,
            target: Self::urn(fields.target.as_deref(), "agreement.target")?,
            provider_id: fields.assigner.unwrap_or_default(),
            consumer_id: fields.assignee.unwrap_or_default(),
            content: domain
                .typed
                .agreement_content()
                .cloned()
                .unwrap_or_default(),
        })
    }

    /// The agreement id as stored: DSP leaves it opaque, the table keys on URNs (the TCK sends
    /// UUIDs), so anything else is kept under `urn:agreement:`.
    fn stored_id(id: &str) -> Outcome<Urn> {
        Urn::from_str(id)
            .or_else(|_| Self::urn(Some(&format!("urn:agreement:{id}")), "agreement.@id"))
    }

    fn urn(value: Option<&str>, field: &str) -> Outcome<Urn> {
        Urn::from_str(value.unwrap_or_default())
            .map_err(|_| Errors::format(BadFormat::Received, format!("{field} is not a URN"), None))
    }
}
