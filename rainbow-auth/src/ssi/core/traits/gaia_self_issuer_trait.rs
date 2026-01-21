/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::async_trait;
use serde_json::Value;

use crate::ssi::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use crate::ssi::services::wallet::WalletServiceTrait;
use crate::ssi::types::vc_issuing::{
    AuthServerMetadata, IssuerMetadata, IssuingToken, VCCredOffer
};
use crate::ssi::types::wallet::OidcUri;

#[async_trait]
pub trait CoreGaiaSelfIssuerTrait: Send + Sync + 'static {
    fn self_issuer(&self) -> Arc<dyn GaiaSelfIssuerTrait>;
    fn wallet(&self) -> Arc<dyn WalletServiceTrait>;
    async fn generate_gaia_vcs(&self) -> anyhow::Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let uri = self.self_issuer().generate_issuing_uri(&id);
        let payload = OidcUri { uri };
        let cred_offer = self.wallet().resolve_credential_offer(&payload).await?;
        let _issuer_metadata = self.wallet().resolve_credential_issuer(&cred_offer).await?;
        self.wallet().use_offer_req(&payload, &cred_offer).await
    }
    fn get_cred_offer_data(&self) -> VCCredOffer { self.self_issuer().get_cred_offer_data() }
    fn get_issuer_data(&self) -> IssuerMetadata { self.self_issuer().get_issuer_data() }
    fn get_oauth_server_data(&self) -> AuthServerMetadata {
        self.self_issuer().get_oauth_server_data()
    }
    fn get_token(&self) -> IssuingToken { self.self_issuer().get_token() }
    async fn issue_cred(&self) -> anyhow::Result<Value> {
        let did = self.wallet().get_did().await?;
        self.self_issuer().issue_cred(&did).await
    }
}
