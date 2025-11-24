use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageType};
use crate::core::dsp::validator::ValidatorTrait;
use crate::entities::transfer_process::{TransferAgentProcessesTrait, TransferProcessDto};
use anyhow::{anyhow, bail};
use log::error;
use rainbow_common::errors::helpers::BadFormat;
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
        let role_key = match role.as_str() {
            "Provider" => Ok("providerPid"),
            "Consumer" => Ok("consumerPid"),
            role => {
                let err = CommonErrors::format_new(
                    BadFormat::Received,
                    format!("No role expected: {}", role).as_str(),
                );
                error!("{}", err.log());
                Err(err)
            }
        }?;
        let pid = dto.identifiers.get(role_key).ok_or_else(|| {
            let err = CommonErrors::missing_resource_new(
                role_key.to_string().as_str(),
                format!("Identifiers not found {}", &role_key).as_str(),
            );
            error!("{}", err.log());
            anyhow!(err)
        })?;
        let pid_as_urn = Urn::from_str(pid)?;
        if pid_as_urn.to_string() != id.to_string() {
            let err = CommonErrors::format_new(
                BadFormat::Received,
                "Body and Uri identifiers do not coincide",
            );
            error!("{}", err.log());
            anyhow!(err);
        }

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
        let provider_pid_in_dto = dto.identifiers.get("providerPid").ok_or_else(|| {
            let err = CommonErrors::format_new(
                BadFormat::Received,
                "Identifiers providerPid not found",
            );
            error!("{}", err.log());
            anyhow!(err)
        })?;
        let consumer_pid_in_dto = dto.identifiers.get("consumerPid").ok_or_else(|| {
            let err = CommonErrors::format_new(
                BadFormat::Received,
                "Identifiers providerPid not found",
            );
            error!("{}", err.log());
            anyhow!(err)
        })?;
        if provider_pid != provider_pid_in_dto || consumer_pid != consumer_pid_in_dto {
            let err = CommonErrors::format_new(
                BadFormat::Received,
                "ConsumerPid or providerPid not coincide with transfer process identifiers",
            );
            error!("{}", err.log());
            anyhow!(err);
        }
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
                let id_as_urn = id.map(|id| Urn::from_str(&id).unwrap()).ok_or_else(|| {
                    let err = CommonErrors::format_new(BadFormat::Received, "Invalid transfer process message ID");
                    error!("{}", err.log());
                    anyhow!(err)
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
                        anyhow!(err)
                    })?;
                self.validate_uri_and_pid(&id_as_urn, &current_process).await?;
                self.validate_consumer_provider_pids_correlation(payload.clone(), &current_process)?;
            }
        };
        // check mate correlation
        self.validate_mate_association(id, payload.clone())?;
        Ok(())
    }
}
