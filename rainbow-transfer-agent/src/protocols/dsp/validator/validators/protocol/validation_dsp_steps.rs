use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto,
    TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use std::sync::Arc;

pub struct ValidationDspStepsService {
    payload_validator: Arc<dyn ValidatePayload>,
    step_transition_validator: Arc<dyn ValidateStateTransition>,
    helpers: Arc<dyn ValidationHelpers>,
}
impl ValidationDspStepsService {
    pub fn new(
        payload_validator: Arc<dyn ValidatePayload>,
        step_transition_validator: Arc<dyn ValidateStateTransition>,
        helpers: Arc<dyn ValidationHelpers>,
    ) -> Self {
        Self { payload_validator, step_transition_validator, helpers }
    }
}
#[async_trait::async_trait]
impl ValidationDspSteps for ValidationDspStepsService {
    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<()> {
        self.payload_validator.validate_with_json_schema(&input.dto).await?;
        self.payload_validator.validate_identifiers_as_urn(&input.dto).await?;
        self.payload_validator.validate_auth(&input.dto).await?;
        self.payload_validator.validate_format_data_address(&input.dto).await?;
        Ok(())
    }

    async fn on_transfer_start(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<()> {
        let dto = self.helpers.get_current_dto_from_payload(&input.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = input._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.payload_validator.validate_with_json_schema(&input.dto).await?;
        self.payload_validator.validate_uri_id_as_urn(uri_id).await?;
        self.payload_validator.validate_identifiers_as_urn(&input.dto).await?;
        self.payload_validator.validate_uri_and_pid(uri_id, &input.dto, &role).await?;
        self.payload_validator.validate_correlation(&input.dto, &dto).await?;
        self.payload_validator.validate_auth(&input.dto).await?;
        self.payload_validator.validate_data_address_in_start(&input.dto, &dto).await?;
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

    async fn on_transfer_completion(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<()> {
        let dto = self.helpers.get_current_dto_from_payload(&input.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = input._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.payload_validator.validate_with_json_schema(&input.dto).await?;
        self.payload_validator.validate_uri_id_as_urn(uri_id).await?;
        self.payload_validator.validate_identifiers_as_urn(&input.dto).await?;
        self.payload_validator.validate_uri_and_pid(uri_id, &input.dto, &role).await?;
        self.payload_validator.validate_correlation(&input.dto, &dto).await?;
        self.payload_validator.validate_auth(&input.dto).await?;
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

    async fn on_transfer_suspension(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<()> {
        let dto = self.helpers.get_current_dto_from_payload(&input.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = input._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.payload_validator.validate_with_json_schema(&input.dto).await?;
        self.payload_validator.validate_uri_id_as_urn(uri_id).await?;
        self.payload_validator.validate_identifiers_as_urn(&input.dto).await?;
        self.payload_validator.validate_uri_and_pid(uri_id, &input.dto, &role).await?;
        self.payload_validator.validate_correlation(&input.dto, &dto).await?;
        self.payload_validator.validate_auth(&input.dto).await?;
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

    async fn on_transfer_termination(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<()> {
        let dto = self.helpers.get_current_dto_from_payload(&input.dto).await?;
        let role = self.helpers.get_role_from_dto(&dto).await?;
        let message_type = input._type.clone();
        let current_state = self.helpers.get_state_from_dto(&dto).await?;
        let current_state_attribute = self.helpers.get_state_attribute_from_dto(&dto).await?;
        self.payload_validator.validate_with_json_schema(&input.dto).await?;
        self.payload_validator.validate_uri_id_as_urn(uri_id).await?;
        self.payload_validator.validate_identifiers_as_urn(&input.dto).await?;
        self.payload_validator.validate_uri_and_pid(uri_id, &input.dto, &role).await?;
        self.payload_validator.validate_correlation(&input.dto, &dto).await?;
        self.payload_validator.validate_auth(&input.dto).await?;
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
