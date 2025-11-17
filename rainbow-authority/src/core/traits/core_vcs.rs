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

use crate::data::entities::request;
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::repo::RepoTrait;
use crate::types::vcs::VcDecisionApproval;
use axum::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CoreVcsTrait: Send + Sync + 'static {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait>;
    fn repo(&self) -> Arc<dyn RepoTrait>;
    async fn get_all(&self) -> anyhow::Result<Vec<request::Model>> {
        self.repo().request().get_all(None, None).await
    }
    async fn get_by_id(&self, id: String) -> anyhow::Result<request::Model> {
        self.repo().request().get_by_id(&id).await
    }
    async fn manage_req(&self, id: String, payload: VcDecisionApproval) -> anyhow::Result<()> {
        let mut req_model = self.repo().request().get_by_id(&id).await?;
        let int_model = self.repo().interaction().get_by_id(&id).await?;
        self.gatekeeper().apprv_dny_req(payload.approve, &mut req_model, int_model).await?;
        Ok(())
    }
}
