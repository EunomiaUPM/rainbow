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

use crate::gateway::execute_proxy;
use axum::body::Body;
use axum::extract::ws::Message;
use axum::extract::{Path, Request, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{any, get, get_service, post};
use axum::{Json, Router};
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

pub struct RainbowConsumerGateway {
    config: ApplicationConsumerConfig,
    client: Client,
    notification_tx: broadcast::Sender<String>,
}

impl RainbowConsumerGateway {
    pub fn new(config: ApplicationConsumerConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        let (notification_tx, _) = broadcast::channel(100);
        Self { config, client, notification_tx }
    }
    pub fn router(self) -> Router {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(Any);

        let mut router = Router::new()
            .route(
                "/gateway/api/:service_prefix/*extra",
                any(Self::proxy_handler_with_extra),
            )
            .route(
                "/gateway/api/:service_prefix",
                any(Self::proxy_handler_without_extra),
            )
            .route("/gateway/api/ws", get(Self::websocket_handler))
            .route("/incoming-notification", post(Self::incoming_notification));

        if self.config.is_gateway_in_production {
            let static_file_service = get_service(
                ServeDir::new("src/static/consumer").fallback(ServeFile::new("src/static/consumer/index.html")),
            );
            router = router.fallback_service(static_file_service);
        }

        router.layer(cors).with_state((self.config, self.client, self.notification_tx))
    }

    async fn proxy_handler_with_extra(
        State((config, client, notification_tx)): State<(ApplicationConsumerConfig, Client, broadcast::Sender<String>)>,
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
        State((config, client, notification_tx)): State<(ApplicationConsumerConfig, Client, broadcast::Sender<String>)>,
        Path(service_prefix): Path<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        Self::execute_proxy((config, client, notification_tx), service_prefix, None, req).await
    }
    async fn execute_proxy(
        (config, client, _notification_tx): (ApplicationConsumerConfig, Client, broadcast::Sender<String>),
        service_prefix: String,
        extra_opt: Option<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let microservice_base_url = match service_prefix.as_str() {
            "dataplane" => config.get_transfer_host_url(),
            "subscriptions" => config.get_transfer_host_url(),
            "catalog-bypass" => config.get_catalog_bypass_host_url(),
            // "catalogs" => config.get_catalog_host_url(),
            // "datasets" => config.get_catalog_host_url(),
            // "data-services" => config.get_catalog_host_url(),
            // "distributions" => config.get_catalog_host_url(),
            "contract-negotiation" => config.get_contract_negotiation_host_url(),
            "mates" => config.get_contract_negotiation_host_url(),
            "negotiations" => config.get_contract_negotiation_host_url(),
            "transfers" => config.get_transfer_host_url(),
            "auth" => config.get_transfer_host_url(),
            "wallet" => config.get_transfer_host_url(),
            "ssi-auth" => config.get_ssi_auth_host_url(),
            "request" => config.get_ssi_auth_host_url(),
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
    async fn websocket_handler(
        State((_config, _client, notification_tx)): State<(ApplicationConsumerConfig, Client, broadcast::Sender<String>)>,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |mut socket| async move {
            let mut notification_rx = notification_tx.subscribe();
            loop {
                tokio::select! {
                    // Forward messages from the broadcast channel to the WebSocket client
                    Ok(msg_to_send) = notification_rx.recv() => {
                        if socket.send(Message::Text(msg_to_send)).await.is_err() {
                            // Client disconnected or error sending
                            eprintln!("WS client disconnected or send error.");
                            break;
                        }
                    }
                    // Handle messages from the WebSocket client (optional)
                    Some(Ok(ws_msg)) = socket.recv() => {
                        match ws_msg {
                            Message::Text(text) => {
                                println!("Received WS message from client: {}", text);
                                // Process message from client, e.g., echo or handle command
                                // if socket.send(Message::Text(format!("Echo: {}", text))).await.is_err() {
                                //     break;
                                // }
                            }
                            Message::Binary(_) => {
                                println!("Received binary message from client.");
                            }
                            Message::Ping(ping) => {
                                if socket.send(Message::Pong(ping)).await.is_err() {
                                   break;
                                }
                            }
                            Message::Pong(_) => {
                                 // Pong received
                            }
                            Message::Close(_) => {
                                eprintln!("WS client initiated close.");
                                break;
                            }
                        }
                    }
                    // If either the broadcast channel closes or the socket.recv() returns None/Err
                    else => {
                        // This branch can be reached if notification_rx.recv() fails (e.g. channel closed)
                        // or if socket.recv() indicates the connection is closed or errored out.
                        eprintln!("WS connection or broadcast channel error/closed.");
                        break;
                    }
                }
            }
            println!("WebSocket connection handler finished.");
        })
    }
    async fn incoming_notification(
        State((_config, _client, notification_tx)): State<(ApplicationConsumerConfig, Client, broadcast::Sender<String>)>,
        Json(input): Json<Value>,
    ) -> impl IntoResponse {
        let value_str = match serde_json::to_string(&input) {
            Ok(value_str) => value_str,
            Err(_e) => return (StatusCode::BAD_REQUEST, "Not able to deserialize").into_response(),
        };
        let _req = match notification_tx.send(value_str) {
            Ok(num_receivers) => num_receivers,
            // Send Pending
            Err(_e) => return (StatusCode::BAD_REQUEST, "Not able to deserialize").into_response(),
        };
        StatusCode::ACCEPTED.into_response()
    }
}
