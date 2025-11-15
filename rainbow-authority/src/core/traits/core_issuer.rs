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

use crate::services::issuer::IssuerTrait;
use crate::services::repo::RepoTrait;
use crate::services::wallet::WalletTrait;
use crate::types::issuing::{
    AuthServerMetadata, CredentialRequest, GiveVC, IssuerMetadata, IssuingToken, TokenRequest, VCCredOffer,
    WellKnownJwks,
};
use axum::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CoreIssuerTrait: Send + Sync + 'static {
    fn issuer(&self) -> Arc<dyn IssuerTrait>;
    fn wallet(&self) -> Arc<dyn WalletTrait>;
    fn repo(&self) -> Arc<dyn RepoTrait>;
    async fn get_cred_offer_data(&self, id: String) -> anyhow::Result<VCCredOffer> {
        let mut model = self.repo().issuing().get_by_id(&id).await?;
        let data = self.issuer().get_cred_offer_data(&model)?;
        match model.step {
            true => {
                model.step = false;
                self.repo().issuing().update(model).await?;
            }
            false => {}
        };
        Ok(data)
    }
    fn issuer_metadata(&self) -> IssuerMetadata {
        self.issuer().get_issuer_data()
    }

    fn oauth_server_metadata(&self) -> AuthServerMetadata {
        self.issuer().get_oauth_server_data()
    }

    fn jwks(&self) -> anyhow::Result<WellKnownJwks> {
        self.wallet().get_jwks_data()
    }

    async fn get_token(&self, payload: TokenRequest) -> anyhow::Result<IssuingToken> {
        let model = self.repo().issuing().get_by_tx_code(&payload.tx_code).await?;
        self.issuer().validate_token_req(&model, &payload.tx_code, &payload.pre_authorized_code)?;
        let response = self.issuer().get_token(&model);
        Ok(response)
    }

    async fn get_credential(&self, payload: CredentialRequest, token: String) -> anyhow::Result<GiveVC> {
        let mut model = self.repo().issuing().get_by_token(&token).await?;
        self.issuer().validate_cred_req(&mut model, &payload, &token)?;
        let did = self.wallet().get_did().await?;
        let data = self.issuer().issue_cred(&mut model, &did)?;
        self.repo().issuing().update(model).await?;
        Ok(data)
    }
}
