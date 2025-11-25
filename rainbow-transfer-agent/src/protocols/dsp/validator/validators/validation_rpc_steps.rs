#![allow(unused)]
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
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
        Ok(())
    }

    async fn transfer_start_rpc(&self, input: &RpcTransferStartMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn transfer_completion_rpc(&self, input: &RpcTransferCompletionMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn transfer_suspension_rpc(&self, input: &RpcTransferSuspensionMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn transfer_termination_rpc(&self, input: &RpcTransferTerminationMessageDto) -> anyhow::Result<()> {
        Ok(())
    }
}
