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
use std::format;
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::HeaderMap;
use tracing::{error, info};
use crate::data::entities::{auth_interaction, auth_request};
use crate::errors::Errors;
use crate::types::enums::request::Body;
use crate::types::gnap::{CallbackBody, GrantRequest, Interact4GR, RejectedCallbackBody};
use crate::types::vcs::VCIData;

#[async_trait]
pub trait AccessManagerServiceTrait: Send + Sync {
    fn manage_acc_req(
        &self,
        grant_request: GrantRequest,
    ) -> anyhow::Result<(auth_request::NewModel, auth_interaction::NewModel)>;
    fn validate_acc_req(&self, payload: &GrantRequest) -> anyhow::Result<Interact4GR>;
    fn manage_cont_req(&self, req_model: &auth_request::Model)
        -> anyhow::Result<VCIData>;
    fn validate_cont_req(
        &self,
        int_model: &auth_interaction::Model,
        int_ref: String,
        token: String,
    ) -> anyhow::Result<()>;
    async fn end_verification(&self, model: auth_interaction::Model) -> anyhow::Result<(Option<String>)>;
    async fn apprv_dny_req(&self, approve: bool, req_model: &mut auth_request::Model, int_model: auth_interaction::Model) -> anyhow::Result<()>;
}
