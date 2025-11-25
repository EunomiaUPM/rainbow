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
