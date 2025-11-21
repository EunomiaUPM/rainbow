use axum::{
    extract::{rejection::JsonRejection, FromRef, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tracing::{error, info};
use urn::Urn;

use crate::errors::error_adapter::CustomToResponse;
use crate::http::common::{extract_payload, parse_urn};
use rainbow_common::config::provider_config::ApplicationProviderConfig;

use super::protocol::{
    TransferCompletionMessageDto, TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto,
    TransferTerminationMessageDto,
};

#[derive(Clone)]
pub struct TransferAgentTransfersRouter {
    service: Arc<dyn TransferAgentTransfersTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl FromRef<TransferAgentTransfersRouter> for Arc<dyn TransferAgentTransfersTrait> {
    fn from_ref(state: &TransferAgentTransfersRouter) -> Self {
        state.service.clone()
    }
}

impl FromRef<TransferAgentTransfersRouter> for Arc<ApplicationProviderConfig> {
    fn from_ref(state: &TransferAgentTransfersRouter) -> Self {
        state.config.clone()
    }
}

impl TransferAgentTransfersRouter {
    pub fn new(service: Arc<dyn TransferAgentTransfersTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { service, config }
    }

    pub fn router(self) -> Router {
        Router::new()
            // 10.2.2 Transfer Request Endpoint
            .route("/request", post(Self::handle_transfer_request))
            // 10.2.1 Transfer Process Endpoint
            .route("/:id", get(Self::handle_get_transfer_process))
            // 10.2.3 Transfer Start Endpoint
            .route("/:id/start", post(Self::handle_transfer_start))
            // 10.2.4 Transfer Completion Endpoint
            .route("/:id/completion", post(Self::handle_transfer_completion))
            // 10.2.5 Transfer Termination Endpoint
            .route("/:id/termination", post(Self::handle_transfer_termination))
            // 10.2.6 Transfer Suspension Endpoint
            .route("/:id/suspension", post(Self::handle_transfer_suspension))
            .with_state(self)
    }

    /// 10.2.1 Transfer Process Endpoint - GET /transfers/:providerPid
    async fn handle_get_transfer_process(
        State(state): State<TransferAgentTransfersRouter>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };

        match state.service.get_transfer_process_by_id(&id_urn).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    /// 10.2.2 Transfer Request Endpoint - POST /transfers/request
    async fn handle_transfer_request(
        State(state): State<TransferAgentTransfersRouter>,
        input: Result<Json<TransferRequestMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };

        match state.service.initiate_transfer_process(&input).await {
            Ok(process) => (StatusCode::CREATED, Json(process)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    /// 10.2.3 Transfer Start Endpoint - POST /transfers/:providerPid/start
    async fn handle_transfer_start(
        State(state): State<TransferAgentTransfersRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferStartMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };

        match state.service.start_transfer_process(&id_urn, &input).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(err) => err.to_response(),
        }
    }

    /// 10.2.4 Transfer Completion Endpoint - POST /transfers/:providerPid/completion
    async fn handle_transfer_completion(
        State(state): State<TransferAgentTransfersRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferCompletionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };

        match state.service.complete_transfer_process(&id_urn, &input).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(err) => err.to_response(),
        }
    }

    /// 10.2.5 Transfer Termination Endpoint - POST /transfers/:providerPid/termination
    async fn handle_transfer_termination(
        State(state): State<TransferAgentTransfersRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferTerminationMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };

        match state.service.terminate_transfer_process(&id_urn, &input).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(err) => err.to_response(),
        }
    }

    /// 10.2.6 Transfer Suspension Endpoint - POST /transfers/:providerPid/suspension
    async fn handle_transfer_suspension(
        State(state): State<TransferAgentTransfersRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferSuspensionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };

        match state.service.suspend_transfer_process(&id_urn, &input).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(err) => err.to_response(),
        }
    }
}
