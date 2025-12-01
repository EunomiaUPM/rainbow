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

#![allow(unused)]
use crate::entities::transfer_process::TransferProcessDto;
use crate::protocols::dsp::protocol_types::{
    TransferProcessMessageTrait, TransferProcessState, TransferStateAttribute,
};
use rainbow_common::protocol::transfer::TransferRoles;
use urn::Urn;

#[async_trait::async_trait]
pub trait ValidationHelpers: Send + Sync + 'static {
    async fn parse_urn(&self, uri_id: &String) -> anyhow::Result<Urn>;
    async fn parse_identifier_into_role(&self, identifier: &str) -> anyhow::Result<TransferRoles>;
    async fn parse_role_into_identifier(&self, role: &TransferRoles) -> anyhow::Result<&str>;
    async fn get_current_dto_from_payload(
        &self,
        payload: &dyn TransferProcessMessageTrait,
    ) -> anyhow::Result<TransferProcessDto>;
    async fn get_pid_by_role(&self, dto: &TransferProcessDto, role: TransferRoles) -> anyhow::Result<Urn>;
    async fn get_role_from_dto(&self, dto: &TransferProcessDto) -> anyhow::Result<TransferRoles>;
    async fn get_state_from_dto(&self, dto: &TransferProcessDto) -> anyhow::Result<TransferProcessState>;
    async fn get_state_attribute_from_dto(&self, dto: &TransferProcessDto) -> anyhow::Result<TransferStateAttribute>;
}
