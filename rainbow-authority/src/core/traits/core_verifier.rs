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

use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::repo::RepoTrait;
use crate::services::verifier::VerifierTrait;
use crate::types::vcs::VPDef;
use axum::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CoreVerifierTrait: Send + Sync + 'static {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait>;
    fn verifier(&self) -> Arc<dyn VerifierTrait>;
    fn repo(&self) -> Arc<dyn RepoTrait>;
    async fn get_vp_def(&self, state: String) -> anyhow::Result<VPDef> {
        let ver_model = self.repo().verification().get_by_state(&state).await?;
        let vpd = self.verifier().generate_vpd(ver_model);
        Ok(vpd)
    }
    async fn verify(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>> {
        let mut ver_model = self.repo().verification().get_by_state(&state).await?;
        let result = self.verifier().verify_all(&mut ver_model, vp_token);
        let int_model = self.repo().interaction().get_by_id(&ver_model.id).await?;
        result?;
        self.repo().verification().update(ver_model).await?;
        self.gatekeeper().end_verification(int_model).await
    }
}
