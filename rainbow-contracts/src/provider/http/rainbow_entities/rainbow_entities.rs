/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::provider::core::rainbow_entities::rainbow_entities_errors::CnErrorProvider;
use crate::provider::core::rainbow_entities::rainbow_entities_types::{EditAgreementRequest, EditContractNegotiationMessageRequest, EditContractNegotiationOfferRequest, EditContractNegotiationRequest, NewAgreementRequest, NewContractNegotiationMessageRequest, NewContractNegotiationOfferRequest, NewContractNegotiationRequest, ProcessesQuery};
use crate::provider::core::rainbow_entities::RainbowEntitiesContractNegotiationProviderTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;


pub struct RainbowEntitesContractNegotiationProviderRouter<T>
where
    T: RainbowEntitiesContractNegotiationProviderTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> RainbowEntitesContractNegotiationProviderRouter<T>
where
    T: RainbowEntitiesContractNegotiationProviderTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            // CNProcess
            .route(
                "/api/v1/contract-negotiation/processes",
                get(Self::handle_get_cn_processes),
            )
            .route(
                "/api/v1/contract-negotiation/processes/:process_id",
                get(Self::handle_get_cn_process_by_id),
            )
            .route(
                "/api/v1/contract-negotiation/processes/provider/:provider_id",
                get(Self::handle_get_cn_process_by_provider),
            )
            .route(
                "/api/v1/contract-negotiation/processes/consumer/:consumer_id",
                get(Self::handle_get_cn_process_by_consumer),
            )
            .route(
                "/api/v1/contract-negotiation/processes/participant/:participant_id",
                get(Self::handle_get_cn_processes_by_participant),
            )
            .route(
                "/api/v1/contract-negotiation/processes",
                post(Self::handle_post_cn_process),
            )
            .route(
                "/api/v1/contract-negotiation/processes/:process_id",
                put(Self::handle_put_cn_process_by_id),
            )
            .route(
                "/api/v1/contract-negotiation/processes/:process_id",
                delete(Self::handle_delete_cn_process_by_id),
            )
            // CNMessages
            .route(
                "/api/v1/contract-negotiation/messages",
                get(Self::handle_get_cn_messages),
            )
            .route(
                "/api/v1/contract-negotiation/processes/:process_id/messages",
                get(Self::handle_get_cn_messages_by_cn_process),
            )
            .route(
                "/api/v1/contract-negotiation/processes/provider/:provider_id/messages",
                get(Self::handle_get_cn_messages_by_provider),
            )
            .route(
                "/api/v1/contract-negotiation/processes/consumer/:consumer_id/messages",
                get(Self::handle_get_cn_messages_by_consumer),
            )
            .route(
                "/api/v1/contract-negotiation/messages/:message_id",
                get(Self::handle_get_cn_messages_by_cn_message_id),
            )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages",
            //     post(Self::handle_post_cn_message_by_cn_process),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id",
            //     put(Self::handle_put_cn_message_by_cn_process),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id",
            //     delete(Self::handle_delete_cn_message_by_cn_process),
            // )
            // CNOffers
            .route(
                "/api/v1/contract-negotiation/processes/:process_id/offers",
                get(Self::handle_get_cn_offers_by_cn_process_id),
            )
            .route(
                "/api/v1/contract-negotiation/processes/:process_id/offers/last",
                get(Self::handle_get_last_cn_offers_by_cn_process_id),
            )
            .route(
                "/api/v1/contract-negotiation/messages/:message_id/offer",
                get(Self::handle_get_cn_offers_by_cn_message_id),
            )
            .route(
                "/api/v1/contract-negotiation/offers/:offer_id",
                get(Self::handle_get_cn_offer_by_offer_id),
            )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers",
            //     post(Self::handle_post_cn_offers_by_cn_process_id_and_message_id),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers/:offer_id",
            //     put(Self::handle_put_cn_offers_by_cn_process_id_and_message_id),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/offers/:offer_id",
            //     delete(Self::handle_delete_cn_offers_by_cn_process_id_and_message_id),
            // )
            //
            // Agreements
            .route(
                "/api/v1/contract-negotiation/processes/:process_id/agreement",
                get(Self::handle_get_agreement_by_cn_process_id),
            )
            .route(
                "/api/v1/contract-negotiation/messages/:message_id/agreement",
                get(Self::handle_get_agreement_by_cn_message_id),
            )
            .route(
                "/api/v1/contract-negotiation/agreements",
                get(Self::handle_get_agreements),
            )
            .route(
                "/api/v1/contract-negotiation/agreements/:agreement_id",
                get(Self::handle_get_agreement_by_agreement_id),
            )
            .route(
                "/api/v1/contract-negotiation/agreements/participant/:participant_id",
                get(Self::handle_get_agreements_by_participant_id),
            )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements",
            //     post(Self::handle_post_agreement),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements/:agreement_id",
            //     put(Self::handle_put_agreement),
            // )
            // .route(
            //     "/api/v1/contract-negotiation/processes/:process_id/messages/:message_id/agreements/:agreement_id",
            //     delete(Self::handle_delete_agreement),
            // )
            .with_state(self.service)
    }

    ///
    /// CNProcess Rainbow API
    ///
    async fn handle_get_cn_processes(
        State(service): State<Arc<T>>,
        query: Query<ProcessesQuery>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/processes");
        let client_type = query.0.client_type;
        match service.get_cn_processes(client_type).await {
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

    async fn handle_get_cn_process_by_id(
        State(service): State<Arc<T>>,
        Path(process_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/processes/{}", process_id);
        let process_id = match get_urn_from_string(&process_id) {
            Ok(process_id) => process_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_process_by_id(process_id).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_process_by_provider(
        State(service): State<Arc<T>>,
        Path(provider_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/provider/{}",
            provider_id
        );
        let provider_id = match get_urn_from_string(&provider_id) {
            Ok(process_id) => process_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_process_by_provider(provider_id).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_process_by_consumer(
        State(service): State<Arc<T>>,
        Path(consumer_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/consumer/{}",
            consumer_id
        );
        let consumer_id = match get_urn_from_string(&consumer_id) {
            Ok(consumer_id) => consumer_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_process_by_consumer(consumer_id).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_processes_by_participant(
        State(service): State<Arc<T>>,
        query: Query<ProcessesQuery>,
        Path(participant_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/participant/{}",
            participant_id
        );
        let client_type = query.0.client_type;
        match service.get_cn_processes_by_participant(participant_id, client_type).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_cn_process(
        State(service): State<Arc<T>>,
        input: Result<Json<NewContractNegotiationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/contract-negotiation/processes");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
        };
        match service.post_cn_process(input).await {
            Ok(process) => (StatusCode::CREATED, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_cn_process_by_id(
        State(service): State<Arc<T>>,
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
        match service.put_cn_process(process_id, input).await {
            Ok(process) => (StatusCode::ACCEPTED, Json(process)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_cn_process_by_id(
        State(service): State<Arc<T>>,
        Path(process_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/contract-negotiation/processes/{}",
            process_id
        );
        let process_id = match get_urn_from_string(&process_id) {
            Ok(process_id) => process_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.delete_cn_process_by_id(process_id).await {
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
    async fn handle_get_cn_messages(State(service): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/messages");
        match service.get_cn_messages().await {
            Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_messages_by_cn_process(
        State(service): State<Arc<T>>,
        Path(process_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/{}/messages",
            process_id
        );
        let process_id = match get_urn_from_string(&process_id) {
            Ok(process_id) => process_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_messages_by_cn_process(process_id).await {
            Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_messages_by_cn_message_id(
        State(service): State<Arc<T>>,
        Path(message_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/messages/{}", message_id);
        let message_id = match get_urn_from_string(&message_id) {
            Ok(message_id) => message_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_messages_by_cn_message_id(message_id).await {
            Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_messages_by_provider(
        State(service): State<Arc<T>>,
        Path(provider_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/provider/{}/messages",
            provider_id
        );
        let provider_id = match get_urn_from_string(&provider_id) {
            Ok(provider_id) => provider_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_messages_by_cn_provider_id(provider_id).await {
            Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_messages_by_consumer(
        State(service): State<Arc<T>>,
        Path(consumer_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/processes/consumer/{}/messages",
            consumer_id
        );
        let consumer_id = match get_urn_from_string(&consumer_id) {
            Ok(consumer_id) => consumer_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_cn_messages_by_cn_consumer_id(consumer_id).await {
            Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_cn_message_by_cn_process(
        State(service): State<Arc<T>>,
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
        match service.post_cn_message_by_cn_process(process_id, input).await {
            Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_cn_message_by_cn_process(
        State(service): State<Arc<T>>,
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
        match service.put_cn_message_by_cn_process(process_id, message_id, input).await {
            Ok(message) => (StatusCode::ACCEPTED, Json(message)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_cn_message_by_cn_process(
        State(service): State<Arc<T>>,
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
        match service.delete_cn_message_by_cn_process(process_id, message_id).await {
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
        State(service): State<Arc<T>>,
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

        match service.get_cn_offers_by_cn_process_id(process_id).await {
            Ok(offers) => (StatusCode::OK, Json(offers)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_last_cn_offers_by_cn_process_id(
        State(service): State<Arc<T>>,
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

        match service.get_last_cn_offers_by_cn_process_id(process_id).await {
            Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_offers_by_cn_message_id(
        State(service): State<Arc<T>>,
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

        match service.get_cn_offer_by_cn_message_id(message_id).await {
            Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_cn_offer_by_offer_id(
        State(service): State<Arc<T>>,
        Path(offer_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/offers/{}", offer_id);
        let offer_id = match get_urn_from_string(&offer_id) {
            Ok(offer_id) => offer_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };

        match service.get_cn_offer_by_offer_id(offer_id).await {
            Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_cn_offers_by_cn_process_id_and_message_id(
        State(service): State<Arc<T>>,
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
        match service.post_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, input).await {
            Ok(offer) => (StatusCode::CREATED, Json(offer)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_cn_offers_by_cn_process_id_and_message_id(
        State(service): State<Arc<T>>,
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
        match service.put_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, offer_id, input).await {
            Ok(offer) => (StatusCode::ACCEPTED, Json(offer)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_cn_offers_by_cn_process_id_and_message_id(
        State(service): State<Arc<T>>,
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

        match service.delete_cn_offer_by_cn_process_id_and_message_id(process_id, message_id, offer_id).await {
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
        State(service): State<Arc<T>>,
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
        match service.get_agreement_by_cn_process_id(process_id).await {
            Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_agreement_by_cn_message_id(
        State(service): State<Arc<T>>,
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
        match service.get_agreement_by_cn_message_id(message_id).await {
            Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_agreements(State(service): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /api/v1/contract-negotiation/agreements");
        match service.get_agreements().await {
            Ok(agreements) => (StatusCode::OK, Json(agreements)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_agreement_by_agreement_id(
        State(service): State<Arc<T>>,
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
        match service.get_agreement_by_agreement_id(agreement_id).await {
            Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_agreements_by_participant_id(
        State(service): State<Arc<T>>,
        Path(participant_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/contract-negotiation/agreements/participant/{}",
            participant_id
        );
        let participant_id = match get_urn_from_string(&participant_id) {
            Ok(participant_id) => participant_id,
            Err(err) => return CnErrorProvider::UrnUuidSchema(err.to_string()).into_response(),
        };
        match service.get_agreements_by_participant_id(participant_id).await {
            Ok(agreements) => (StatusCode::OK, Json(agreements)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_agreement(
        State(service): State<Arc<T>>,
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
        match service.post_agreement(process_id, message_id, input).await {
            Ok(agreement) => (StatusCode::CREATED, Json(agreement)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_agreement(
        State(service): State<Arc<T>>,
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
        match service.put_agreement(process_id, message_id, agreement_id, input).await {
            Ok(agreement) => (StatusCode::ACCEPTED, Json(agreement)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_agreement(
        State(service): State<Arc<T>>,
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
        match service.delete_agreement(process_id, message_id, agreement_id).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
