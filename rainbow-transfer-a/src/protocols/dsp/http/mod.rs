use axum::{
    extract::{rejection::JsonRejection, FromRef, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::future::Future;
use std::sync::Arc;

use super::protocol_types::{
    TransferCompletionMessageDto, TransferErrorDto, TransferProcessMessageType, TransferProcessMessageWrapper,
    TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::protocols::dsp::errors::extract_payload_error;
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::CommonErrors;
use rainbow_common::protocol::context_field::ContextField;

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
            .route("/request", post(Self::handle_transfer_request))
            .route(
                "/rpc/setup-request",
                post(Self::handle_transfer_request_rpc),
            )
            .route("/:id", get(Self::handle_get_transfer_process))
            .route("/:id/start", post(Self::handle_transfer_start))
            .route("/rpc/setup-start", post(Self::handle_transfer_start_rpc))
            .route("/:id/completion", post(Self::handle_transfer_completion))
            .route(
                "/rpc/setup-completion",
                post(Self::handle_transfer_completion_rpc),
            )
            .route("/:id/termination", post(Self::handle_transfer_termination))
            .route(
                "/rpc/setup-termination",
                post(Self::handle_transfer_termination_rpc),
            )
            .route("/:id/suspension", post(Self::handle_transfer_suspension))
            .route(
                "/rpc/setup-suspension",
                post(Self::handle_transfer_suspension_rpc),
            )
            .with_state(self)
    }

    async fn process_request<T, R, F, Fut>(
        input: Result<Json<T>, JsonRejection>,
        success_code: StatusCode,
        action: F,
    ) -> impl IntoResponse
    where
        T: Send,
        R: Serialize,
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = anyhow::Result<R>> + Send,
    {
        let payload = match extract_payload_error(input) {
            Ok(v) => v,
            Err(e) => {
                let error_dto: TransferProcessMessageWrapper<TransferErrorDto> = e.into();
                return (StatusCode::BAD_REQUEST, Json(error_dto)).into_response();
            }
        };
        Self::map_service_result(action(payload).await, success_code).into_response()
    }

    fn map_service_result<R>(result: anyhow::Result<R>, success_code: StatusCode) -> impl IntoResponse
    where
        R: Serialize,
    {
        match result {
            Ok(data) => (success_code, Json(data)).into_response(),
            Err(err) => match err.downcast::<CommonErrors>() {
                Ok(common_errors) => {
                    let error_dto: TransferProcessMessageWrapper<TransferErrorDto> = common_errors.into();
                    (StatusCode::BAD_REQUEST, Json(error_dto)).into_response()
                }
                Err(original_err) => {
                    let error_dto: TransferProcessMessageWrapper<TransferErrorDto> = TransferProcessMessageWrapper {
                        context: ContextField::default(),
                        _type: TransferProcessMessageType::TransferError,
                        dto: TransferErrorDto {
                            consumer_pid: None,
                            provider_pid: None,
                            code: Some("5000".to_string()),
                            reason: Some(vec![original_err.to_string()]),
                        },
                    };
                    (StatusCode::BAD_REQUEST, Json(error_dto)).into_response()
                }
            },
        }
    }

    async fn handle_get_transfer_process(State(state): State<DspRouter>, Path(id): Path<String>) -> impl IntoResponse {
        Self::map_service_result(
            state.orchestrator.get_protocol_service().on_get_transfer_process(&id).await,
            StatusCode::OK,
        )
    }

    async fn handle_transfer_request(
        State(state): State<DspRouter>,
        input: Result<Json<TransferProcessMessageWrapper<TransferRequestMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::CREATED, |data| async move {
            state.orchestrator.get_protocol_service().on_transfer_request(&data).await
        })
        .await
    }

    async fn handle_transfer_request_rpc(
        State(state): State<DspRouter>,
        input: Result<Json<RpcTransferRequestMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::CREATED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_request(&data).await
        })
        .await
    }

    async fn handle_transfer_start(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferStartMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_protocol_service().on_transfer_start(&id, &data).await
        })
        .await
    }

    async fn handle_transfer_start_rpc(
        State(state): State<DspRouter>,
        input: Result<Json<RpcTransferStartMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_start(&data).await
        })
        .await
    }

    async fn handle_transfer_completion(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferCompletionMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_protocol_service().on_transfer_completion(&id, &data).await
        })
        .await
    }

    async fn handle_transfer_completion_rpc(
        State(state): State<DspRouter>,
        input: Result<Json<RpcTransferCompletionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_completion(&data).await
        })
        .await
    }

    async fn handle_transfer_termination(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferTerminationMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_protocol_service().on_transfer_termination(&id, &data).await
        })
        .await
    }

    async fn handle_transfer_termination_rpc(
        State(state): State<DspRouter>,
        input: Result<Json<RpcTransferTerminationMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_termination(&data).await
        })
        .await
    }

    async fn handle_transfer_suspension(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<TransferProcessMessageWrapper<TransferSuspensionMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::OK, |data| async move {
            state.orchestrator.get_protocol_service().on_transfer_suspension(&id, &data).await
        })
        .await
    }

    async fn handle_transfer_suspension_rpc(
        State(state): State<DspRouter>,
        input: Result<Json<RpcTransferSuspensionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_suspension(&data).await
        })
        .await
    }
}
