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

//! The inbound flow's load step: the process, our role and the offers a typed message refers to.

use std::str::FromStr;
use std::sync::Arc;

use common::dsp_common::persistence::DspPersistenceTrait;
use common::dsp_common::DspCaller;
use common::facades::catalog::CatalogOffer;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::protocols::dsp::entities::context_common::NegotiationProcessSlot;
use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspTyped};
use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::role::NegotiationRole;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::setup::context::AppContext;

/// Loads the process an inbound message refers to, or makes room for the one it opens.
pub struct NegotiationDspLoader {
    ctx: Arc<AppContext>,
    persistence: NegotiationDspPersistence,
}

impl NegotiationDspLoader {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self {
            persistence: NegotiationDspPersistence::new(ctx.clone()),
            ctx,
        }
    }

    /// Resolves the domain stage of a typed message.
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn load(
        &self,
        caller: &DspCaller,
        typed: NegotiationDspTyped,
    ) -> Outcome<NegotiationDspDomain> {
        let process = self.process_slot(caller, &typed).await?;
        let (role, opening_target) = match &process {
            NegotiationProcessSlot::Existing(p) => {
                (p.role, self.persistence.opening_target(p).await?)
            }
            NegotiationProcessSlot::New { .. } => (
                Self::role_opened_by(typed.message),
                typed.fields.offer.as_ref().and_then(|o| o.target.clone()),
            ),
        };
        let catalog_offer = match (&process, role) {
            (NegotiationProcessSlot::New { .. }, NegotiationRole::Provider) => {
                let offer_id = typed.fields.offer.as_ref().and_then(|o| o.id.as_deref());
                self.published_offer(offer_id).await?
            }
            _ => None,
        };
        Ok(NegotiationDspDomain {
            typed,
            process,
            role,
            catalog_offer,
            opening_target,
        })
    }

    /// The existing process by either pid, checked against the caller; an opening message on an
    /// initial route may start one.
    async fn process_slot(
        &self,
        caller: &DspCaller,
        typed: &NegotiationDspTyped,
    ) -> Outcome<NegotiationProcessSlot> {
        let fields = &typed.fields;
        let pids = [
            fields.provider_pid.as_deref(),
            fields.consumer_pid.as_deref(),
        ];
        for pid in pids.into_iter().flatten() {
            if let Some(found) = self.persistence.hydrate_process(pid).await? {
                caller
                    .actor()
                    .authorize(&found.owner(), found.peer_id(), &found.id()?)?;
                return Ok(NegotiationProcessSlot::Existing(Box::new(found)));
            }
        }
        let opening = match typed.message {
            NegotiationMessageType::ContractRequestMessage if fields.provider_pid.is_none() => {
                fields.consumer_pid.clone()
            }
            NegotiationMessageType::ContractOfferMessage if fields.consumer_pid.is_none() => {
                fields.provider_pid.clone()
            }
            _ => None,
        };
        match (opening, typed.path_id()) {
            (Some(peer_pid), None) => Ok(NegotiationProcessSlot::New { peer_pid }),
            _ => {
                let pid = pids.into_iter().flatten().next().unwrap_or_default();
                Err(Errors::missing_resource(pid, "negotiation not found", None))
            }
        }
    }

    /// A request opens a negotiation we provide; an offer, one we consume.
    fn role_opened_by(message: NegotiationMessageType) -> NegotiationRole {
        match message {
            NegotiationMessageType::ContractOfferMessage => NegotiationRole::Consumer,
            _ => NegotiationRole::Provider,
        }
    }

    /// The offer the catalog publishes under `offer_id`; none when it publishes no such offer.
    async fn published_offer(&self, offer_id: Option<&str>) -> Outcome<Option<CatalogOffer>> {
        let Some(id) = offer_id.and_then(|id| Urn::from_str(id).ok()) else {
            return Ok(None);
        };
        match self.ctx.ports.catalog.get_offer(&id).await {
            Ok(offer) => Ok(Some(offer)),
            Err(Errors::MissingResourceError { .. }) => Ok(None),
            Err(other) => Err(other),
        }
    }
}
