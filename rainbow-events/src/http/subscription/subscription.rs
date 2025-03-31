use crate::core::subscription::subscription_err::SubscriptionErrors;
use crate::core::subscription::subscription_types::{RainbowEventsSubscriptionCreationRequest, SubscriptionEntities};
use crate::core::subscription::RainbowEventsSubscriptionTrait;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use log::info;
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct RainbowEventsSubscriptionRouter<T>
where
    T: RainbowEventsSubscriptionTrait + Send + Sync,
{
    service: Arc<T>,
    entity_type: Option<SubscriptionEntities>,
}
impl<T> RainbowEventsSubscriptionRouter<T>
where
    T: RainbowEventsSubscriptionTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>, entity_type: Option<SubscriptionEntities>) -> Self {
        Self { service, entity_type }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/subscriptions", get(Self::handle_get_all_subscriptions))
            .route(
                "/subscriptions/:id",
                get(Self::handle_get_subscription_by_id),
            )
            .route(
                "/subscriptions/:id",
                put(Self::handle_put_subscription_by_id),
            )
            .route("/subscriptions", post(Self::handle_post_subscription_by_id))
            .route(
                "/subscriptions/:id",
                delete(Self::handle_delete_subscription_by_id),
            )
            .with_state((self.service, self.entity_type))
    }
    fn serialize_entity_type(entity: &Option<SubscriptionEntities>) -> String {
        match entity {
            None => "".to_string(),
            Some(entity) => match entity {
                SubscriptionEntities::TransferProcess => "/api/v1/transfers".to_string(),
                SubscriptionEntities::Catalog => "/api/v1/catalog".to_string(),
                SubscriptionEntities::ContractNegotiationProcess => "/api/v1/contract-negotiations".to_string(),
                SubscriptionEntities::DataPlaneProcess => "/api/v1/data-plane".to_string(),
            },
        }
    }
    async fn handle_get_all_subscriptions(
        State((service, entity)): State<(Arc<T>, Option<SubscriptionEntities>)>,
    ) -> impl IntoResponse {
        info!("GET {}/subscriptions", Self::serialize_entity_type(&entity));
        match service.get_all_subscriptions().await {
            Ok(subscriptions) => (StatusCode::OK, Json(subscriptions)).into_response(),
            Err(e) => match e.downcast::<SubscriptionErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_get_subscription_by_id(
        State((service, entity)): State<(Arc<T>, Option<SubscriptionEntities>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET {}/subscriptions/{}",
            Self::serialize_entity_type(&entity),
            id
        );
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return SubscriptionErrors::UrnUuidSchema(id.to_string()).into_response(),
        };
        match service.get_subscription_by_id(id).await {
            Ok(subscriptions) => (StatusCode::OK, Json(subscriptions)).into_response(),
            Err(e) => match e.downcast::<SubscriptionErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_put_subscription_by_id(
        State((service, entity)): State<(Arc<T>, Option<SubscriptionEntities>)>,
        Path(id): Path<String>,
        input: Result<Json<RainbowEventsSubscriptionCreationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "PUT {}/subscriptions/{}",
            Self::serialize_entity_type(&entity),
            id
        );
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return SubscriptionErrors::UrnUuidSchema(id.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => return SubscriptionErrors::JsonRejection(err).into_response(),
        };
        match service.put_subscription_by_id(id, input).await {
            Ok(subscriptions) => (StatusCode::OK, Json(subscriptions)).into_response(),
            Err(e) => match e.downcast::<SubscriptionErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_post_subscription_by_id(
        State((service, entity)): State<(Arc<T>, Option<SubscriptionEntities>)>,
        input: Result<Json<RainbowEventsSubscriptionCreationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST {}/subscriptions",
            Self::serialize_entity_type(&entity)
        );
        let input = match input {
            Ok(input) => input.0,
            Err(err) => return SubscriptionErrors::JsonRejection(err).into_response(),
        };
        match service.create_subscription(input, entity.unwrap()).await {
            Ok(subscriptions) => (StatusCode::OK, Json(subscriptions)).into_response(),
            Err(e) => match e.downcast::<SubscriptionErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
    async fn handle_delete_subscription_by_id(
        State((service, entity)): State<(Arc<T>, Option<SubscriptionEntities>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "DELETE {}/subscriptions/{}",
            Self::serialize_entity_type(&entity),
            id
        );
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return SubscriptionErrors::UrnUuidSchema(id.to_string()).into_response(),
        };
        match service.delete_subscription_by_id(id).await {
            Ok(subscriptions) => (StatusCode::OK, Json(subscriptions)).into_response(),
            Err(e) => match e.downcast::<SubscriptionErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
}
