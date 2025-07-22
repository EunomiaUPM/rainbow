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

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::delete;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::policy_templates::CreatePolicyTemplateRequest;
use rainbow_db::datahub::repo::{NewPolicyTemplateModel, PolicyTemplatesRepo};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, to_value};
use std::sync::Arc;
use tracing::error;
use tracing::info;


pub struct PolicyTemplatesRouter<T, U>
where
    T: PolicyTemplatesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync + 'static,
{
    policy_templates_service: Arc<T>,
    notification_service: Arc<U>,
}

#[derive(Debug, Deserialize)]
pub struct GetPolicyTemplatesParams {
    #[serde(default = "default_limit")]
    pub limit: Option<u64>,
    #[serde(default = "default_page")]
    pub page: Option<u64>,
}

fn default_limit() -> Option<u64> {
    Some(10)
}

fn default_page() -> Option<u64> {
    Some(1)
}

impl<T, U> PolicyTemplatesRouter<T, U>
where
    T: PolicyTemplatesRepo + Send + Sync,
    U: RainbowEventsNotificationTrait + Send + Sync + 'static,
{
    pub fn new(policy_templates_service: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { policy_templates_service, notification_service }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/datahub/policy-templates",
                post(Self::create_policy_template),
            )
            .route(
                "/api/v1/datahub/policy-templates/:id",
                delete(Self::delete_policy_template_by_id),
            )
            .route(
                "/api/v1/datahub/policy-templates",
                get(Self::get_all_policy_templates),
            )
            .route(
                "/api/v1/datahub/policy-templates/:id",
                get(Self::get_policy_template_by_id),
            )
            .with_state((self.policy_templates_service, self.notification_service))
    }

    async fn create_policy_template(
        State((policy_templates_service, notification_service)): State<(Arc<T>, Arc<U>)>,
        Json(payload): Json<CreatePolicyTemplateRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/datahub/policy-templates");

        let new_template: NewPolicyTemplateModel = payload.into();
        match policy_templates_service.create_policy_template(new_template).await {
            Ok(template) => {
                notification_service
                    .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                        category: RainbowEventsNotificationMessageCategory::Catalog,
                        subcategory: "PolicyTemplate".to_string(),
                        message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                        message_content: to_value(&template).unwrap(),
                        message_operation: RainbowEventsNotificationMessageOperation::Creation,
                    })
                    .await.unwrap();
                (
                    StatusCode::CREATED,
                    Json(serde_json::to_value(template).unwrap()),
                )
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    async fn delete_policy_template_by_id(
        State((policy_templates_service, notification_service)): State<(Arc<T>, Arc<U>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/datahub/policy-templates/{}", id);

        match policy_templates_service.delete_policy_template_by_id(id.clone()).await {
            Ok(_) => {
                info!("Policy template eliminada exitosamente");
                notification_service
                    .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                        category: RainbowEventsNotificationMessageCategory::Catalog,
                        subcategory: "PolicyTemplate".to_string(),
                        message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                        message_content: json!({
                            "@type": "Catalog",
                            "@id": id.clone().to_string()
                        }),
                        message_operation: RainbowEventsNotificationMessageOperation::Deletion,
                    })
                    .await.unwrap();
                (
                    StatusCode::NO_CONTENT,
                    Json(json!({ "message": "Policy template deleted successfully" })),
                )
            }
            Err(e) => {
                error!("Error al eliminar policy template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
            }
        }
    }

    async fn get_all_policy_templates(
        State((policy_templates_service, _notification_service)): State<(Arc<T>, Arc<U>)>,
        Query(params): Query<GetPolicyTemplatesParams>, // Añadimos los parámetros de query
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/policy-templates");

        match policy_templates_service.get_all_policy_templates(params.limit, params.page).await {
            Ok(templates) => {
                info!("Policy templates obtenidas exitosamente");
                (StatusCode::OK, Json(templates)).into_response()
            }
            Err(e) => {
                error!("Error al obtener policy templates: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                ).into_response()
            }
        }
    }

    async fn get_policy_template_by_id(
        State((policy_templates_service, _notification_service)): State<(Arc<T>, Arc<U>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/policy-templates/{}", id);

        match policy_templates_service.get_policy_template_by_id(id.clone()).await {
            Ok(Some(template)) => {
                info!("Policy template encontrada exitosamente");
                (StatusCode::OK, Json(template)).into_response()
            }
            Ok(None) => {
                info!("Policy template no encontrada");
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Policy template not found" })),
                ).into_response()
            }
            Err(e) => {
                error!("Error al obtener policy template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                ).into_response()
            }
        }
    }
}
