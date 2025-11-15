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
mod core_wallet;
mod core_trait;
mod core_gatekeeper;
mod core_verifier;
mod core_issuer;
mod core_vcs;

pub use core_wallet::CoreWalletTrait;
pub use core_trait::CoreTrait;
pub use core_gatekeeper::CoreGatekeeperTrait;
pub use core_verifier::CoreVerifierTrait;
pub use core_issuer::CoreIssuerTrait;
pub use core_vcs::CoreVcsTrait;