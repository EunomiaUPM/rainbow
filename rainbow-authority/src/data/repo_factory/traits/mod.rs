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
mod auth_interaction_trait;
mod auth_request_trait;
mod auth_verification_trait;
mod basic_repo_trait;
mod into_active_trait;
mod minions_trait;

pub use auth_interaction_trait::AuthInteractionRepoTrait;
pub use auth_request_trait::AuthRequestRepoTrait;
pub use auth_verification_trait::AuthVerificationRepoTrait;
pub use basic_repo_trait::BasicRepoTrait;
pub use into_active_trait::IntoActiveSet;
pub use minions_trait::MinionsRepoTrait;
