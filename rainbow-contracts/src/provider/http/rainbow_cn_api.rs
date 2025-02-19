/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::provider::core::rainbow_cn_api::*;
use crate::provider::core::rainbow_cn_errors::CnErrorProvider;
use crate::provider::core::rainbow_cn_types::*;
use axum::extract::rejection::JsonRejection;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::repo::NewParticipant;
use serde_json::Value;
use std::future::Future;
use tracing::info;

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
            put(handle_put_cn_process_by_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id",
            delete(handle_delete_cn_process_by_id),
        )
        // CNMessages
        .route(
            "/api/v1/contract-negotiation/messages",
            get(handle_get_cn_messages),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages",
            get(handle_get_cn_messages_by_cn_process),
        )
        .route(
            "/api/v1/contract-negotiation/processes/provider/:provider_id/messages",
            get(handle_get_cn_messages_by_provider),
        )
        .route(
            "/api/v1/contract-negotiation/processes/consumer/:consumer_id/messages",
            get(handle_get_cn_messages_by_consumer),
        )
        .route(
            "/api/v1/contract-negotiation/messages/:message_id",
            get(handle_get_cn_messages_by_cn_message_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages",
            post(handle_post_cn_message_by_cn_process),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id",
            put(handle_put_cn_message_by_cn_process),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id",
            delete(handle_delete_cn_message_by_cn_process),
        )
        // CNOffers
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/offers",
            get(handle_get_cn_offers_by_cn_process_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/offers/last",
            get(handle_get_last_cn_offers_by_cn_process_id),
        )
        .route(
            "/api/v1/contract-negotiation/messages/:message_id/offer",
            get(handle_get_cn_offers_by_cn_message_id),
        )
        .route(
            "/api/v1/contract-negotiation/offers/:offer_id",
            get(handle_get_cn_offer_by_offer_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers",
            post(handle_post_cn_offers_by_cn_process_id_and_message_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers/:offer_id",
            put(handle_put_cn_offers_by_cn_process_id_and_message_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers/:offer_id",
            delete(handle_delete_cn_offers_by_cn_process_id_and_message_id),
        )
        //
        // Agreements
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/agreement",
            get(handle_get_agreement_by_cn_process_id),
        )
        .route(
            "/api/v1/contract-negotiation/messages/:message_id/agreement",
            get(handle_get_agreement_by_cn_message_id),
        )
        .route(
            "/api/v1/contract-negotiation/agreements",
            get(handle_get_agreements),
        )
        .route(
            "/api/v1/contract-negotiation/agreements/:agreement_id",
            get(handle_get_agreement_by_agreement_id),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements",
            post(handle_post_agreement),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements/:agreement_id",
            put(handle_put_agreement),
        )
        .route(
            "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements/:agreement_id",
            delete(handle_delete_agreement),
        )
        //
        // Participants
        .route(
            "/api/v1/participants",
            get(handle_get_participants),
        )
        .route(
            "/api/v1/participants/:participant_id",
            get(handle_get_participant_by_id),
        )
        .route(
            "/api/v1/participants/:participant_id/agreements",
            get(handle_get_participant_agreements),
        )
        .route(
            "/api/v1/participants",
            post(handle_post_participant),
        )
        .route(
            "/api/v1/participants/:participant_id",
            put(handle_put_participant),
        )
        .route(
            "/api/v1/participants/:participant_id",
            delete(handle_delete_participant),
        )
}

///
/// CNProcess Rainbow API
///
async fn handle_get_cn_processes() -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/processes");
    match get_cn_processes().await {
        Ok(processes) => (StatusCode::OK, Json(processes)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        },
    }
}

async fn handle_get_cn_process_by_id(Path(process_id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/processes/{}", process_id);
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_process_by_id(process_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
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
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_process_by_provider(provider_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
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
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_process_by_consumer(consumer_id).await {
        Ok(process) => (StatusCode::OK, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
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
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match post_cn_process(input).await {
        Ok(process) => (StatusCode::CREATED, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_cn_process_by_id(
    Path(process_id): Path<String>,
    input: Result<Json<EditContractNegotiationRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!("PUT /api/v1/contract-negotiation/processes/{}", process_id);
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match put_cn_process(process_id, input).await {
        Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_cn_process_by_id(Path(process_id): Path<String>) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/contract-negotiation/processes/{}",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match delete_cn_process_by_id(process_id).await {
        Ok(process) => (StatusCode::NO_CONTENT, Json(process)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

///
/// CNMessages Rainbow API
///
async fn handle_get_cn_messages() -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/messages");
    match get_cn_messages().await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_messages_by_cn_process(Path(process_id): Path<String>) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/{}/messages",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_messages_by_cn_process(process_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_messages_by_cn_message_id(
    Path(message_id): Path<String>,
) -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/messages/{}", message_id);
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_messages_by_cn_message_id(message_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_messages_by_provider(Path(provider_id): Path<String>) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/provider/{}/messages",
        provider_id
    );
    let provider_id = match get_urn_from_string(&provider_id) {
        Ok(provider_id) => provider_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_messages_by_cn_provider_id(provider_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_messages_by_consumer(Path(consumer_id): Path<String>) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/consumer/{}/messages",
        consumer_id
    );
    let consumer_id = match get_urn_from_string(&consumer_id) {
        Ok(consumer_id) => consumer_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_cn_messages_by_cn_consumer_id(consumer_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_cn_message_by_cn_process(
    Path(process_id): Path<String>,
    input: Result<Json<NewContractNegotiationMessageRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /api/v1/contract-negotiation/processes/{}/messages",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match post_cn_message_by_cn_process(process_id, input).await {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_cn_message_by_cn_process(
    Path((process_id, message_id)): Path<(String, String)>,
    input: Result<Json<EditContractNegotiationMessageRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "PUT /api/v1/contract-negotiation/processes/{}/messages/{}",
        process_id, message_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match put_cn_message_by_cn_process(process_id, message_id, input).await {
        Ok(message) => (StatusCode::ACCEPTED, Json(message)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_cn_message_by_cn_process(
    Path((process_id, message_id)): Path<(String, String)>,
) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/contract-negotiation/processes/{}/messages/{}",
        process_id, message_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match delete_cn_message_by_cn_process(process_id, message_id).await {
        Ok(message) => (StatusCode::NO_CONTENT, Json(message)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

///
/// CNOffers Rainbow API
///
async fn handle_get_cn_offers_by_cn_process_id(
    Path(process_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/{}/offers",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_offers_by_cn_process_id(process_id).await {
        Ok(offers) => (StatusCode::OK, Json(offers)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_last_cn_offers_by_cn_process_id(
    Path(process_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/{}/offers/last",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_last_cn_offers_by_cn_process_id(process_id).await {
        Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_offers_by_cn_message_id(
    Path(message_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/messages/{}/offer",
        message_id
    );
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_offer_by_cn_message_id(message_id).await {
        Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_cn_offer_by_offer_id(Path(offer_id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/offers/{}", offer_id);
    let offer_id = match get_urn_from_string(&offer_id) {
        Ok(offer_id) => offer_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_cn_offer_by_offer_id(offer_id).await {
        Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_cn_offers_by_cn_process_id_and_message_id(
    Path((process_id, message_id)): Path<(String, String)>,
    input: Result<Json<NewContractNegotiationOfferRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /api/v1/contract-negotiation/processes/{}/messages/{}/offers",
        process_id, message_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match post_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, input).await {
        Ok(offer) => (StatusCode::CREATED, Json(offer)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_cn_offers_by_cn_process_id_and_message_id(
    Path((process_id, message_id, offer_id)): Path<(String, String, String)>,
    input: Result<Json<EditContractNegotiationOfferRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "PUT /api/v1/contract-negotiation/processes/{}/messages/{}/offers/{}",
        process_id, message_id, offer_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let offer_id = match get_urn_from_string(&offer_id) {
        Ok(offer_id) => offer_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match put_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, offer_id, input)
        .await
    {
        Ok(offer) => (StatusCode::ACCEPTED, Json(offer)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_cn_offers_by_cn_process_id_and_message_id(
    Path((process_id, message_id, offer_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/contract-negotiation/processes/{}/messages/{}/offers/{}",
        process_id, message_id, offer_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let offer_id = match get_urn_from_string(&offer_id) {
        Ok(offer_id) => offer_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match delete_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, offer_id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

///
/// Agreements
///
async fn handle_get_agreement_by_cn_process_id(
    Path(process_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/processes/{}/agreement",
        process_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_agreement_by_cn_process_id(process_id).await {
        Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_agreement_by_cn_message_id(
    Path(message_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/messages/{}/agreement",
        message_id
    );
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_agreement_by_cn_message_id(message_id).await {
        Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_agreements() -> impl IntoResponse {
    info!("GET /api/v1/contract-negotiation/agreements");
    match get_agreements().await {
        Ok(agreements) => (StatusCode::OK, Json(agreements)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_agreement_by_agreement_id(
    Path(agreement_id): Path<String>,
) -> impl IntoResponse {
    info!(
        "GET /api/v1/contract-negotiation/agreements/{}",
        agreement_id
    );
    let agreement_id = match get_urn_from_string(&agreement_id) {
        Ok(agreement_id) => agreement_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match get_agreement_by_agreement_id(agreement_id).await {
        Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_agreement(
    Path((process_id, message_id)): Path<(String, String)>,
    input: Result<Json<NewAgreementRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /api/v1/contract-negotiation/processes/{}/messages/{}/agreements",
        process_id, message_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match post_agreement(process_id, message_id, input).await {
        Ok(agreement) => (StatusCode::CREATED, Json(agreement)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_agreement(
    Path((process_id, message_id, agreement_id)): Path<(String, String, String)>,
    input: Result<Json<EditAgreementRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "PUT /api/v1/contract-negotiation/processes/{}/messages/{}/agreements/{}",
        process_id, message_id, agreement_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let agreement_id = match get_urn_from_string(&agreement_id) {
        Ok(agreement_id) => agreement_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };
    match put_agreement(process_id, message_id, agreement_id, input).await {
        Ok(agreement) => (StatusCode::ACCEPTED, Json(agreement)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_agreement(
    Path((process_id, message_id, agreement_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/contract-negotiation/processes/{}/messages/{}/agreements/{}",
        process_id, message_id, agreement_id
    );
    let process_id = match get_urn_from_string(&process_id) {
        Ok(process_id) => process_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let message_id = match get_urn_from_string(&message_id) {
        Ok(message_id) => message_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let agreement_id = match get_urn_from_string(&agreement_id) {
        Ok(agreement_id) => agreement_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    match delete_agreement(process_id, message_id, agreement_id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

///
/// Participants
///

async fn handle_get_participants() -> impl IntoResponse {
    info!("GET /api/v1/participants");

    match get_participants().await {
        Ok(participants) => (StatusCode::OK, Json(participants)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_participant_by_id(Path(participant_id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/participants/{}", participant_id);
    let participant_id = match get_urn_from_string(&participant_id) {
        Ok(participant_id) => participant_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_participant_by_id(participant_id).await {
        Ok(participant) => (StatusCode::OK, Json(participant)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_participant_agreements(
    Path(participant_id): Path<String>,
) -> impl IntoResponse {
    info!("GET /api/v1/participants/{}/agreements", participant_id);
    let participant_id = match get_urn_from_string(&participant_id) {
        Ok(participant_id) => participant_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match get_participant_agreements(participant_id).await {
        Ok(agreements) => (StatusCode::OK, Json(agreements)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_participant(
    input: Result<Json<NewParticipantRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /api/v1/participants");
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };

    match post_participant(input).await {
        Ok(participant) => (StatusCode::CREATED, Json(participant)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_participant(
    Path(participant_id): Path<String>,
    input: Result<Json<EditParticipantRequest>, JsonRejection>,
) -> impl IntoResponse {
    info!("PUT /api/v1/participants/{}", participant_id);
    let participant_id = match get_urn_from_string(&participant_id) {
        Ok(participant_id) => participant_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
    };

    match put_participant(participant_id, input).await {
        Ok(participant) => (StatusCode::ACCEPTED, Json(participant)).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_participant(Path(participant_id): Path<String>) -> impl IntoResponse {
    info!("DELETE /api/v1/participants/{}", participant_id);
    let participant_id = match get_urn_from_string(&participant_id) {
        Ok(participant_id) => participant_id,
        Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
    };

    match delete_participant(participant_id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => match err.downcast::<CnErrorProvider>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}
