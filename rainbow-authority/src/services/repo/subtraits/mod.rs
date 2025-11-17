/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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
mod interaction_trait;
mod request_trait;
mod verification_trait;
mod basic_repo_trait;
mod minions_trait;
mod issuing_trait;

pub use interaction_trait::InteractionRepoTrait;
pub use request_trait::RequestRepoTrait;
pub use verification_trait::VerificationRepoTrait;
pub use basic_repo_trait::BasicRepoTrait;
pub use minions_trait::MinionsRepoTrait;
pub use issuing_trait::IssuingRepoTrait;
