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

use crate::gateway::execute_proxy;
use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use axum::Router;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use reqwest::Client;
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

pub struct RainbowProviderGateway {
    config: ApplicationProviderConfig,
    client: Client,
    notification_tx: broadcast::Sender<String>,
}

impl RainbowProviderGateway {
    pub fn new(config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        let (notification_tx, _) = broadcast::channel(100);
        Self { config, client, notification_tx }
    }
    pub fn router(self) -> Router {
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

        Router::new()
            .route(
                "/gateway/api/:service_prefix/*extra",
                any(Self::proxy_handler_with_extra),
            )
            .route(
                "/gateway/api/:service_prefix",
                any(Self::proxy_handler_without_extra),
            )
            .route("/ws", get(Self::websocket_handler))
            .route("/notify-clients", get(Self::notify_clients))
            .layer(cors)
            .with_state((self.config, self.client, self.notification_tx))
    }
    async fn proxy_handler_with_extra(
        State((config, client, notification_tx)): State<(ApplicationProviderConfig, Client, broadcast::Sender<String>)>,
        Path((service_prefix, extra)): Path<(String, String)>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        Self::execute_proxy(
            (config, client, notification_tx),
            service_prefix,
            Some(extra),
            req,
        )
            .await
    }
    async fn proxy_handler_without_extra(
        State((config, client, notification_tx)): State<(ApplicationProviderConfig, Client, broadcast::Sender<String>)>,
        Path(service_prefix): Path<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        Self::execute_proxy((config, client, notification_tx), service_prefix, None, req).await
    }
    async fn execute_proxy(
        (config, client, _notification_tx): (ApplicationProviderConfig, Client, broadcast::Sender<String>),
        service_prefix: String,
        extra_opt: Option<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let microservice_base_url = match service_prefix.as_str() {
            "catalogs" => config.get_transfer_host_url(),
            "datasets" => config.get_transfer_host_url(),
            "data-services" => config.get_transfer_host_url(),
            "distributions" => config.get_transfer_host_url(),
            "contract-negotiation" => config.get_contract_negotiation_host_url(),
            "participants" => config.get_contract_negotiation_host_url(),
            "negotiations" => config.get_contract_negotiation_host_url(),
            "transfers" => config.get_transfer_host_url(),
            "auth" => config.get_auth_host_url(),
            "ssi-auth" => config.get_ssi_auth_host_url(),
            _ => return (StatusCode::NOT_FOUND, "prefix not found").into_response(),
        };
        let microservice_base_url = match microservice_base_url {
            Some(microservice_url) => microservice_url,
            None => return (StatusCode::INTERNAL_SERVER_ERROR, "prefix not configured").into_response(),
        };
        execute_proxy(
            client,
            microservice_base_url,
            service_prefix,
            extra_opt,
            req,
        )
            .await
    }
    async fn websocket_handler() -> impl IntoResponse {
        "ok"
    }
    async fn notify_clients() -> impl IntoResponse {
        "ok"
    }
}
