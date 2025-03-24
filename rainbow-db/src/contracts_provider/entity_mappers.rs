use crate::contracts_provider::entities::cn_process;
use crate::contracts_provider::repo::CnErrors;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;

impl From<cn_process::Model> for ContractAckMessage {
    fn from(model: cn_process::Model) -> Self {
        ContractAckMessage {
            provider_pid: model.provider_id.unwrap(),
            consumer_pid: model.consumer_id.unwrap(),
            state: model.state.parse().unwrap(),
            ..Default::default()
        }
    }
}

impl From<CnErrors> for ContractErrorMessage {
    fn from(error: CnErrors) -> Self {
        match error {
            e @ CnErrors::CNProcessNotFound => Self {
                code: Some("NOT_FOUND".to_string()),
                reason: Some(vec![e.to_string()]),
                ..Default::default()
            },
            e => Self {
                code: Some("DB_ERROR".to_string()),
                reason: Some(vec![e.to_string()]),
                ..Default::default()
            }
        }
    }
}