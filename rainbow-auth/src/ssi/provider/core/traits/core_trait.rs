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
use crate::ssi::common::core::{CoreGaiaSelfIssuerTrait, CoreMateTrait, CoreVcRequesterTrait, CoreWalletTrait};
use crate::ssi::provider::config::AuthProviderConfigTrait;
use crate::ssi::provider::core::traits::{CoreBusinessTrait, CoreGateKeeperTrait, CoreVerifierTrait};
use rainbow_common::config::services::SsiAuthConfig;
use std::sync::Arc;

pub trait CoreProviderTrait:
    CoreBusinessTrait
    + CoreVerifierTrait
    + CoreGateKeeperTrait
    + CoreWalletTrait
    + CoreVcRequesterTrait
    + CoreMateTrait
    + CoreGaiaSelfIssuerTrait
    + Send
    + Sync
    + 'static
{
    fn gaia_active(&self) -> bool;
    fn config(&self) -> Arc<SsiAuthConfig>;
}
