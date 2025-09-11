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

mod auth_interaction_repo;
mod auth_request_repo;
mod auth_token_requirements_repo;
mod auth_verification_repo;
mod mates_repo;
mod authority_request_repo;

pub use auth_interaction_repo::AuthInteractionConsumerRepo;
pub use auth_request_repo::AuthRequestConsumerRepo;
pub use auth_token_requirements_repo::AuthTokenRequirementsConsumerRepo;
pub use auth_verification_repo::AuthVerificationConsumerRepo;
pub use mates_repo::MatesConsumerRepo;
pub use authority_request_repo::AuthorityRequestConsumerRepo;
