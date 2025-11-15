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
use crate::ssi::common::core::{CoreMateTrait, CoreVcRequesterTrait, CoreWalletTrait};
use crate::ssi::provider::config::AuthProviderConfigTrait;
use crate::ssi::provider::core::traits::core_verifier::CoreVerifierTrait;
use crate::ssi::provider::core::traits::CoreGateKeeperTrait;
use std::sync::Arc;

pub trait CoreProviderTrait:
    CoreVerifierTrait + CoreGateKeeperTrait + CoreWalletTrait + CoreVcRequesterTrait + CoreMateTrait + Send + Sync + 'static
{
    fn config(&self) -> Arc<dyn AuthProviderConfigTrait>;
}
