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

//! The offer the consumer sends when the DSP TCK starts a negotiation, with no doubles.

use negotiation_agent::protocols::dsp::http::rpc::NegotiationRpcRouter;
use serde_json::json;

const DATASET: &str = "urn:dataset:776e1adc-1037-4c68-bc49-4873872d8302";

/// The TCK's made-up offer id gives a full DSP 2025-1 offer: type, target and `use`.
#[test]
fn the_tck_offer_is_a_full_message_offer() {
    let offer_id = format!("offer{DATASET}");
    let offer = NegotiationRpcRouter::tck_offer(&offer_id, DATASET);
    assert_eq!(
        offer,
        json!({"@id": offer_id, "@type": "Offer", "target": DATASET, "permission": [{"action": "use"}]})
    );
}
