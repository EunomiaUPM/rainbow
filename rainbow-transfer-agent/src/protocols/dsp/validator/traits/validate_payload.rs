use crate::entities::transfer_process::TransferProcessDto;
use crate::protocols::dsp::protocol_types::TransferProcessMessageTrait;
use rainbow_common::protocol::transfer::TransferRoles;

#[async_trait::async_trait]
pub trait ValidatePayload: Send + Sync + 'static {
    /// Validates with json schema
    async fn validate_with_json_schema(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()>;
    /// Validates uri in URL to check if it is URN encoded
    async fn validate_uri_id_as_urn(&self, uri_id: &String) -> anyhow::Result<()>;
    /// Validates if identifiers provider_pid and consumer_pid are urn
    async fn validate_identifiers_as_urn(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()>;
    /// Validates depending on role if uri_id == ***_pid
    async fn validate_uri_and_pid(
        &self,
        uri_id: &String,
        payload: &dyn TransferProcessMessageTrait,
        role: &TransferRoles,
    ) -> anyhow::Result<()>;
    /// Validates if consumer_pid and provider_pid are equal to identifiers in db
    async fn validate_correlation(
        &self,
        payload: &dyn TransferProcessMessageTrait,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<()>; // db call
    /// Validates if Header Bearer token corresponds to associated_consumer in db
    async fn validate_auth(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()>; // db call
    /// Validates if data_address_present if format contains PUSH
    async fn validate_format_data_address(&self, payload: &dyn TransferProcessMessageTrait) -> anyhow::Result<()>;
    /// Validates if data_address_present if format contains PUSH
    async fn validate_data_address_in_start(
        &self,
        payload: &dyn TransferProcessMessageTrait,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<()>;
}
