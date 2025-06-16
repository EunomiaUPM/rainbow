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

use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::TryStreamExt;
use reqwest::Client;
use tracing::{debug, error, info, warn};

pub mod consumer_gateway;
pub mod provider_gateway;

pub async fn execute_proxy(
    client: Client,
    microservice_base_url: String,
    service_prefix: String,
    extra_opt: Option<String>,
    req: Request<Body>,
) -> Response {
    let method = req.method();
    let path = req.uri().path();
    let query = req.uri().query(); // Get the raw query string
    info!("{} {}", method, path);

    let microservice_api_path = match service_prefix.as_str() {
        "dataplane" => "api/v1/dataplane",
        "notifications" => "api/v1/catalog/notifications",
        "subscriptions" => "api/v1/catalog/subscriptions",
        "catalogs" => "api/v1/catalogs",
        "datasets" => "api/v1/datasets",
        "data-services" => "api/v1/data-services",
        "distributions" => "api/v1/distributions",
        "contract-negotiation" => "api/v1/contract-negotiation",
        "mates" => "api/v1/mates",
        "catalog-bypass" => "api/v1/catalog-bypass",
        "negotiations" => "api/v1/negotiations",
        "transfers" => "api/v1/transfers",
        "auth" => "api/v1/auth",
        "ssi-auth" => "api/v1/ssi-auth",
        _ => return (StatusCode::NOT_FOUND, "prefix not found").into_response(),
    };

    // Prepare target url
    let mut target_url_str = format!(
        "{}/{}",
        microservice_base_url.trim_end_matches('/'),
        microservice_api_path.trim_matches('/')
    );

    if let Some(extra_path) = extra_opt {
        let trimmed_extra = extra_path.trim_matches('/');
        if !trimmed_extra.is_empty() {
            target_url_str.push('/');
            target_url_str.push_str(trimmed_extra);
        }
    }

    // Prepare query
    if let Some(q) = query {
        target_url_str.push('?');
        target_url_str.push_str(q);
        debug!("Target URL with original query: {}", target_url_str);
    } else {
        debug!("No query parameters in original request.");
    }

    debug!("Redirecting to {}", target_url_str);
    // Prepare method
    let original_method = req.method().clone();

    // Prepare headers
    let mut original_headers = req.headers().clone();
    let headers_to_remove = [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailers",
        "transfer-encoding",
        "upgrade",
        "host",
    ];
    for header_name in headers_to_remove.iter() {
        original_headers.remove(*header_name);
    }
    if let Ok(host_val) = HeaderValue::from_str(&microservice_base_url) {
        original_headers.insert(axum::http::header::HOST, host_val);
    } else {
        warn!(
            "Host header not possible to be created Host: {}",
            microservice_base_url
        );
    }
    // X-Forwarded-*
    if let Some(client_ip) = req.extensions().get::<axum::extract::ConnectInfo<std::net::SocketAddr>>() {
        if let Ok(x_forwarded_for_val) = HeaderValue::from_str(&client_ip.0.ip().to_string()) {
            original_headers.insert("x-forwarded-for", x_forwarded_for_val);
        }
    }
    // X-Forwarded-Proto & X-Forwarded-Host
    if let Ok(val) = HeaderValue::try_from("http") {
        original_headers.insert("x-forwarded-proto", val);
    }
    if let Some(original_host) = req.headers().get(axum::http::header::HOST) {
        original_headers.insert("x-forwarded-host", original_host.clone());
    }

    // Body
    let axum_body = req.into_body();
    let stream_for_reqwest = http_body_util::BodyStream::new(axum_body)
        .try_filter_map(|frame: http_body::Frame<bytes::Bytes>| futures_util::future::ready(Ok(frame.into_data().ok())))
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>);

    let reqwest_body = reqwest::Body::wrap_stream(stream_for_reqwest);

    // Send
    let backend_request =
        client.request(original_method, target_url_str.clone()).headers(original_headers).body(reqwest_body);

    // Handle request
    match backend_request.send().await {
        Ok(backend_res) => {
            let status = backend_res.status();
            let version = backend_res.version();
            let mut headers = backend_res.headers().clone();

            for header_name in headers_to_remove.iter() {
                headers.remove(*header_name);
            }

            let axum_body = Body::from_stream(backend_res.bytes_stream());
            let mut response_builder = Response::builder().status(status).version(version);
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
        Err(e) => {
            error!(
                "Error on requesting microservice '{}': {}",
                target_url_str.clone(),
                e
            );
            (
                StatusCode::BAD_GATEWAY,
                format!(
                    "Error on requesting backend '{}': {}",
                    target_url_str.clone(),
                    e
                ),
            )
                .into_response()
        }
    }
}
