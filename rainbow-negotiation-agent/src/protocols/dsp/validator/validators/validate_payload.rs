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

use crate::entities::negotiation_process::NegotiationProcessDto;
use crate::protocols::dsp::protocol_types::NegotiationProcessMessageTrait;
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use anyhow::{anyhow, bail};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::dcat_formats::{DctFormats, FormatAction};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct ValidatePayloadService {
    helpers: Arc<dyn ValidationHelpers>,
}
impl ValidatePayloadService {
    pub fn new(helpers: Arc<dyn ValidationHelpers>) -> Self {
        Self { helpers }
    }
}
#[async_trait::async_trait]
impl ValidatePayload for ValidatePayloadService {
    #[allow(unused)]
    async fn validate_with_json_schema(&self, payload: &dyn NegotiationProcessMessageTrait) -> anyhow::Result<()> {
        // TODO set json_schema
        Ok(())
    }

    async fn validate_uri_id_as_urn(&self, uri_id: &String) -> anyhow::Result<()> {
        self.helpers.parse_urn(uri_id).await.map_err(|e| {
            let err = CommonErrors::parse_new(format!("Uri id parameter must be urn. {}", e.to_string()).as_str());
            error!("{}", err.log());
            anyhow!(err)
        })?;
        Ok(())
    }

    #[allow(unused)]
    async fn validate_identifiers_as_urn(&self, payload: &dyn NegotiationProcessMessageTrait) -> anyhow::Result<()> {
        // Are as urn defined in dtos
        Ok(())
    }

    async fn validate_uri_and_pid(
        &self,
        uri_id: &String,
        payload: &dyn NegotiationProcessMessageTrait,
        role: &RoleConfig,
    ) -> anyhow::Result<()> {
        let identifier = match role {
            RoleConfig::Provider => payload.get_provider_pid(),
            RoleConfig::Consumer => payload.get_consumer_pid(),
            _ => {
                let err = CommonErrors::parse_new("Something went wrong. Role not recognized.");
                error!("{}", err.log());
                bail!(err)
            }
        }
        .ok_or_else(|| {
            let err = CommonErrors::parse_new("Something went wrong. Role not recognized.");
            error!("{}", err.log());
            anyhow!(err)
        })?
        .to_string();
        let uri_id = self.helpers.parse_urn(uri_id).await?.to_string();
        if identifier.ne(&uri_id) {
            let err = CommonErrors::parse_new("Uri string and body identifier are not correlated");
            error!("{}", err.log());
            bail!(err);
        }
        Ok(())
    }

    async fn validate_correlation(
        &self,
        payload: &dyn NegotiationProcessMessageTrait,
        dto: &NegotiationProcessDto,
    ) -> anyhow::Result<()> {
        let provider_pid_in_dto = self.helpers.get_pid_by_role(dto, &RoleConfig::Provider).await?.to_string();
        let consumer_pid_in_dto = self.helpers.get_pid_by_role(dto, &RoleConfig::Consumer).await?.to_string();
        let provider_pid_in_payload = payload.get_provider_pid().unwrap_or(Urn::from_str("urn:fake:0")?).to_string();
        let consumer_pid_in_payload = payload.get_consumer_pid().unwrap_or(Urn::from_str("urn:fake:0")?).to_string();
        if provider_pid_in_dto != provider_pid_in_payload || consumer_pid_in_dto != consumer_pid_in_payload {
            let err = CommonErrors::parse_new("Uri string and body identifier are not correlated");
            error!("{}", err.log());
            bail!(err);
        }
        Ok(())
    }

    #[allow(unused)]
    async fn validate_auth(&self, payload: &dyn NegotiationProcessMessageTrait) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    async fn validate_format_data_address(&self, payload: &dyn NegotiationProcessMessageTrait) -> anyhow::Result<()> {
        Ok(())
    }

    async fn validate_data_address_in_start(
        &self,
        _payload: &dyn NegotiationProcessMessageTrait,
        _dto: &NegotiationProcessDto,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
