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
use crate::core::authority_trait::AuthorityTrait;
use crate::data::entities::request;
use crate::errors::Errors;
use crate::services::access_manager::AccessManagerServiceTrait;
use crate::services::client::ClientServiceTrait;
use crate::services::issuer::IssuerServiceTrait;
use crate::services::repo::RepoServiceTrait;
use crate::services::verifier::VerifierServiceTrait;
use crate::services::wallet::WalletServiceTrait;
use crate::setup::AuthorityApplicationConfig;
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::gnap::{GrantRequest, GrantResponse, RefBody};
use crate::types::issuing::{AuthServerMetadata, CredentialRequest, GiveVC, IssuerMetadata, IssuingToken, TokenRequest, VCCredOffer, WellKnownJwks};
use crate::types::vcs::{VPDef, VcDecisionApproval};
use crate::types::wallet::{DidsInfo, KeyDefinition};
use anyhow::bail;
use axum::async_trait;
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

pub struct Authority {
    wallet: Arc<dyn WalletServiceTrait>,
    access: Arc<dyn AccessManagerServiceTrait>,
    issuer: Arc<dyn IssuerServiceTrait>,
    verifier: Arc<dyn VerifierServiceTrait>,
    repo: Arc<dyn RepoServiceTrait>,
    client: Arc<dyn ClientServiceTrait>,
    config: AuthorityApplicationConfig,
}

impl Authority {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        access: Arc<dyn AccessManagerServiceTrait>,
        issuer: Arc<dyn IssuerServiceTrait>,
        verifier: Arc<dyn VerifierServiceTrait>,
        repo: Arc<dyn RepoServiceTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: AuthorityApplicationConfig,
    ) -> Self {
        Self { wallet, access, issuer, verifier, repo, client, config }
    }
}

#[async_trait]
impl AuthorityTrait for Authority {
    async fn wallet_register(&self) -> anyhow::Result<()> {
        self.wallet.register().await
    }

    async fn wallet_login(&self) -> anyhow::Result<()> {
        self.wallet.login().await
    }

    async fn wallet_logout(&self) -> anyhow::Result<()> {
        self.wallet.logout().await
    }

    async fn wallet_onboard(&self) -> anyhow::Result<()> {
        let minion = self.wallet.onboard().await?;
        self.repo.minions().force_create(minion).await?;
        Ok(())
    }

    async fn wallet_partial_onboard(&self) -> anyhow::Result<()> {
        self.wallet.partial_onboard().await
    }

    async fn register_key(&self) -> anyhow::Result<()> {
        self.wallet.register_key().await
    }

    async fn register_did(&self) -> anyhow::Result<()> {
        self.wallet.register_did().await
    }

    async fn delete_key(&self, key_definition: KeyDefinition) -> anyhow::Result<()> {
        self.wallet.delete_key(key_definition).await
    }

    async fn delete_did(&self, dids_info: DidsInfo) -> anyhow::Result<()> {
        self.wallet.delete_did(dids_info).await
    }

    async fn did_json(&self) -> anyhow::Result<Value> {
        self.wallet.get_did_doc().await
    }

    async fn vc_access_request(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse> {
        let (n_req_mod, n_int_model) = self.access.manage_acc_req(payload)?;

        let _req_model = self.repo.request().create(n_req_mod).await?;
        let int_model = self.repo.interaction().create(n_int_model).await?;

        if int_model.start.contains(&"oidc4vp".to_string()) {
            let n_ver_model = self.verifier.start_vp(&int_model.id, VcType::Identity)?;
            let ver_model = self.repo.verification().create(n_ver_model).await?;

            let uri = self.verifier.generate_verification_uri(ver_model);

            let response = GrantResponse::default4oidc4vp(
                int_model.id,
                int_model.continue_endpoint,
                int_model.continue_token,
                int_model.as_nonce,
                uri,
            );
            return Ok(response);
        }
        if int_model.start.contains(&"cross-user".to_string()) {
            let response = GrantResponse::default4cross_user(
                int_model.id,
                int_model.continue_endpoint,
                int_model.continue_token,
                int_model.as_nonce,
            );
            return Ok(response);
        }
        let error = Errors::format_new(BadFormat::Received, "Interact method not supported");
        error!("{}", error);
        bail!(error)
    }

    async fn vc_continue_request(&self, cont_id: String, payload: RefBody, token: String) -> anyhow::Result<String> {
        let int_model = self.repo.interaction().get_by_cont_id(&cont_id).await?;
        self.access.validate_cont_req(&int_model, payload.interact_ref, token)?;

        let mut req_model = self.repo.request().get_by_id(&int_model.id).await?;
        let vc_uri = self.issuer.generate_issuing_uri(&int_model.id);

        req_model.status = "Approved".to_string();
        req_model.vc_uri = Some(vc_uri.clone());
        let req_model = self.repo.request().update(req_model).await?;

        let iss_model = self.issuer.start_vci(&req_model);
        self.repo.issuing().create(iss_model).await?;
        info!(vc_uri);
        Ok(vc_uri)
    }

    async fn generate_vp_def(&self, state: String) -> anyhow::Result<VPDef> {
        let ver_model = self.repo.verification().get_by_state(&state).await?;
        let vpd = self.verifier.generate_vpd(ver_model);
        Ok(vpd)
    }

    async fn verify(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>> {
        let mut ver_model = self.repo.verification().get_by_state(&state).await?;
        let result = self.verifier.verify_all(&mut ver_model, vp_token);
        let int_model = self.repo.interaction().get_by_id(&ver_model.id).await?;
        result?;
        self.repo.verification().update(ver_model).await?;
        self.access.end_verification(int_model).await
    }

    async fn get_cred_offer_data(&self, id: String) -> anyhow::Result<VCCredOffer> {
        let mut model = self.repo.issuing().get_by_id(&id).await?;
        let data = self.issuer.get_cred_offer_data(&model)?;
        match model.step {
            true => {
                model.step = false;
                self.repo.issuing().update(model).await?;
            }
            false => {}
        };
        Ok(data)
    }

    fn issuer(&self) -> IssuerMetadata {
        self.issuer.get_issuer_data()
    }

    fn oauth_server(&self) -> AuthServerMetadata {
        self.issuer.get_oauth_server_data()
    }

    fn jwks(&self) -> anyhow::Result<WellKnownJwks> {
        self.wallet.get_jwks_data()
    }

    async fn get_token(&self, payload: TokenRequest) -> anyhow::Result<IssuingToken> {
        let model = self.repo.issuing().get_by_tx_code(&payload.tx_code).await?;
        self.issuer.validate_token_req(&model, &payload.tx_code, &payload.pre_authorized_code)?;
        let response = self.issuer.get_token(&model);
        Ok(response)
    }

    async fn get_credential(&self, payload: CredentialRequest, token: String) -> anyhow::Result<GiveVC> {
        let mut model = self.repo.issuing().get_by_token(&token).await?;
        self.issuer.validate_cred_req(&mut model, &payload, &token)?;
        let did = self.wallet.get_did().await?;
        let data = self.issuer.issue_cred(&mut model, &did)?;
        self.repo.issuing().update(model).await?;

        Ok(data)
    }

    async fn get_all_req(&self) -> anyhow::Result<Vec<request::Model>> {
        self.repo.request().get_all(None, None).await
    }

    async fn get_one_req(&self, id: String) -> anyhow::Result<request::Model> {
        self.repo.request().get_by_id(&id).await
    }

    async fn manage_req(&self, id: String, payload: VcDecisionApproval) -> anyhow::Result<()> {
        let mut req_model = self.repo.request().get_by_id(&id).await?;
        let int_model = self.repo.interaction().get_by_id(&id).await?;
        self.access.apprv_dny_req(payload.approve, &mut req_model, int_model).await?;
        Ok(())
    }
}
