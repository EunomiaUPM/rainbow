use crate::common::err::TransferErrorType;
use crate::common::err::TransferErrorType::{ConsumerNotReachableError, NotCheckedError, ProtocolBodyError, TransferProcessNotFound};
use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::common::http::middleware::{pids_as_urn_validation_middleware, protocol_rules_middleware, schema_validation_middleware};
use crate::protocol::messages::{
    TransferCompletionMessage, TransferMessageTypes, TransferMessageTypesForDb,
    TransferProcessMessage, TransferRequestMessage, TransferRoles, TransferStartMessage,
    TransferState, TransferStateForDb, TransferSuspensionMessage, TransferTerminationMessage,
    TRANSFER_CONTEXT,
};
use crate::provider::data::entities::{transfer_message, transfer_process};
use crate::provider::lib::control_plane::{
    get_transfer_requests_by_provider, transfer_completion, transfer_request, transfer_start,
    transfer_suspension, transfer_termination,
};
use anyhow::bail;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use clap::builder::TypedValueParser;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::utils::{convert_uri_to_uuid, convert_uuid_to_uri};
use reqwest::Error;
use sea_orm::{ActiveValue, EntityTrait};
use tracing::{debug, error, info};
use uuid::Uuid;

pub fn router() -> Router {
    // Based on a group of middlewares
    let router_group_a = Router::new().route(
        "/transfers/:provider_pid",
        get(handle_get_transfer_by_provider),
    );

    // Based on a group of middlewares
    let router_group_b = Router::new()
        .route("/transfers/request", post(handle_transfer_request))
        .route("/transfers/start", post(handle_transfer_start))
        .route("/transfers/suspension", post(handle_transfer_suspension))
        .route("/transfers/completion", post(handle_transfer_completion))
        .route("/transfers/termination", post(handle_transfer_termination))
        .route_layer(middleware::from_fn(pids_as_urn_validation_middleware))
        .route_layer(middleware::from_fn(protocol_rules_middleware))
        .route_layer(middleware::from_fn(schema_validation_middleware));

    Router::new().merge(router_group_a).merge(router_group_b)
}

async fn handle_get_transfer_by_provider(Path(provider_pid): Path<Uuid>) -> impl IntoResponse {
    info!("GET /transfers/{}", provider_pid.to_string());

    match get_transfer_requests_by_provider(provider_pid).await.unwrap() {
        Some(transfer_process) => (
            StatusCode::OK,
            Json(TransferProcessMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferProcessMessage.to_string(),
                provider_pid: transfer_process.provider_pid.to_string(),
                consumer_pid: transfer_process.consumer_pid.to_string(),
                state: TransferState::try_from(transfer_process.state).unwrap(),
            }),
        )
            .into_response(),
        None => TransferProcessNotFound.into_response(),
    }
}

async fn handle_transfer_request(
    result: Result<Json<TransferRequestMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/request");

    match result {
        Ok(Json(input)) => match transfer_request(input, send_transfer_start).await {
            Ok(tp) => (StatusCode::CREATED, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => {
                    println!("{:#?}", transfer_error);
                    transfer_error.into_response()
                }
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn send_transfer_start(
    Json(input): Json<TransferRequestMessage>,
    provider_pid: Uuid,
    data_plane_id: Uuid,
) -> anyhow::Result<()> {
    // TODO REFACTOR IN CONTROL PLANE
    let transfer_start_message = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&provider_pid)?,
        consumer_pid: input.consumer_pid.to_string(),
        data_address: input.data_address,
    };

    let response = DATA_PLANE_HTTP_CLIENT
        .clone()
        .post(format!("{}/start", input.callback_address))
        .header("content-type", "application/json")
        .json(&transfer_start_message)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                let created_at = chrono::Utc::now().naive_utc();
                let message_id = Uuid::new_v4();

                let db_connection = get_db_connection().await;
                // persist information
                let old_process =
                    transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
                if old_process.is_none() {
                    bail!(TransferProcessNotFound)
                }
                let old_process = old_process.unwrap();
                let transfer_process_db =
                    transfer_process::Entity::update(transfer_process::ActiveModel {
                        provider_pid: ActiveValue::Set(old_process.provider_pid),
                        consumer_pid: ActiveValue::Set(old_process.consumer_pid),
                        agreement_id: ActiveValue::Set(old_process.agreement_id),
                        data_plane_id: ActiveValue::Set(Some(data_plane_id)),
                        subscription_id: ActiveValue::Set(None),
                        state: ActiveValue::Set(TransferStateForDb::STARTED),
                        created_at: ActiveValue::Set(old_process.created_at),
                        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
                    })
                        .exec(db_connection)
                        .await?;

                // // persist
                // let transfer_process = TRANSFER_PROVIDER_REPO
                //     .update_transfer_process_by_provider_pid(
                //         &provider_pid,
                //         TransferState::STARTED,
                //         Some(data_plane_id),
                //     )?
                //     .unwrap();

                let transfer_message_db =
                    transfer_message::Entity::insert(transfer_message::ActiveModel {
                        id: ActiveValue::Set(Uuid::new_v4()),
                        transfer_process_id: ActiveValue::Set(provider_pid),
                        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                        updated_at: ActiveValue::Set(None),
                        message_type: ActiveValue::Set(
                            TransferMessageTypesForDb::TransferStartMessage,
                        ),
                        from: ActiveValue::Set(TransferRoles::Provider),
                        to: ActiveValue::Set(TransferRoles::Consumer),
                        content: ActiveValue::Set(serde_json::to_value(&transfer_start_message)?),
                    })
                        .exec_with_returning(db_connection)
                        .await?;

                // TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
                //     id: message_id,
                //     transfer_process_id: transfer_process.provider_pid,
                //     created_at,
                //     message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                //     from: "provider".to_string(),
                //     to: "consumer".to_string(),
                //     content: serde_json::to_value(&transfer_start_message)?,
                // })?;

                Ok(())
            } else {
                println!("not started...."); // TODO Error
                Ok(())
            }
        }
        Err(_) => bail!(ConsumerNotReachableError),
    }
}

async fn handle_transfer_start(
    result: Result<Json<TransferStartMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/start");

    match result {
        Ok(Json(input)) => match transfer_start(&input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_suspension(
    result: Result<Json<TransferSuspensionMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/suspension");

    match result {
        Ok(Json(input)) => match transfer_suspension(&input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_completion(
    result: Result<Json<TransferCompletionMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/completion");

    match result {
        Ok(Json(input)) => match transfer_completion(&input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_termination(
    result: Result<Json<TransferTerminationMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/termination");

    match result {
        Ok(Json(input)) => match transfer_termination(&input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}
