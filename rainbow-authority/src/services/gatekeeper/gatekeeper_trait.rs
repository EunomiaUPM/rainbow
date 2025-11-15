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

use crate::data::entities::{interaction, request};
use crate::types::gnap::{GrantRequest, Interact4GR};
use crate::types::vcs::VCIData;
use axum::async_trait;

#[async_trait]
pub trait GateKeeperTrait: Send + Sync + 'static {
    fn manage_acc_req(&self, grant_request: GrantRequest)
        -> anyhow::Result<(request::NewModel, interaction::NewModel)>;
    fn validate_acc_req(&self, payload: &GrantRequest) -> anyhow::Result<Interact4GR>;
    fn manage_cont_req(&self, req_model: &request::Model) -> anyhow::Result<VCIData>;
    fn validate_cont_req(&self, int_model: &interaction::Model, int_ref: String, token: String) -> anyhow::Result<()>;
    async fn end_verification(&self, model: interaction::Model) -> anyhow::Result<Option<String>>;
    async fn apprv_dny_req(
        &self,
        approve: bool,
        req_model: &mut request::Model,
        int_model: interaction::Model,
    ) -> anyhow::Result<()>;
}
