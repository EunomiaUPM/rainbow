use axum::extract::ws::Message;
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tracing::error;

pub struct BusinessNotificationsRouter {
    config: ApplicationProviderConfig,
    client: Client,
    notification_tx: broadcast::Sender<String>,
}

#[derive(Debug, Deserialize)]
struct Params {
    callback_address: Option<String>, // Use Option<String> to make the parameter optional
}

impl BusinessNotificationsRouter {
    pub fn new(config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        let (notification_tx, _) = broadcast::channel(100);
        Self { config, client, notification_tx }
    }
    pub fn router(self) -> Router {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(AllowHeaders::list([CONTENT_TYPE, AUTHORIZATION]));
        Router::new()
            .route("/gateway/api/ws", get(Self::websocket_handler))
            .route("/gateway/api/subscriptions", get(Self::subscription_handler))
            .route("/incoming-notification", post(Self::incoming_notification))
            .layer(cors)
            .with_state((self.config, self.client, self.notification_tx))
    }
    async fn subscription_handler(
        State((config, client, _notification_tx)): State<(ApplicationProviderConfig, Client, broadcast::Sender<String>)>,
        Query(params): Query<Params>,
    ) -> axum::response::Response {
        let callback_address = match params.callback_address {
            Some(cb) => cb,
            None => return (StatusCode::BAD_REQUEST, "Callback address not found as parameter").into_response()
        };
        let base_url = config.get_contract_negotiation_host_url().expect("no cn host");
        let subscription_url = format!("{}/api/v1/contract-negotiation/subscriptions", base_url);
        let request = client
            .post(&subscription_url)
            .json(&json!({
                "callbackAddress": callback_address
            }))
            .send()
            .await;
        let backend_res = match request {
            Ok(request) => request,
            Err(e) => {
                error!("Error on subscribing. Microservice not available{}", e);
                return (StatusCode::BAD_REQUEST, format!("Error on subscribing. Microservice not available {}", e)).into_response();
            }
        };
        if !backend_res.status().is_success() {
            return (StatusCode::BAD_REQUEST, format!("Error on subscribing. Status {}", backend_res.status())).into_response();
        }
        let status = backend_res.status();
        let version = backend_res.version();
        let headers = backend_res.headers().clone();

        let axum_body = axum::body::Body::from_stream(backend_res.bytes_stream());
        let mut response_builder = axum::response::Response::builder().status(status).version(version);
        if let Some(response_headers_mut) = response_builder.headers_mut() {
            *response_headers_mut = headers;
        }

        response_builder.body(axum_body).unwrap_or_else(|e| {
            error!("Error construyendo la respuesta del proxy: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del proxy".to_string(),
            )
                .into_response()
        })
    }
    async fn websocket_handler(
        State((_config, _client, notification_tx)): State<(ApplicationProviderConfig, Client, broadcast::Sender<String>)>,
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
        State((_config, _client, notification_tx)): State<(ApplicationProviderConfig, Client, broadcast::Sender<String>)>,
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