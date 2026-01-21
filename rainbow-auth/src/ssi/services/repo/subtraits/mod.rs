/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod business_mates_trait;
mod mates_trait;
pub mod recv_interaction_trait;
pub mod recv_request_trait;
pub mod recv_verification_trait;
mod req_interaction_trait;
pub mod req_request_trait;
mod req_vc_trait;
mod req_verification_trait;
mod token_requirements_trait;

pub use mates_trait::MatesTrait;
pub use req_interaction_trait::ReqInteractionTrait;
pub use req_vc_trait::ReqVcTrait;
pub use req_verification_trait::ReqVerificationTrait;
pub use token_requirements_trait::TokenRequirementsTrait;
