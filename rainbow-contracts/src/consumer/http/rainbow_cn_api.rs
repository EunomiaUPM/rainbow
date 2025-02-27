use crate::consumer::core::rainbow_cn_api::{
    delete_cn_process, get_cn_process_by_consumer, get_cn_process_by_id,
    get_cn_process_by_provider, get_cn_processes, post_cn_process, put_cn_process,
};
use crate::consumer::core::rainbow_cn_errors::CnErrorConsumer;
use crate::consumer::core::rainbow_cn_types::{
    EditContractNegotiationRequest, NewContractNegotiationRequest,
};
use axum::extract::rejection::JsonRejection;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use log::info;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_consumer::repo::{
    EditContractNegotiationProcess, NewContractNegotiationProcess,
};

pub fn router() -> Router {
    Router::new()
        // CNProcess
        .route(
            "/api/v1/contract-negotiation/processes",
            get(handle_get_cn_processes),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id",
            get(handle_get_cn_process_by_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/provider/:provider_id",
            get(handle_get_cn_process_by_provider),
        )
        .route(
            "/api/v1/contract-negotiation/processes/consumer/:consumer_id",
            get(handle_get_cn_process_by_consumer),
        )
        .route(
            "/api/v1/contract-negotiation/processes",
            post(handle_post_cn_process),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id",
            put(handle_put_cn_process),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id",
            delete(handle_delete_cn_process),
        )
}

async fn handle_get_cn_processes() -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/processes");

    match get_cn_processes().await {
        Ok(processes) => (StatusCode::OK, Json(processes)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_process_by_id(Path(process_id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/processes/{}", process_id);
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorConsumer::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_process_by_id(process_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_process_by_provider(Path(provider_id): Path<String>) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/provider/{}",
        provider_id
    );
    let provider_id = match get_urn_from_string(&provider_id) {
        Ok(provider_id) => provider_id,
        Err(err) => return CnErrorConsumer::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_process_by_provider(provider_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_process_by_consumer(Path(consumer_id): Path<String>) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/consumer/{}",
        consumer_id
    );
    let consumer_id = match get_urn_from_string(&consumer_id) {
        Ok(consumer_id) => consumer_id,
        Err(err) => return CnErrorConsumer::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_process_by_consumer(consumer_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_cn_process(
    input: Result<Json<NewContractNegotiationRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /api/v1/contract-negotiation/processes");
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
    };

    match post_cn_process(input).await {
        Ok(process) => (StatusCode::CREATED, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_cn_process(
    Path(process_id): Path<String>,
    input: Result<Json<EditContractNegotiationRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!("PUT /api/v1/contract-negotiation/processes/{}", process_id);
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorConsumer::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
    };

    match put_cn_process(process_id, input).await {
        Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_cn_process(Path(process_id): Path<String>) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/contract-negotiation/processes/{}",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorConsumer::UrnUuidSchema(err.to_string()).into_response(),
    };

    match delete_cn_process(process_id).await {
        Ok(process) => (StatusCode::NO_CONTENT, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorConsumer>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}
