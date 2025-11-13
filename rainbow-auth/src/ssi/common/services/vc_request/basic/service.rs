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
use std::sync::Arc;
use tracing::info;
use rainbow_db::auth::common::entities::authority_request;
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::types::entities::{ReachAuthority, ReachMethod};
use crate::ssi::common::types::gnap::GrantRequest;
use super::config::{VCRequesterConfig, VCRequesterConfigTrait};
use super::super::VCRequesterTrait;

pub struct VCReqService {
    client: Arc<dyn ClientServiceTrait>,
    config: VCRequesterConfig,
}

impl VCReqService {
    pub fn new(client: Arc<dyn ClientServiceTrait>, config: VCRequesterConfig) -> Self {
        VCReqService { client, config }
    }
}

impl VCRequesterTrait for VCReqService {
    fn start(&self, payload: ReachAuthority, method: ReachMethod) -> anyhow::Result<()> {
        info!("Begging for a credential");

        let id = uuid::Uuid::new_v4().to_string();
        let client = self.config.get_pretty_client_config()?;
        let callback_uri = format!(
            "{}/api/v1/callback/{}",
            self.config.get_host(),
            &id
        );

        let vc_req_model = authority_request::NewModel {
            id: id.clone(),
            authority_id: payload.id.clone(),
            authority_slug: payload.slug.clone(),
            grant_endpoint: payload.url.clone(),
            vc_type: payload.vc_type.clone(),
        };

        let int_model = auth





        let grant_req = match method {
            ReachMethod::Oidc => GrantRequest::vc_oidc(
                client,
                "redirect".to_string(),
                Some(callback_uri.clone()),
                payload.vc_type.clone(),
            ),
            ReachMethod::CrossUser => GrantRequest::vc_cross_user(client, Some(callback_uri.clone()), payload.vc_type.clone()),
        };

        Ok(())
    }
}