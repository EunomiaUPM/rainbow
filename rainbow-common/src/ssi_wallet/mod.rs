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

pub use wallet_config::SSIWalletConfig;
pub use wallet_session::WalletSession;
pub use wallet_info::WalletInfo;
pub use dids_info::DidsInfo;
pub use wallet_trait::RainbowSSIAuthWalletTrait;
pub use self_client::ClientConfig;
mod wallet_config;
mod wallet_session;
mod wallet_info;
mod dids_info;
mod wallet_trait;
mod self_client;