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

//! Pieces of Dataspace Protocol 2025-1 shared by the catalog, negotiation and transfer agents.
//!
//! Each agent owns its own messages and state machine; what they have in common lives here:
//! the middleware that normalises incoming JSON-LD, the DSP profile for the RDF engine, the
//! `@context` and version helpers, the [`DspActor`] that decides who may touch a process,
//! the URN and message rules in [`DspRules`], and the ODRL and data address types that
//! travel inside messages.
//!
//! ## 1. Normalising incoming messages
//!
//! Peers such as the DSP TCK send compact JSON-LD with prefixed keys (`dspace:consumerPid`,
//! `odrl:target`). Our serde types expect the bare names, so every DSP router puts
//! [`dsp_namespace_normalizer`] in front. It strips the `dspace:`, `odrl:` and `dct:` prefixes
//! from keys and `@type` values and keeps the untouched bytes as a [`WireBody`] extension, for
//! signature checks and audit.
//!
//! ```rust,ignore
//! use common::dsp_common::normalizer::{dsp_namespace_normalizer, WireBody};
//!
//! Router::new()
//!     .route("/request", post(Self::handle_catalog_request))
//!     .layer(middleware::from_fn_with_state(peer_auth, PeerAuth::run))
//!     .layer(middleware::from_fn(dsp_namespace_normalizer))
//!     .with_state(self)
//!
//! // In a handler, what the peer really sent:
//! let wire = request.extensions().get::<WireBody>().map(|w| w.0.clone());
//! ```
//!
//! [`dsp_namespace_normalizer`]: normalizer::dsp_namespace_normalizer
//! [`WireBody`]: normalizer::WireBody
//!
//! ## 2. JSON-LD with the DSP profile
//!
//! [`DspProfile`] configures the [`crate::rdf`] engine with the DSP 2025-1 context and ODRL
//! profile embedded in the binary, so they never hit the network. Use the process-wide engine
//! from `shared()`; see [`crate::rdf`] for extraction and hashing.
//!
//! ```rust,ignore
//! use common::dsp_common::rdf::DspProfile;
//!
//! let expansion = DspProfile::shared().expand(&payload).await?;
//! let hash = Sha256::digest(expansion.canonical_n_quads.as_bytes());
//!
//! let request: TransferRequest = DspProfile::shared().extract(&payload).await?;
//! ```
//!
//! [`DspProfile`]: rdf::DspProfile
//!
//! ## 3. Context and protocol version
//!
//! [`ContextField`] is the `@context` of outgoing messages (the 2025-1 context by default) and
//! validates incoming ones, which may use 2024-1 or 2025-1. The version a peer speaks is read
//! from its `@context`.
//!
//! ```rust,ignore
//! use common::dsp_common::context_field::{version_from_payload, ContextField};
//!
//! let message = CatalogMessage { context: ContextField::default(), /* ... */ };
//! message.context.validate()?;
//!
//! let version = version_from_payload(&payload); // Some(DSPProtocolVersions::V2025_1)
//! ```
//!
//! [`ContextField`]: context_field::ContextField
//!
//! ## 4. Who is acting on a process
//!
//! A process is touched either by a peer over the protocol or by a local user over the RPC
//! API. [`DspActor`] captures which one, and `authorize` checks it against the process owner:
//! a peer must be the process counterparty, a user must act on the process's owner (its own,
//! below its role, or opened by a peer for its role). A refusal looks like a missing process.
//!
//! ```rust,ignore
//! use common::dsp_common::DspActor;
//!
//! let actor = DspActor::peer(mate);        // protocol endpoint, after SSI auth
//! let actor = DspActor::user(&scope);      // RPC endpoint, after OAuth
//!
//! actor.authorize(&process.owner(), &process.associated_agent_peer, pid)?;
//! ```
//!
//! [`PeerAuth`] is the axum layer every DSP plane mounts: it verifies the peer's token, looks
//! its participant up and leaves a [`DspCaller`] in the request; on RPC routes the OAuth user
//! is the caller. Flow adapters read it back with `DspCaller::from_extensions`.
//!
//! ```rust,ignore
//! use common::dsp_common::{DspCaller, PeerAuth};
//!
//! router.layer(middleware::from_fn_with_state(PeerAuth::new(grants, mates), PeerAuth::run));
//!
//! let caller = DspCaller::from_extensions(request.extensions())?;
//! caller.actor().authorize(&process.owner(), counterparty, pid)?;
//! ```
//!
//! ## 5. Message rules
//!
//! [`DspRules`] are atomic checks for [`crate::validation`] pipelines: URN syntax, the path pid
//! matching the body pid, a DSP `@context` and the expected `@type`.
//!
//! ```rust,ignore
//! use common::dsp_common::DspRules;
//! use common::validation::Validator;
//!
//! let validator = Validator::new()
//!     .rule(|m: &Value| DspRules::valid_dsp_context(m, "@context"))
//!     .rule(|m: &Value| DspRules::expected_type(m, "TransferRequestMessage", "@type"))
//!     .then()
//!     .rule(move |m: &Value| {
//!         let body_pid = m["consumerPid"].as_str().unwrap_or("");
//!         DspRules::correlate_pids(&uri_pid, body_pid, "consumerPid")
//!     });
//! ```
//!
//! ## 6. Where DSP is mounted
//!
//! [`paths`] holds the base every agent mounts its DSP routes under, and the one callbacks name.
//!
//! ```rust,ignore
//! use common::dsp_common::paths;
//!
//! Some((paths::TRANSFERS.to_string(), router))                // a module's DSP plane
//! let callback = format!("{host}{}", paths::DSP_CURRENT);   // our base, as peers call it
//! ```
//!
//! ## 7. Storing a process
//!
//! [`DspPersistenceTrait`] is the write side every agent gives its DSP flows: hydrate a process
//! by any of its pids, change its state recording the message that caused it, or just record a
//! message. Each agent picks its process, state and recorded message types.
//!
//! ```rust,ignore
//! use common::dsp_common::persistence::{DspPersistenceTrait, StateChange};
//!
//! let process = persistence.hydrate_process(pid).await?.ok_or_else(not_found)?;
//! let change = StateChange { target: State::Started, code: None, reason: None, message };
//! let process = persistence.change_process_state(&process, change).await?;
//! ```
//!
//! [`DspPersistenceTrait`]: persistence::DspPersistenceTrait
//!
//! ## 8. Shared wire types
//!
//! - [`odrl`]: offers, agreements, permissions, constraints and the offer forms a contract
//!   request may carry.
//! - [`data_address`]: `DataAddress` and its endpoint properties (DSP Appendix A).
//! - [`well_known_types`]: protocol versions, bindings and the `.well-known/dspace-version`
//!   response.

use serde_json::Value;

pub mod actor;
pub mod caller;
pub mod context_field;
pub mod data_address;
pub mod error_body;
pub mod normalizer;
pub mod odrl;
pub mod paths;
pub mod persistence;
pub mod rdf;
pub mod rules;
pub mod well_known_types;

pub use actor::DspActor;
pub use caller::{DspCaller, PeerAuth};
pub use error_body::DspErrorBody;
pub use rules::DspRules;

/// Parses an embedded JSON schema; panics on invalid JSON, so only use it on compiled-in assets.
pub fn schema_compiler_util(schema_content: &str) -> Value {
    serde_json::from_str::<Value>(schema_content).unwrap()
}

#[cfg(test)]
mod tests;
