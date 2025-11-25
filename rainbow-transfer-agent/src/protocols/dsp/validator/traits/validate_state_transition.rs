use crate::protocols::dsp::protocol_types::{TransferProcessMessageType, TransferProcessState, TransferStateAttribute};
use rainbow_common::protocol::transfer::TransferRoles;

#[async_trait::async_trait]
pub trait ValidateStateTransition: Send + Sync + 'static {
    // validate role for message type
    // provider can receive: [request, start, suspension, completion, termination]
    // consumer can receive: [start, suspension, completion, termination]
    async fn validate_role_for_message(
        &self,
        role: &TransferRoles,
        message_type: &TransferProcessMessageType,
    ) -> anyhow::Result<()>;
    // validate state transition from state a to b based in DSP state machine
    async fn validate_state_transition(
        &self,
        current_state: &TransferProcessState,
        message_type: &TransferProcessMessageType,
    ) -> anyhow::Result<()>;
    // logical semaphore for avoiding consumer to start provider's suspension and viceversa
    async fn validate_state_attribute_transition(
        &self,
        current_state: &TransferProcessState,
        current_state_attribute: &TransferStateAttribute,
        message_type: &TransferProcessMessageType,
        role: &TransferRoles,
    ) -> anyhow::Result<()>;
}
