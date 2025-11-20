use std::collections::HashMap;
use std::str::FromStr;
use chrono::DateTime;
use tonic::Status;
use serde_json::Value as JsonValue;
use urn::Urn;
use rainbow_common::batch_requests::BatchRequests;
// Dominio
use crate::entities::transfer_process::{
    NewTransferProcessDto, EditTransferProcessDto, TransferProcessDto
};
use crate::grpc::api::transfer_processes::{BatchProcessRequest, CreateProcessRequest, TransferProcessResponse, UpdateProcessRequest,PaginationRequestProcesses};
use crate::grpc::api::transfer_messages::{TransferMessageResponse};

// -----------------------------------------------------------------------------
// PROTO -> DOMAIN (TryFrom)
// -----------------------------------------------------------------------------

impl TryFrom<CreateProcessRequest> for NewTransferProcessDto {
    type Error = Status;

    fn try_from(proto: CreateProcessRequest) -> Result<Self, Self::Error> {
        // 1. URNs
        let agreement_urn = Urn::from_str(&proto.agreement_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid Agreement URN: {}", e)))?;

        let id_urn = if let Some(id) = proto.id {
            Some(Urn::from_str(&id)
                .map_err(|e| Status::invalid_argument(format!("Invalid Process URN: {}", e)))?)
        } else {
            None
        };

        // 2. JSON Properties
        let properties = if let Some(json_str) = proto.properties_json {
            Some(serde_json::from_str(&json_str).map_err(|e| {
                Status::invalid_argument(format!("Invalid properties JSON: {}", e))
            })?)
        } else {
            None
        };

        // 3. Identifiers (Map<String, String> -> HashMap<String, String>)
        // Proto genera HashMap automáticamente, pero si es vacío queremos None?
        let identifiers = if proto.identifiers.is_empty() {
            None
        } else {
            Some(proto.identifiers) // Rust HashMap es compatible
        };

        Ok(NewTransferProcessDto {
            id: id_urn,
            state: proto.state,
            associated_agent_peer: proto.associated_agent_peer,
            protocol: proto.protocol,
            transfer_direction: proto.transfer_direction,
            agreement_id: agreement_urn,
            state_attribute: proto.state_attribute,
            properties,
            identifiers,
        })
    }
}

impl TryFrom<UpdateProcessRequest> for EditTransferProcessDto {
    type Error = Status;

    fn try_from(proto: UpdateProcessRequest) -> Result<Self, Self::Error> {
        // Parsing JSONs opcionales
        let properties = parse_optional_json(proto.properties_json)?;
        let error_details = parse_optional_json(proto.error_details_json)?;

        let identifiers = if proto.identifiers.is_empty() {
            None
        } else {
            Some(proto.identifiers)
        };

        Ok(EditTransferProcessDto {
            state: proto.state,
            state_attribute: proto.state_attribute,
            properties,
            error_details,
            identifiers,
        })
    }
}

impl From<BatchProcessRequest> for BatchRequests {
    fn from(proto: BatchProcessRequest) -> Self {
        BatchRequests {
            ids: proto.ids.iter().map(|i| Urn::from_str(i).unwrap()).collect()
        }
    }
}

// -----------------------------------------------------------------------------
// DOMAIN -> PROTO (From)
// -----------------------------------------------------------------------------

impl From<TransferProcessDto> for TransferProcessResponse {
    fn from(dto: TransferProcessDto) -> Self {
        let model = dto.inner;

        let properties_json = serde_json::to_string(&model.properties).unwrap_or_default();
        let error_details_json = model.error_details
            .map(|j| serde_json::to_string(&j).unwrap_or_default());

        // 2. Fechas
        let created_at = to_prost_timestamp(DateTime::from(model.created_at));
        let updated_at = model.updated_at.map(|d|to_prost_timestamp(DateTime::from(model.created_at)));

        // 3. Mensajes Anidados
        // Aquí asumimos que ya implementaste From<TransferMessageDto> for TransferMessageResponse
        // en el paso anterior. Necesitamos convertir el modelo DB a DTO intermedio o mapear directo.
        // Para simplificar, mapearemos el modelo DB de mensaje al Proto Response de mensaje.
        // NOTA: Esto requiere que tengas un From<transfer_message_model::Model> for TransferMessageResponse
        // Si no, iteramos y construimos manualmente o usamos el DTO intermedio.

        // Asumiremos que existe el mapeo desde el DTO de mensaje:
        let messages_proto: Vec<TransferMessageResponse> = dto.messages.into_iter().map(|msg_model| {
            // Hack rápido: convertir model a DTO para reusar el mapper existente
            // O crear un mapper directo Model -> Proto
            let msg_dto = crate::entities::transfer_messages::TransferMessageDto { inner: msg_model };
            msg_dto.into()
        }).collect();

        Self {
            id: model.id,
            state: model.state,
            state_attribute: model.state_attribute,
            associated_agent_peer: model.associated_agent_peer,
            protocol: model.protocol,
            transfer_direction: model.transfer_direction,
            agreement_id: model.agreement_id,

            properties_json,
            error_details_json,

            created_at: created_at.into(),
            updated_at,

            identifiers: dto.identifiers, // HashMap directo
            messages: messages_proto,
        }
    }
}

// Helpers
fn parse_optional_json(input: Option<String>) -> Result<Option<JsonValue>, Status> {
    match input {
        Some(s) if !s.trim().is_empty() => {
            serde_json::from_str(&s).map_err(|e| Status::invalid_argument(format!("Invalid JSON: {}", e))).map(Some)
        },
        _ => Ok(None),
    }
}

fn to_prost_timestamp(dt: chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}