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

//! The inbound flow's load step: the process, agreement, connector, role and direction a
//! typed message refers to.

use std::str::FromStr;
use std::sync::Arc;

use common::dsp_common::DspActor;
use common::oauth::OwnerScope;
use urn::Urn;
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::data::repo::transfer_process::TransferProcessRepoTrait;
use crate::entities::protocol::{TransferDirection, TransferRole};
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::context_common::{
    TransferContextConnectorRole, TransferContextProcessSlot, DIRECTION_PROPERTY,
};
use crate::protocols::dsp::entities::context_dsp::{TransferDspDomain, TransferDspTyped};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::facades::catalog_facade::CatalogFacadeTrait;
use crate::protocols::dsp::facades::negotiation_facade::NegotiationFacadeTrait;

/// Loads or mints the process an inbound message refers to, with its agreement and connector.
pub struct TransferDspLoader {
    process_repo: Arc<dyn TransferProcessRepoTrait>,
    negotiation: Arc<dyn NegotiationFacadeTrait>,
    catalog: Arc<dyn CatalogFacadeTrait>,
}

impl TransferDspLoader {
    pub fn new(
        process_repo: Arc<dyn TransferProcessRepoTrait>,
        negotiation: Arc<dyn NegotiationFacadeTrait>,
        catalog: Arc<dyn CatalogFacadeTrait>,
    ) -> Self {
        Self {
            process_repo,
            negotiation,
            catalog,
        }
    }

    /// Resolves the domain stage of a typed message.
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn load(&self, typed: TransferDspTyped) -> Outcome<TransferDspDomain> {
        let process = self.process_slot(&typed).await?;
        let role = match &process {
            TransferContextProcessSlot::Existing(p) => p.role(),
            // Only a request opens a process, and only a provider receives one.
            TransferContextProcessSlot::New { .. } => TransferRole::Provider,
        };
        let transfer_direction = Self::transfer_direction(&typed, &process);
        let is_restart = Self::is_restart(&typed, &process);
        let agreement_id = Self::agreement_id(&typed, &process)?;
        let agreement = self.negotiation.get_agreement(&agreement_id).await?;
        let connector_instance = match (&process, role) {
            (TransferContextProcessSlot::New { .. }, TransferRole::Provider) => {
                let format = typed.fields.format.as_deref().ok_or_else(|| {
                    Errors::format(BadFormat::Received, "format is required", None)
                })?;
                let connector = self.catalog.get_connector_for(&agreement.target, format).await?;
                TransferContextConnectorRole::ProviderHavingConnector(Box::new(connector))
            }
            _ => TransferContextConnectorRole::ConsumerNotHavingConnector,
        };
        Ok(TransferDspDomain {
            typed,
            process,
            agreement,
            role,
            transfer_direction,
            connector_instance,
            is_restart,
        })
    }

    /// The existing process by either pid, checked against the peer; a request may open one.
    async fn process_slot(
        &self,
        typed: &TransferDspTyped,
    ) -> Outcome<TransferContextProcessSlot> {
        let peer = DspActor::peer(typed.peer());
        for pid in [
            typed.fields.provider_pid.as_deref(),
            typed.fields.consumer_pid.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(found) = self
                .process_repo
                .get_transfer_process_by_key_value(&OwnerScope::All, pid)
                .await?
            {
                let counterparty = found
                    .correlation()
                    .peer_participant_id
                    .as_ref()
                    .map(|p| p.as_str())
                    .unwrap_or_default();
                peer.authorize(found.owner(), counterparty, found.id().as_urn())?;
                return Ok(TransferContextProcessSlot::Existing(Box::new(found)));
            }
        }
        if typed.message != TransferDSPMessageType::TransferRequestMessage {
            let pid = typed
                .fields
                .provider_pid
                .clone()
                .or_else(|| typed.fields.consumer_pid.clone())
                .unwrap_or_default();
            return Err(Errors::missing_resource(
                pid,
                "transfer process not found",
                None,
            ));
        }
        let consumer_pid = typed.fields.consumer_pid.clone().ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "new transfer requires a consumerPid",
                None,
            )
        })?;
        Ok(TransferContextProcessSlot::New { consumer_pid })
    }

    /// The agreement named by the message, or the one the process already runs under.
    fn agreement_id(
        typed: &TransferDspTyped,
        process: &TransferContextProcessSlot,
    ) -> Outcome<Urn> {
        if let Some(id) = typed.fields.agreement_id.as_deref() {
            return Self::urn(id, "agreementId");
        }
        match process {
            TransferContextProcessSlot::Existing(p) => p
                .correlation()
                .agreement_id
                .clone()
                .ok_or_else(|| Errors::crazy("stored transfer process has no agreement", None)),
            TransferContextProcessSlot::New { .. } => Err(Errors::format(
                BadFormat::Received,
                "agreementId is required",
                None,
            )),
        }
    }

    /// Fixed by the request (a data address means push) and read back from the process after.
    fn transfer_direction(
        typed: &TransferDspTyped,
        process: &TransferContextProcessSlot,
    ) -> TransferDirection {
        match process {
            TransferContextProcessSlot::Existing(p) => Self::stored_direction(p),
            TransferContextProcessSlot::New { .. } if typed.fields.data_address.is_some() => {
                TransferDirection::Push
            }
            TransferContextProcessSlot::New { .. } => TransferDirection::Pull,
        }
    }

    /// A start on a suspended process resumes it rather than starting it for the first time.
    fn is_restart(typed: &TransferDspTyped, process: &TransferContextProcessSlot) -> bool {
        matches!(process, TransferContextProcessSlot::Existing(p) if p.state().0 == "SUSPENDED")
            && typed.message == TransferDSPMessageType::TransferStartMessage
    }

    fn stored_direction(process: &TransferProcess) -> TransferDirection {
        process
            .properties()
            .get(DIRECTION_PROPERTY)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or(TransferDirection::Pull)
    }

    fn urn(value: &str, field: &str) -> Outcome<Urn> {
        Urn::from_str(value).map_err(|_| {
            Errors::format(
                BadFormat::Received,
                format!("{field} is not a URN: {value}"),
                None,
            )
        })
    }
}
