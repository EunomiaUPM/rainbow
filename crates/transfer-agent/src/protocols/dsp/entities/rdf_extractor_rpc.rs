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

//! Reading an outbound transfer RPC body, typed with the eunomia context, off its expansion.
//! DSP concepts use the DSP vocabulary there, so they are read as the inbound side reads them.

use common::eunomia::{EunomiaProfile, EUNOMIA_NS};
use common::rdf::RdfNode;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::TransferRpcBody;
use crate::protocols::dsp::entities::data_address::DataAddressDto;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::rdf_extractor_dsp::{DspTransfer, ExtractProtocolFields};

/// Namespace of the DSP terms the eunomia context reuses.
const DSPACE: &str = "https://w3id.org/dspace/2025/1/";
/// `format` is Dublin Core, as in the DSP context.
const DCT_FORMAT: &str = "http://purl.org/dc/terms/format";
/// RPC types are the DSP message they send, prefixed: `RpcTransferRequestMessage`.
const RPC_PREFIX: &str = "Rpc";

/// The transfer RPC, as an extraction target.
pub struct EunomiaTransfer;

impl ExtractProtocolFields for EunomiaTransfer {
    type MessageType = TransferDSPMessageType;
    type Fields = TransferRpcBody;

    fn type_iri(message: &TransferDSPMessageType) -> String {
        EunomiaProfile::term(&Self::compact_type(message))
    }

    fn message_type(node: &RdfNode<'_, '_>) -> Option<TransferDSPMessageType> {
        node.types()
            .filter_map(|iri| iri.strip_prefix(EUNOMIA_NS)?.strip_prefix(RPC_PREFIX))
            .find_map(|term| term.parse().ok())
    }

    fn extract(node: &RdfNode<'_, '_>) -> Outcome<TransferRpcBody> {
        let reason: Vec<String> = node.get_strings(&Self::dspace("reason"));
        Ok(TransferRpcBody {
            associated_agent_peer: Self::owned(node, &EunomiaProfile::term("associatedAgentPeer")),
            agreement_id: Self::owned(node, &Self::dspace("agreementId")),
            format: Self::owned(node, DCT_FORMAT),
            data_address: DspTransfer::data_address(node)?.map(DataAddressDto::from),
            provider_address: Self::owned(node, &EunomiaProfile::term("providerAddress")),
            callback_address: Self::owned(node, &Self::dspace("callbackAddress")),
            consumer_pid: Self::owned(node, &Self::dspace("consumerPid")),
            provider_pid: Self::owned(node, &Self::dspace("providerPid")),
            code: Self::owned(node, &Self::dspace("code")),
            reason: (!reason.is_empty()).then_some(reason),
        })
    }
}

impl EunomiaTransfer {
    /// The compact `@type` of the RPC that sends `message`.
    pub fn compact_type(message: &TransferDSPMessageType) -> String {
        format!("{RPC_PREFIX}{message}")
    }

    fn dspace(term: &str) -> String {
        format!("{DSPACE}{term}")
    }

    fn owned(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        node.iri_or_literal(predicate).map(str::to_string)
    }
}
