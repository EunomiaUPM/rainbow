use std::str::FromStr;
use chrono::DateTime;
use tonic::Status;
use serde_json::Value as JsonValue;
use urn::Urn;
use crate::entities::transfer_messages::{NewTransferMessageDto, TransferMessageDto};
use crate::grpc::api::transfer_messages::{CreateMessageRequest, TransferMessageResponse, PaginationRequestMessages};
use crate::http::transfer_messages::PaginationParams; // Tu struct de params
impl TryFrom<CreateMessageRequest> for NewTransferMessageDto {
    type Error = Status;

    fn try_from(proto: CreateMessageRequest) -> Result<Self, Self::Error> {
        let process_urn = Urn::from_str(&proto.transfer_agent_process_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid Process URN: {}", e)))?;
        let id_urn = if let Some(id_str) = proto.id {
            Some(Urn::from_str(&id_str)
                .map_err(|e| Status::invalid_argument(format!("Invalid Message URN: {}", e)))?)
        } else {
            None
        };
        let payload: Option<JsonValue> = if let Some(json_str) = proto.payload_json {
            if json_str.trim().is_empty() {
                None
            } else {
                Some(serde_json::from_str(&json_str).map_err(|e| {
                    Status::invalid_argument(format!("Invalid JSON payload: {}", e))
                })?)
            }
        } else {
            None
        };

        Ok(NewTransferMessageDto {
            id: id_urn,
            transfer_agent_process_id: process_urn, // Asumiendo que tu DTO usa TransferUrn
            direction: proto.direction,
            protocol: proto.protocol,
            message_type: proto.message_type,
            state_transition_from: proto.state_transition_from,
            state_transition_to: proto.state_transition_to,
            payload,
        })
    }
}

impl From<PaginationRequestMessages> for PaginationParams {
    fn from(proto: PaginationRequestMessages) -> Self {
        Self {
            limit: proto.limit,
            page: proto.page,
        }
    }
}


impl From<TransferMessageDto> for TransferMessageResponse {
    fn from(dto: TransferMessageDto) -> Self {
        let model = dto.inner; // Accedemos al modelo de SeaORM interno
        let payload_json = model.payload
            .map(|json| serde_json::to_string(&json).unwrap_or_default());
        let created_at = to_prost_timestamp(DateTime::from(model.created_at));

        Self {
            id: model.id.to_string(),
            transfer_agent_process_id: model.transfer_agent_process_id.to_string(),
            direction: model.direction,
            protocol: model.protocol,
            state_transition_from: model.state_transition_from,
            message_type: model.message_type,
            state_transition_to: model.state_transition_to,
            payload_json,
            created_at: Some(created_at),
            updated_at: None,
        }
    }
}

fn to_prost_timestamp(dt: chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
