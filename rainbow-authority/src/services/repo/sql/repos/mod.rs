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
mod interaction_repo;
mod request_repo;
mod verification_repo;
mod generic_repo;
mod minions_repo;
mod issuing_repo;

pub use interaction_repo::InteractionRepo;
pub use request_repo::AuthRequestRepo;
pub use verification_repo::AuthVerificationRepo;
pub use minions_repo::MinionsRepo;
pub use issuing_repo::IssuingRepo;
pub use generic_repo::GenericRepo;
