use crate::core::dsp::protocol_types::{
    TransferProcessAckDto, TransferProcessMessageType, TransferProcessMessageWrapper, TransferProcessState,
};
use crate::db::entities::transfer_process::Model;
use crate::entities::transfer_process::TransferProcessDto;
use anyhow::bail;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::context_field::ContextField;
use std::str::FromStr;
use tracing::error;
use urn::Urn;

impl TryFrom<TransferProcessDto> for TransferProcessMessageWrapper<TransferProcessAckDto> {
    type Error = anyhow::Error;

    fn try_from(value: TransferProcessDto) -> Result<Self, Self::Error> {
        let consumer_str = match value.identifiers.get("consumerPid") {
            Some(val) => val,
            None => {
                let err =
                    CommonErrors::parse_new("Critical: Missing 'consumerPid' in TransferProcessDto identifiers map");
                error!("{}", err.log());
                bail!(err);
            }
        };
        let consumer_pid = match Urn::from_str(consumer_str) {
            Ok(urn) => urn,
            Err(e) => {
                let err = CommonErrors::parse_new(&format!(
                    "Critical: Invalid URN format for consumerPid '{}': {}",
                    consumer_str, e
                ));
                error!("{}", err.log());
                bail!(err);
            }
        };

        let provider_str = match value.identifiers.get("providerPid") {
            Some(val) => val,
            None => {
                let err =
                    CommonErrors::parse_new("Critical: Missing 'providerPid' in TransferProcessDto identifiers map");
                error!("{}", err.log());
                bail!(err);
            }
        };
        let provider_pid = match Urn::from_str(provider_str) {
            Ok(urn) => urn,
            Err(e) => {
                let err = CommonErrors::parse_new(&format!(
                    "Critical: Invalid URN format for providerPid '{}': {}",
                    provider_str, e
                ));
                error!("{}", err.log());
                bail!(err);
            }
        };

        let state = match value.inner.state.parse::<TransferProcessState>() {
            Ok(s) => s,
            Err(_) => {
                let err = CommonErrors::parse_new(&format!(
                    "Critical: Invalid or unknown TransferProcessState '{}' in database model",
                    value.inner.state
                ));
                error!("{}", err.log());
                bail!(err);
            }
        };

        Ok(Self {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferProcess,
            dto: TransferProcessAckDto { consumer_pid, provider_pid, state },
        })
    }
}

// TODO mapper for CommonErrors for TransferProcessErrorDto
