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

//! The DSP negotiation layer: message types, states, RDF extraction, inbound stages and rules,
//! and the offer the consumer sends when the TCK starts a negotiation.

mod context_dsp;
mod context_rpc;
mod dsp_rules;
mod message_types;
mod rdf_extractor_dsp;
mod rpc_rules;
mod state_machine;
mod tck_offer;
