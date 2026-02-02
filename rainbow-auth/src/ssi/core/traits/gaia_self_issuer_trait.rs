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

use crate::ssi::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use async_trait::async_trait;
use serde_json::Value;
use tracing::error;
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::services::wallet::WalletTrait;
use ymir::types::issuing::{AuthServerMetadata, IssuerMetadata, IssuingToken, VCCredOffer};
use ymir::types::wallet::OidcUri;

#[async_trait]
pub trait CoreGaiaSelfIssuerTrait: Send + Sync + 'static {
    fn self_issuer(&self) -> Arc<dyn GaiaSelfIssuerTrait>;
    fn wallet(&self) -> Option<Arc<dyn WalletTrait>>;
    async fn generate_gaia_vcs(&self) -> anyhow::Result<Option<OidcUri>> {
        let id = uuid::Uuid::new_v4().to_string();
        let uri = self.self_issuer().generate_issuing_uri(&id);

        match self.wallet() {
            Some(wallet) => {
                let payload = OidcUri { uri };
                let cred_offer = wallet.resolve_credential_offer(&payload).await?;
                let _issuer_metadata = wallet.resolve_credential_issuer(&cred_offer).await?;
                wallet.use_offer_req(&payload, &cred_offer).await?;
                Ok(None)
            }
            None => Ok(Some(OidcUri { uri })),
        }
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
        let did = match self.wallet() {
            Some(wallet) => wallet.get_did().await?,
            None => self.self_issuer().get_did(),
        };

        self.self_issuer().issue_cred(&did).await
    }

    async fn request_gaia_vc(&self) -> anyhow::Result<()> {
        let wallet = self.wallet().ok_or_else(|| {
            let error = Errors::not_impl_new(
                "Not implemented if wallet is not connected",
                "Not implemented if wallet is not connected",
            );
            error!("{}", error.log());
            error
        })?;

        let vcs = wallet.retrieve_wallet_credentials().await?;

        let did = wallet.get_did().await?;
        let kk = self.self_issuer().build_vp(vcs, Some(did)).await?;
        
        Ok(())
    }
}
