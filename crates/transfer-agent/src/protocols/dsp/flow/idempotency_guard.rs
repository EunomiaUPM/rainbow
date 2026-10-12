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

//! A repeated message gets the answer the first one got (DSP 9.2.1), keyed on protocol identity.

use std::sync::Arc;

use common::facades::grants_facade::VerifiedPeer;
use common::flow::{FlowCx, FlowError, FlowGuardTrait, GuardVerdict};
use common::validation::{codes, violation};
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_common::TransferContextProcessSlot;
use crate::protocols::dsp::entities::context_dsp::TransferDSPContextDomain;
use crate::protocols::dsp::entities::idempotency::{
    IdempotencyGuard, IdempotencyStoreTrait, IdempotencyVerdict,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::flow::{TransferDspFlow, TransferDspReply};

/// Claims each message before it is applied and stores the reply once it is committed.
pub struct TransferIdempotencyGuard {
    store: Arc<dyn IdempotencyStoreTrait>,
}

impl TransferIdempotencyGuard {
    pub fn new(store: Arc<dyn IdempotencyStoreTrait>) -> Self {
        Self { store }
    }

    fn process_version(facts: &TransferDSPContextDomain) -> Option<u64> {
        match &facts.process {
            TransferContextProcessSlot::Existing(p) => Some(p.version()),
            TransferContextProcessSlot::New { .. } => None,
        }
    }

    fn conflict(reason: impl Into<String>) -> FlowError {
        FlowError::Conflict(violation("@type", codes::STALE, reason.into()))
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<TransferDspFlow> for TransferIdempotencyGuard {
    async fn after_load(
        &self,
        _cx: &FlowCx<'_, VerifiedPeer>,
        _key: &TransferDSPMessageType,
        facts: &TransferDSPContextDomain,
    ) -> Result<GuardVerdict<TransferDspReply>, FlowError> {
        let guard = IdempotencyGuard::new(self.store.as_ref());
        let version = Self::process_version(facts);
        match guard.check(&facts.typed, version).await? {
            IdempotencyVerdict::New(key) | IdempotencyVerdict::Superseded { key, .. } => {
                guard.claim(key, &facts.typed, version).await?;
                Ok(GuardVerdict::Continue)
            }
            IdempotencyVerdict::Replay(record) => match record.response {
                Some(response) => serde_json::from_value(response)
                    .map(GuardVerdict::Replay)
                    .map_err(|e| Self::conflict(format!("stored reply is unreadable: {e}"))),
                None => Err(Self::conflict("the same message is still being processed")),
            },
            IdempotencyVerdict::Conflict(record) => {
                Err(Self::conflict(record.conflict_error().reason().to_string()))
            }
        }
    }

    async fn after_commit(
        &self,
        _caller: &VerifiedPeer,
        _key: &TransferDSPMessageType,
        output: &TransferDspReply,
    ) -> Outcome<()> {
        let Some(record) = self
            .store
            .get_idempotency_record(&output.idempotency_key)
            .await?
        else {
            return Ok(());
        };
        let response = serde_json::to_value(output).unwrap_or_default();
        IdempotencyGuard::new(self.store.as_ref())
            .record_response(record, response, Some(output.process_version))
            .await
    }
}
