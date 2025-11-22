use axum::{
    extract::{rejection::JsonRejection, FromRef, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use super::protocol_types::{
    TransferCompletionMessageDto, TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto,
    TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::core::dsp::orchestrator::OrchestratorTrait;
use crate::http::common::extract_payload;
use rainbow_common::config::provider_config::ApplicationProviderConfig;

#[derive(Clone)]
pub struct DspRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl FromRef<DspRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &DspRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl FromRef<DspRouter> for Arc<ApplicationProviderConfig> {
    fn from_ref(state: &DspRouter) -> Self {
        state.config.clone()
    }
}

impl DspRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { orchestrator: service, config }
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
    async fn handle_get_transfer_process(State(state): State<DspRouter>, Path(id): Path<String>) -> impl IntoResponse {
        match state.orchestrator.get_protocol_service().on_get_transfer_process(&id).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }

    /// 10.2.2 Transfer Request Endpoint - POST /transfers/request
    async fn handle_transfer_request(
        State(state): State<DspRouter>,
        input: Result<Json<TransferProcessMessageWrapper<TransferRequestMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.orchestrator.get_protocol_service().on_transfer_request(&input).await {
            Ok(process) => (StatusCode::CREATED, Json(process)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }

    /// 10.2.3 Transfer Start Endpoint - POST /transfers/:providerPid/start
    async fn handle_transfer_start(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferStartMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.orchestrator.get_protocol_service().on_transfer_start(&id, &input).await {
            Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }

    /// 10.2.4 Transfer Completion Endpoint - POST /transfers/:providerPid/completion
    async fn handle_transfer_completion(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferCompletionMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.orchestrator.get_protocol_service().on_transfer_completion(&id, &input).await {
            Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }

    /// 10.2.5 Transfer Termination Endpoint - POST /transfers/:providerPid/termination
    async fn handle_transfer_termination(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferTerminationMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.orchestrator.get_protocol_service().on_transfer_termination(&id, &input).await {
            Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }

    /// 10.2.6 Transfer Suspension Endpoint - POST /transfers/:providerPid/suspension
    async fn handle_transfer_suspension(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferSuspensionMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.orchestrator.get_protocol_service().on_transfer_suspension(&id, &input).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }
}
