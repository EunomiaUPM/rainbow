#![allow(unused)]
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto,
    TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use anyhow::bail;
use std::sync::Arc;

pub struct ValidationRpcStepsService {
    payload_validator: Arc<dyn ValidatePayload>,
    step_transition_validator: Arc<dyn ValidateStateTransition>,
    helpers: Arc<dyn ValidationHelpers>,
}
impl ValidationRpcStepsService {
    pub fn new(
        payload_validator: Arc<dyn ValidatePayload>,
        step_transition_validator: Arc<dyn ValidateStateTransition>,
        helpers: Arc<dyn ValidationHelpers>,
    ) -> Self {
        Self { payload_validator, step_transition_validator, helpers }
    }
}

#[async_trait::async_trait]
impl ValidationRpcSteps for ValidationRpcStepsService {
    async fn transfer_request_rpc(&self, input: &RpcTransferRequestMessageDto) -> anyhow::Result<()> {
        let request_body: TransferProcessMessageWrapper<TransferRequestMessageDto> = input.clone().into();
        self.payload_validator.validate_format_data_address(&request_body.dto).await?;
        Ok(())
    }

    async fn transfer_start_rpc(&self, input: &RpcTransferStartMessageDto) -> anyhow::Result<()> {
        let request_body: TransferProcessMessageWrapper<TransferStartMessageDto> = input.clone().into();
        let dto = self.helpers.get_current_dto_from_payload(&request_body.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = request_body._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.payload_validator.validate_data_address_in_start(&request_body.dto, &dto).await?;
        self.step_transition_validator.validate_role_for_message(&role, &message_type).await?;
        self.step_transition_validator.validate_state_transition(&current_state, &message_type).await?;
        self.step_transition_validator
            .validate_state_attribute_transition(
                &current_state,
                &current_state_attribute,
                &message_type,
                &role,
            )
            .await?;
        Ok(())
    }

    async fn transfer_completion_rpc(&self, input: &RpcTransferCompletionMessageDto) -> anyhow::Result<()> {
        let request_body: TransferProcessMessageWrapper<TransferCompletionMessageDto> = input.clone().into();
        let dto = self.helpers.get_current_dto_from_payload(&request_body.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = request_body._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.step_transition_validator.validate_role_for_message(&role, &message_type).await?;
        self.step_transition_validator.validate_state_transition(&current_state, &message_type).await?;
        self.step_transition_validator
            .validate_state_attribute_transition(
                &current_state,
                &current_state_attribute,
                &message_type,
                &role,
            )
            .await?;
        Ok(())
    }

    async fn transfer_suspension_rpc(&self, input: &RpcTransferSuspensionMessageDto) -> anyhow::Result<()> {
        let request_body: TransferProcessMessageWrapper<TransferSuspensionMessageDto> = input.clone().into();
        let dto = self.helpers.get_current_dto_from_payload(&request_body.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = request_body._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.step_transition_validator.validate_role_for_message(&role, &message_type).await?;
        self.step_transition_validator.validate_state_transition(&current_state, &message_type).await?;
        self.step_transition_validator
            .validate_state_attribute_transition(
                &current_state,
                &current_state_attribute,
                &message_type,
                &role,
            )
            .await?;
        Ok(())
    }

    async fn transfer_termination_rpc(&self, input: &RpcTransferTerminationMessageDto) -> anyhow::Result<()> {
        let request_body: TransferProcessMessageWrapper<TransferTerminationMessageDto> = input.clone().into();
        let dto = self.helpers.get_current_dto_from_payload(&request_body.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = request_body._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.step_transition_validator.validate_role_for_message(&role, &message_type).await?;
        self.step_transition_validator.validate_state_transition(&current_state, &message_type).await?;
        self.step_transition_validator
            .validate_state_attribute_transition(
                &current_state,
                &current_state_attribute,
                &message_type,
                &role,
            )
            .await?;
        Ok(())
    }
}
