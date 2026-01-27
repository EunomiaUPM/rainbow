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

use crate::entities::transfer_process::{TransferAgentProcessesTrait, TransferProcessDto};
use crate::protocols::dsp::protocol_types::{
    TransferProcessMessageTrait, TransferProcessState, TransferStateAttribute,
};
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use anyhow::{anyhow, bail};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct ValidationHelperService {
    transfer_process_service: Arc<dyn TransferAgentProcessesTrait>,
}
impl ValidationHelperService {
    pub fn new(transfer_process_service: Arc<dyn TransferAgentProcessesTrait>) -> Self {
        Self { transfer_process_service }
    }
}
#[async_trait::async_trait]
impl ValidationHelpers for ValidationHelperService {
    async fn parse_urn(&self, uri_id: &String) -> anyhow::Result<Urn> {
        Urn::from_str(uri_id.as_str()).map_err(|_e| {
            let err = CommonErrors::parse_new("Invalid URN URN. The URN URN is malformed.");
            error!("{}", err.log());
            anyhow::anyhow!(err)
        })
    }

    async fn parse_identifier_into_role(&self, identifier: &str) -> anyhow::Result<RoleConfig> {
        match identifier {
            "consumerPid" => Ok(RoleConfig::Consumer),
            "providerPid" => Ok(RoleConfig::Provider),
            _ => {
                let err = CommonErrors::parse_new(
                    "Not a valid DSP identifiers. Please use 'consumerPid' or 'providerPid'.",
                );
                error!("{}", err.log());
                bail!(err);
            }
        }
    }

    async fn parse_role_into_identifier(&self, role: &RoleConfig) -> anyhow::Result<&str> {
        match role {
            RoleConfig::Provider => Ok("providerPid"),
            RoleConfig::Consumer => Ok("consumerPid"),
            _ => {
                let err = CommonErrors::parse_new(
                    "Not a valid DSP identifiers. Please use 'consumerPid' or 'providerPid'.",
                );
                error!("{}", err.log());
                bail!(err);
            }
        }
    }

    async fn get_current_dto_from_payload(
        &self,
        payload: &dyn TransferProcessMessageTrait,
    ) -> anyhow::Result<TransferProcessDto> {
        let consumer_pid = payload.get_consumer_pid().ok_or_else(|| {
            let err =
                CommonErrors::parse_new("Not a valid DSP payload, consumer_pid is mandatory.");
            error!("{}", err.log());
            anyhow!(err)
        })?;
        let dto = self
            .transfer_process_service
            .get_transfer_process_by_key_value(&consumer_pid)
            .await
            .map(Some)
            .or_else(|e| {
                if let Some(common_err) = e.downcast_ref::<CommonErrors>() {
                    if matches!(common_err, CommonErrors::MissingResourceError { .. }) {
                        return Ok(None);
                    }
                }
                Err(e)
            })?
            .ok_or_else(|| {
                let err = CommonErrors::parse_new("A dto should be available at this point");
                error!("{}", err.log());
                anyhow!(err)
            })?;
        Ok(dto)
    }

    async fn get_pid_by_role(
        &self,
        dto: &TransferProcessDto,
        role: RoleConfig,
    ) -> anyhow::Result<Urn> {
        let role_as_identifier = self.parse_role_into_identifier(&role).await?;
        let pid = dto.identifiers.get(role_as_identifier).ok_or_else(|| {
            let err = CommonErrors::parse_new("There is no such a identifier, role is mandatory.");
            error!("{}", err.log());
            anyhow!(err)
        })?;
        let urn = self.parse_urn(pid).await?;
        Ok(urn)
    }

    async fn get_role_from_dto(&self, dto: &TransferProcessDto) -> anyhow::Result<RoleConfig> {
        let role = &dto.inner.role;
        let role = role.parse::<RoleConfig>()?;
        Ok(role)
    }

    async fn get_state_from_dto(
        &self,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<TransferProcessState> {
        let state = &dto.inner.state;
        let state = state.parse::<TransferProcessState>().map_err(|_e| {
            let err = CommonErrors::parse_new(
                "Something is wrong. Seems this process' state is not protocol compliant",
            );
            log::error!("{}", err.log());
            err
        })?;
        Ok(state)
    }

    async fn get_state_attribute_from_dto(
        &self,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<TransferStateAttribute> {
        let state_attribute = dto
            .inner
            .state_attribute
            .clone()
            .unwrap_or(TransferStateAttribute::OnRequest.to_string())
            .parse::<TransferStateAttribute>()?;
        Ok(state_attribute)
    }
}
