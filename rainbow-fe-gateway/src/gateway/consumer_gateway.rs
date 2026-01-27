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
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get, post};
use axum::{Json, Router};
use rainbow_common::config::services::GatewayConfig;
use rainbow_common::config::traits::CommonConfigTrait;
use rainbow_common::config::types::HostType;
use reqwest::Client;
use rust_embed::Embed;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

pub struct RainbowConsumerGateway {
    config: GatewayConfig,
    client: Client,
    notification_tx: broadcast::Sender<String>,
}

#[derive(Embed)]
#[folder = "src/static/consumer/dist"]
pub struct RainbowConsumerReactApp;

impl RainbowConsumerGateway {
    pub fn new(config: GatewayConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
        let (notification_tx, _) = broadcast::channel(100);
        Self { config, client, notification_tx }
    }
    pub fn router(self) -> Router {
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);

        let mut router = Router::new()
            .route(
                "/gateway/api/:service_prefix/*extra",
                any(Self::proxy_handler_with_extra),
            )
            .route("/gateway/api/:service_prefix", any(Self::proxy_handler_without_extra))
            .route("/gateway/api/ws", get(Self::websocket_handler))
            .route("/incoming-notification", post(Self::incoming_notification));

        if self.config.is_production() {
            router = router
                .route("/fe-config", get(Self::config_handler))
                .fallback(Self::static_path_handler);
        }

        router.layer(cors).with_state((self.config, self.client, self.notification_tx))
    }

    pub async fn static_path_handler(uri: Uri) -> impl IntoResponse {
        let mut path = uri.path().trim_start_matches('/').to_string();
        if path.is_empty() {
            path = "index.html".to_string();
        }

        match RainbowConsumerReactApp::get(&path) {
            Some(content) => {
                let mime_type = mime_guess::from_path(&path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime_type.as_ref())
                    .body(Body::from(content.data))
                    .unwrap()
            }
            None => match RainbowConsumerReactApp::get("index.html") {
                Some(content) => {
                    let mime_type = mime_guess::from_path("index.html").first_or_octet_stream();
                    Response::builder()
                        .header(header::CONTENT_TYPE, mime_type.as_ref())
                        .body(Body::from(content.data))
                        .unwrap()
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("<h1>404</h1><p>index.html not found</p>"))
                    .unwrap(),
            },
        }
    }

    async fn config_handler(
        State((config, _client, _notification_tx)): State<(
            GatewayConfig,
            Client,
            broadcast::Sender<String>,
        )>,
    ) -> impl IntoResponse {
        let gateway_host = config.common().hosts.http.url.clone();
        let gateway_port = config.common().hosts.http.port.clone().unwrap_or("80".to_string());
        let config_role = config.common().role.to_string();
        let json = json!({
            "gateway_host": gateway_host,
            "gateway_port": gateway_port,
            "config_role": config_role,
        });
        (StatusCode::OK, Json(json).into_response())
    }

    async fn proxy_handler_with_extra(
        State((config, client, notification_tx)): State<(
            GatewayConfig,
            Client,
            broadcast::Sender<String>,
        )>,
        Path((service_prefix, extra)): Path<(String, String)>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        Self::execute_proxy((config, client, notification_tx), service_prefix, Some(extra), req)
            .await
    }
    async fn proxy_handler_without_extra(
        State((config, client, notification_tx)): State<(
            GatewayConfig,
            Client,
            broadcast::Sender<String>,
        )>,
        Path(service_prefix): Path<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        Self::execute_proxy((config, client, notification_tx), service_prefix, None, req).await
    }
    async fn execute_proxy(
        (config, client, _notification_tx): (GatewayConfig, Client, broadcast::Sender<String>),
        service_prefix: String,
        extra_opt: Option<String>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let microservice_base_url = match service_prefix.as_str() {
            "dataplane" => config.transfer().get_host(HostType::Http),
            "subscriptions" => config.transfer().get_host(HostType::Http),
            "catalog-bypass" => config.catalog().get_host(HostType::Http),
            "catalogs" => config.catalog().get_host(HostType::Http),
            "datasets" => config.catalog().get_host(HostType::Http),
            // TODO Carlos
            "data-services" => config.transfer().get_host(HostType::Http),
            "distributions" => config.transfer().get_host(HostType::Http),
            "request" => config.transfer().get_host(HostType::Http),

            "contract-negotiation" => config.contracts().get_host(HostType::Http),
            "mates" => config.ssi_auth().get_host(HostType::Http),
            "negotiations" => config.contracts().get_host(HostType::Http),
            "transfers" => config.transfer().get_host(HostType::Http),
            "wallet" => config.ssi_auth().get_host(HostType::Http),
            "ssi-auth" => config.ssi_auth().get_host(HostType::Http),
            _ => return (StatusCode::NOT_FOUND, "prefix not found").into_response(),
        };

        execute_proxy(client, microservice_base_url, service_prefix, extra_opt, req).await
    }
    async fn websocket_handler(
        State((_config, _client, notification_tx)): State<(
            GatewayConfig,
            Client,
            broadcast::Sender<String>,
        )>,
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
        State((_config, _client, notification_tx)): State<(
            GatewayConfig,
            Client,
            broadcast::Sender<String>,
        )>,
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
