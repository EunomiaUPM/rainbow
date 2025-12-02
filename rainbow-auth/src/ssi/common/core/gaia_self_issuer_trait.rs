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
use crate::ssi::common::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::common::types::vc_issuing::{AuthServerMetadata, IssuerMetadata, IssuingToken, VCCredOffer};
use axum::async_trait;
use serde_json::Value;
use std::sync::Arc;

#[async_trait]
pub trait CoreGaiaSelfIssuerTrait: Send + Sync + 'static {
    fn self_issuer(&self) -> Arc<dyn GaiaSelfIssuerTrait>;
    fn wallet(&self) -> Arc<dyn WalletServiceTrait>;
    fn generate_issuing_uri(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.self_issuer().generate_issuing_uri(&id)
    }
    fn get_cred_offer_data(&self) -> VCCredOffer {
        self.self_issuer().get_cred_offer_data()
    }
    fn get_issuer_data(&self) -> IssuerMetadata {
        self.self_issuer().get_issuer_data()
    }
    fn get_oauth_server_data(&self) -> AuthServerMetadata {
        self.self_issuer().get_oauth_server_data()
    }
    fn get_token(&self) -> IssuingToken {
        self.self_issuer().get_token()
    }
    async fn issue_cred(&self) -> anyhow::Result<Value> {
        let did = self.wallet().get_did().await?;
        self.self_issuer().issue_cred(&did)
    }
}
