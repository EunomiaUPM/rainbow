use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAcceptanceRequest, SetupRequestRequest, SetupTerminationRequest, SetupVerificationRequest,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCContractNegotiationConsumerTrait;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use std::sync::Arc;
use tracing::info;

pub struct DSRPCContractNegotiationConsumerRouter<T>
where
    T: DSRPCContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> DSRPCContractNegotiationConsumerRouter<T>
where
    T: DSRPCContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/negotiations/rpc/setup-request",
                post(Self::handle_setup_request),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-acceptance",
                post(Self::handle_setup_acceptance),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-verification",
                post(Self::handle_setup_verification),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-termination",
                post(Self::handle_setup_termination),
            )
            .with_state(self.service)
    }
    async fn handle_setup_request(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupRequestRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_request(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_setup_acceptance(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupAcceptanceRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-acceptance");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_acceptance(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_setup_verification(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupVerificationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-verification");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_verification(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_setup_termination(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-termination");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_termination(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
}
