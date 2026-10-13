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

//! How the DSP TCK authenticates in TCK mode: its fixed participant id is also its token.

use ymir::types::oauth::RolePath;
use ymir::types::participants::Visibility;

use crate::facades::grants_facade::VerifiedPeer;
use crate::tck::TCK_MAPPING_ENV;

/// The TCK as a peer. It always calls itself `TCK_PARTICIPANT` and cannot hold a GNAP grant,
/// so in TCK mode that id is accepted as its token, both ways.
pub struct TckPeer;

impl TckPeer {
    pub const PARTICIPANT_ID: &'static str = "TCK_PARTICIPANT";

    /// TCK mode is on when `DSP_TCK_MAPPING` is set.
    pub fn enabled() -> bool {
        std::env::var_os(TCK_MAPPING_ENV).is_some()
    }

    /// The TCK as a verified peer, when TCK mode is on and `token` is its id.
    pub fn verify(enabled: bool, token: &str) -> Option<VerifiedPeer> {
        (enabled && token == Self::PARTICIPANT_ID).then(|| VerifiedPeer {
            participant_id: Self::PARTICIPANT_ID.to_string(),
            role: RolePath::root(),
            visibility: Visibility::Public,
        })
    }

    /// The token to present to `participant_id`, when TCK mode is on and it is the TCK.
    pub fn token_for(enabled: bool, participant_id: &str) -> Option<String> {
        (enabled && participant_id == Self::PARTICIPANT_ID)
            .then(|| Self::PARTICIPANT_ID.to_string())
    }
}
