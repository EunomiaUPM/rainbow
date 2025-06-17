use crate::contracts_consumer::entities::cn_process;
use crate::contracts_consumer::entities::cn_process::Model;
use rainbow_common::protocol::contract::consumer_process_ack::CnConsumerProcess;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::{ContractNegotiationMessages, ContractNegotiationState};
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult, Clone)]
pub struct CnConsumerProcessFromSQL {
    pub consumer_id: String,
    pub provider_id: Option<String>,
    pub associated_provider: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub message_type: Option<String>,
    pub message_subtype: Option<String>,
}

impl From<CnConsumerProcessFromSQL> for CnConsumerProcess {
    fn from(value: CnConsumerProcessFromSQL) -> Self {
        let state = match value.message_type {
            None => "".to_string(),
            Some(s) => match s.parse::<ContractNegotiationMessages>().unwrap() {
                ContractNegotiationMessages::ContractRequestMessage => ContractNegotiationState::Requested.to_string(),
                ContractNegotiationMessages::ContractOfferMessage => ContractNegotiationState::Offered.to_string(),
                ContractNegotiationMessages::ContractAgreementMessage => ContractNegotiationState::Agreed.to_string(),
                ContractNegotiationMessages::ContractAgreementVerificationMessage => {
                    ContractNegotiationState::Verified.to_string()
                }
                ContractNegotiationMessages::ContractNegotiationEventMessage => {
                    match value.message_subtype.unwrap().as_str() {
                        "accepted" => ContractNegotiationState::Accepted.to_string(),
                        _ => ContractNegotiationState::Finalized.to_string(),
                    }
                }
                ContractNegotiationMessages::ContractNegotiationTerminationMessage => {
                    ContractNegotiationState::Terminated.to_string()
                }
                _ => "".to_string(),
            },
        };
        Self {
            consumer_id: value.consumer_id,
            provider_id: value.provider_id,
            associated_provider: value.associated_provider,
            created_at: value.created_at,
            updated_at: value.updated_at,
            state,
        }
    }
}

impl Into<cn_process::Model> for CnConsumerProcessFromSQL {
    fn into(self) -> Model {
        cn_process::Model {
            consumer_id: self.consumer_id,
            provider_id: self.provider_id,
            associated_provider: self.associated_provider,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<CnConsumerProcessFromSQL> for ContractAckMessage {
    fn from(model: CnConsumerProcessFromSQL) -> Self {
        ContractAckMessage {
            provider_pid: model.provider_id.unwrap(),
            consumer_pid: model.consumer_id,
            ..Default::default()
        }
    }
}
