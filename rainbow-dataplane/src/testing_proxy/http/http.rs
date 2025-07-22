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

use crate::coordinator::dataplane_process::DataPlaneProcessTrait;
use axum::body::{to_bytes, Body};
use axum::extract::{Path, Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use hyper::Method;
use rainbow_common::adv_protocol::interplane::{DataPlaneProcessDirection, DataPlaneProcessState};
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::utils::get_urn_from_string;
use reqwest::Response as ReqwestResponse;
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use tracing::info;

pub struct TestingHTTPProxy<T>
where
    T: DataPlaneProcessTrait + Send + Sync + 'static,
{
    client: Client,
    config: ApplicationGlobalConfig,
    dataplane_service: Arc<T>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct TestingHTTPProxyState<T>
where
    T: DataPlaneProcessTrait + Send + Sync + 'static,
{
    client: Client,
    config: ApplicationGlobalConfig,
    dataplane_service: Arc<T>,
}

impl<T> TestingHTTPProxy<T>
where
    T: DataPlaneProcessTrait + Send + Sync + 'static,
{
    pub fn new(config: ApplicationGlobalConfig, dataplane_service: Arc<T>) -> Self {
        let client = reqwest::Client::new();
        Self { client, config, dataplane_service }
    }
    pub fn router(self) -> Router {
        Router::new().route("/data/:data_plane_id", any(Self::forward_request)).with_state((
            self.client,
            self.config,
            self.dataplane_service,
        ))
    }

    async fn forward_request(
        State((client, _config, dataplane_service)): State<(Client, ApplicationGlobalConfig, Arc<T>)>,
        Path(data_plane_id): Path<String>,
        mut req: Request,
    ) -> impl IntoResponse {
        info!("* /data/{}", data_plane_id);
        // validations
        let data_plane_id = match get_urn_from_string(&data_plane_id) {
            Ok(data_plane_id) => data_plane_id,
            Err(_) => return (StatusCode::BAD_REQUEST, "data_plane_id not urn").into_response()
        };

        // PDP
        let dataplane = match dataplane_service.
            get_dataplane_process_by_id(data_plane_id).await {
            Ok(dataplane) => dataplane,
            Err(_) => return (StatusCode::BAD_REQUEST, "dataplane id not found").into_response()
        };
        match dataplane.process_direction {
            DataPlaneProcessDirection::PULL => {}
            _ => return (StatusCode::BAD_REQUEST, "wrong direction").into_response()
        }
        match dataplane.state {
            DataPlaneProcessState::STARTED => {}
            DataPlaneProcessState::REQUESTED => return (StatusCode::FORBIDDEN, "state requested").into_response(),
            DataPlaneProcessState::STOPPED => return (StatusCode::FORBIDDEN, "state stopped").into_response(),
            DataPlaneProcessState::TERMINATED => return (StatusCode::FORBIDDEN, "state terminated").into_response(),
        }

        // ODRL Evaluation here!!!!!
        // if you are Provider
        // ODRL Evaluator facade


        // forward request downstream
        let next_hop = dataplane.downstream_hop.url;
        let body = std::mem::take(req.body_mut());
        let body_bytes = match to_bytes(body, 2024) // MAX_BUFFER
            .await {
            Ok(body_bytes) => body_bytes,
            Err(_) => return (StatusCode::BAD_REQUEST, "body too big").into_response()
        };
        let method = match Method::try_from(req.method()) {
            Ok(method) => method,
            Err(_) => return (StatusCode::BAD_REQUEST, "method not allowed").into_response()
        };
        let res = client
            .request(method, next_hop)
            .body(body_bytes)
            .send()
            .await;


        // Notify && transfer event
        // Notification
        // Create TransferEvent

        // forward request upstream
        match res {
            Ok(res) => forward_response(res),
            Err(_) => return (StatusCode::BAD_REQUEST, "peer connection problem").into_response()
        }
    }
}

pub fn forward_response(reqwest_response: ReqwestResponse) -> Response {
    let status = reqwest_response.status();
    let headers = reqwest_response.headers().clone();
    let body_stream = reqwest_response.bytes_stream();
    let body = Body::from_stream(body_stream);
    let mut response = Response::builder().status(status);
    let response_headers = response.headers_mut().unwrap();
    for (key, value) in headers.iter() {
        response_headers.insert(key, value.clone());
    }

    response.body(body).unwrap()
}