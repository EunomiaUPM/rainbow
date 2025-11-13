/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

mod recv_interaction_trait;
mod recv_request_trait;
mod recv_token_requirements_trait;
mod recv_verification_trait;
mod business_mates_trait;

pub use recv_interaction_trait::RecvInteractionTrait;
pub use recv_request_trait::RecvRequestTrait;
pub use recv_token_requirements_trait::RecvTokenRequirementsTrait;
pub use recv_verification_trait::RecvVerificationTrait;
pub use business_mates_trait::BusinessMatesRepoTrait;