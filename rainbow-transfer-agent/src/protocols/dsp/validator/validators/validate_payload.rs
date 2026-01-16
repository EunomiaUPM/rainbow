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

use crate::entities::transfer_process::TransferProcessDto;
use crate::protocols::dsp::protocol_types::{TransferProcessMessageTrait, TransferStateAttribute};
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
    async fn validate_with_json_schema(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()> {
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
    async fn validate_identifiers_as_urn(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()> {
        // Are as urn defined in dtos
        Ok(())
    }

    async fn validate_uri_and_pid(
        &self,
        uri_id: &String,
        payload: &dyn TransferProcessMessageTrait,
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
        payload: &dyn TransferProcessMessageTrait,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<()> {
        let provider_pid_in_dto = self.helpers.get_pid_by_role(dto, RoleConfig::Provider).await?.to_string();
        let consumer_pid_in_dto = self.helpers.get_pid_by_role(dto, RoleConfig::Consumer).await?.to_string();
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
    async fn validate_auth(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    async fn validate_format_data_address(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()> {
        let is_data_address_in_payload = payload.get_data_address().is_some();
        let format = payload.get_format().unwrap(); // in this call there is always format
        let format = format.parse::<DctFormats>().map_err(|_e| {
            let err = CommonErrors::parse_new("Bad format action: Must be push or pull");
            error!("{}", err.log());
            anyhow!(err)
        })?;
        let format_direction = format.action;
        match (is_data_address_in_payload, format_direction) {
            (is_data_address_in_payload, FormatAction::Push) if is_data_address_in_payload == true => Ok(()),
            (is_data_address_in_payload, FormatAction::Pull) if is_data_address_in_payload == false => Ok(()),
            _ => {
                let err = CommonErrors::parse_new("Data address should be defined if format action is push");
                error!("{}", err.log());
                bail!(err);
            }
        }
    }

    async fn validate_data_address_in_start(
        &self,
        payload: &dyn TransferProcessMessageTrait,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<()> {
        let role = dto.inner.role.parse::<RoleConfig>()?;
        let state_attribute =
            dto.inner.state_attribute.clone().unwrap_or("".to_string()).parse::<TransferStateAttribute>()?;
        let is_data_address_in_payload = payload.get_data_address().is_some();

        if is_data_address_in_payload == true {
            if role == RoleConfig::Consumer {
                if state_attribute != TransferStateAttribute::OnRequest {
                    let err = CommonErrors::parse_new(
                        "Data address should be defined only in the first TransferStart message from provider",
                    );
                    error!("{}", err.log());
                    bail!(err);
                }
            }
        }

        Ok(())
    }
}
