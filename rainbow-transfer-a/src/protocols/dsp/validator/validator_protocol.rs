use crate::entities::transfer_process::{TransferAgentProcessesTrait, TransferProcessDto};
use crate::protocols::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageType};
use crate::protocols::dsp::validator::{helpers, ValidatorTrait};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;
use urn::Urn;

pub struct ValidatorProtocolService {
    transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
}

// 1. Validate provider_pid or consumer_pid in message body matches those from URI
// 2. Validate provider_pid consumer_pid correlation
// 4. Validate process is correlated with mate

impl ValidatorProtocolService {
    pub fn new(transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>) -> Self {
        Self { transfer_agent_process_entities }
    }
    async fn validate_uri_and_pid(&self, id: &Urn, dto: &TransferProcessDto) -> anyhow::Result<()> {
        debug!("Validating transfer process payload: validate_uri_and_pid");
        let role = &dto.inner.role;
        let role_key = helpers::validate_role(role)?;
        let pid = helpers::get_pid_from_identifiers(dto, role_key)?;
        helpers::validate_pid_match(&pid, id)?;
        Ok(())
    }
    fn validate_mate_association(
        &self,
        _id: Option<&String>,
        _payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
        debug!("Validating transfer process payload: validate_mate_association");
        Ok(())
    }
    fn validate_consumer_provider_pids_correlation(
        &self,
        payload: Arc<dyn TransferProcessMessageTrait>,
        dto: &TransferProcessDto,
    ) -> anyhow::Result<()> {
        debug!("Validating transfer process payload: validate_consumer_provider_pids_correlation");
        let provider_pid = &payload.get_provider_pid().unwrap().to_string(); // both must be
        let consumer_pid = &payload.get_consumer_pid().unwrap().to_string(); // both must be
        let provider_pid_in_dto = helpers::get_pid_from_identifiers(dto, "providerPid")?;
        let consumer_pid_in_dto = helpers::get_pid_from_identifiers(dto, "consumerPid")?;
        helpers::validate_pids_correlation(
            provider_pid,
            consumer_pid,
            &provider_pid_in_dto,
            &consumer_pid_in_dto,
        )?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl ValidatorTrait for ValidatorProtocolService {
    async fn validate(&self, id: Option<&String>, payload: Arc<dyn TransferProcessMessageTrait>) -> anyhow::Result<()> {
        debug!("Validating transfer process payload");
        let message_type = payload.get_message();
        match message_type {
            TransferProcessMessageType::TransferRequestMessage => {}
            TransferProcessMessageType::TransferProcess => {} // caught in state transition
            TransferProcessMessageType::TransferError => {}   // caught in state transition
            _ => {
                // id must be urn
                let id_as_urn = Urn::from_str(id.unwrap().as_str()).map_err(|_e| {
                    let err = CommonErrors::parse_new("Invalid URN identifier");
                    error!("{}", err);
                    err
                })?;
                // there must be process
                let current_process = self
                    .transfer_agent_process_entities
                    .get_transfer_process_by_key_value(&id_as_urn)
                    .await
                    .map(Some)?
                    .ok_or_else(|| {
                        let err = CommonErrors::missing_resource_new(
                            id_as_urn.to_string().as_str(),
                            format!("Process not found {}", &id_as_urn).as_str(),
                        );
                        error!("{}", err.log());
                        err
                    })?;
                // uri and pid must coincide
                self.validate_uri_and_pid(&id_as_urn, &current_process).await?;
                // consumer_pid and provider_pid in request must coincide with transfer_process_dto_identifiers
                self.validate_consumer_provider_pids_correlation(payload.clone(), &current_process)?;
            }
        };
        // check mate correlation
        self.validate_mate_association(id, payload.clone())?;
        Ok(())
    }
}
