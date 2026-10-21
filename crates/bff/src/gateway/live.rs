/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! Live updates: one WebSocket per browser, fed by the bus's event stream read as the caller.

use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{RawQuery, State};
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::StreamExt;
use tracing::{debug, warn};
use ymir::http::USER_TOKEN_HEADER;
use ymir::services::client::ClientTrait;
use ymir::types::http::{Method, StreamBody};
use ymir::utils::stream_client;

use crate::setup::context::AppContext;

/// Relays the bus to the browser; the bus applies the caller's scope, so nothing is filtered here.
pub struct LiveHandlers;

impl LiveHandlers {
    const PING_EVERY: Duration = Duration::from_secs(25);

    /// Upgrades to a WebSocket sending each visible event as one JSON text frame.
    pub async fn live(
        State(ctx): State<Arc<AppContext>>,
        RawQuery(query): RawQuery,
        headers: HeaderMap,
        ws: WebSocketUpgrade,
    ) -> Response {
        let Some(url) = ctx.proxy.events_stream_url(query.as_deref()) else {
            return (StatusCode::SERVICE_UNAVAILABLE, "no event bus configured").into_response();
        };
        let user_header = ctx
            .oauth_validator
            .as_ref()
            .map_or(USER_TOKEN_HEADER, |v| v.user_header());
        let upstream_headers = Self::upstream_headers(&headers, user_header);
        ws.on_upgrade(move |socket| Self::relay(socket, url, upstream_headers))
    }

    /// Only the user's token (the header the proxy forwards it in) carries the caller to the bus.
    fn upstream_headers(incoming: &HeaderMap, user_header: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(name) = HeaderName::from_bytes(user_header.as_bytes()) {
            if let Some(v) = incoming.get(&name) {
                headers.insert(name, v.clone());
            }
        }
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("text/event-stream"),
        );
        headers
    }

    async fn relay(mut socket: WebSocket, url: String, headers: HeaderMap) {
        let upstream = stream_client()
            .stream(Method::GET, &url, Some(headers), StreamBody::from(""), None)
            .await;
        let response = match upstream {
            Ok(r) if r.status().is_success() => r,
            Ok(r) => {
                warn!(status = %r.status(), "event bus refused the live stream");
                let _ = socket.send(Message::Close(None)).await;
                return;
            }
            Err(e) => {
                warn!(error = %e, "event bus unreachable for the live stream");
                let _ = socket.send(Message::Close(None)).await;
                return;
            }
        };

        let mut bytes = response.bytes_stream();
        let mut buffer = String::new();
        let mut ping = tokio::time::interval(Self::PING_EVERY);
        loop {
            tokio::select! {
                chunk = bytes.next() => {
                    let Some(Ok(chunk)) = chunk else { break };
                    buffer.push_str(&String::from_utf8_lossy(&chunk).replace('\r', ""));
                    while let Some(end) = buffer.find("\n\n") {
                        let frame: String = buffer.drain(..end + 2).collect();
                        if let Some(data) = Self::sse_data(&frame) {
                            if socket.send(Message::Text(data.into())).await.is_err() {
                                return;
                            }
                        }
                    }
                }
                incoming = socket.recv() => match incoming {
                    Some(Ok(Message::Close(_))) | Some(Err(_)) | None => return,
                    Some(Ok(_)) => {}
                },
                _ = ping.tick() => {
                    if socket.send(Message::Ping(Vec::new().into())).await.is_err() {
                        return;
                    }
                }
            }
        }
        debug!("event bus closed the live stream");
        let _ = socket.send(Message::Close(None)).await;
    }

    /// The `data:` lines of one SSE frame, joined; keep-alive comments carry none.
    fn sse_data(frame: &str) -> Option<String> {
        let data: Vec<&str> = frame
            .lines()
            .filter_map(|l| l.strip_prefix("data:"))
            .map(|l| l.strip_prefix(' ').unwrap_or(l))
            .collect();
        (!data.is_empty()).then(|| data.join("\n"))
    }
}
