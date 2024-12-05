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

use crate::core::{DataPlanePeer, DataPlanePeerDefaultBehavior};
use crate::implementations::ngsi_ld::NgsiLdDataPlane;
use crate::implementations::plain_http::HttpDataPlane;
use axum::extract::{Path, Request};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use rainbow_db::dataplane::repo::DATA_PLANE_REPO;
use reqwest::StatusCode;
use uuid::Uuid;

pub fn consumer_dataplane_router() -> Router {
    Router::new()
        .route(
            "/:callback_id/data/pull/:data_id",
            any(handle_dataplane_pull),
        )
        .route(
            "/:callback_id/data/push/:data_id",
            any(handle_dataplane_push),
        )
        .route(
            "/:callback_id/data/pull/:data_id/*extras",
            any(handle_dataplane_pull_extras),
        )
        .route(
            "/:callback_id/data/push/:data_id/*extras",
            any(handle_dataplane_push_extras),
        )
}

async fn handle_dataplane_pull(
    Path((callback_id, data_id)): Path<(Uuid, Uuid)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_pull(callback_id, data_id, None, request).await
}

async fn handle_dataplane_pull_extras(
    Path((callback_id, data_id, extras)): Path<(Uuid, Uuid, String)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_pull(callback_id, data_id, Some(extras), request).await
}

async fn handle_dataplane_push(
    Path((callback_id, data_id)): Path<(Uuid, Uuid)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_push(callback_id, data_id, None, request).await
}

async fn handle_dataplane_push_extras(
    Path((callback_id, data_id, extras)): Path<(Uuid, Uuid, String)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_push(callback_id, data_id, Some(extras), request).await
}

async fn dataplane_pull(
    callback_id: Uuid,
    data_id: Uuid,
    extras: Option<String>,
    request: Request,
) -> impl IntoResponse {
    let data_plane_process =
        match DATA_PLANE_REPO.get_data_plane_process_by_id_in_url(data_id).await {
            Ok(data_plane_process) => data_plane_process,
            Err(e) => return (StatusCode::BAD_REQUEST).into_response(),
        };

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer =
                DataPlanePeer::load_model_by_id(dp.id).await.unwrap();

            match data_plane_peer.role {
                ConfigRoles::Consumer => match data_plane_peer.dct_formats.action {
                    FormatAction::Pull => match data_plane_peer.dct_formats.protocol {
                        FormatProtocol::FiwareContextBroker => {
                            let res =
                                NgsiLdDataPlane::on_pull_data(*data_plane_peer, request, extras)
                                    .await;
                            res.unwrap_or_else(|_| (StatusCode::BAD_REQUEST).into_response())
                        }
                        FormatProtocol::Http => {
                            let res =
                                HttpDataPlane::on_pull_data(*data_plane_peer, request, extras)
                                    .await;
                            res.unwrap_or_else(|_| (StatusCode::BAD_REQUEST).into_response())
                        }
                        _ => {
                            todo!("not implemented yet")
                        }
                    },
                    _ => (StatusCode::BAD_REQUEST).into_response(),
                },
                _ => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn dataplane_push(
    callback_id: Uuid,
    data_id: Uuid,
    extras: Option<String>,
    request: Request,
) -> impl IntoResponse {
    let data_plane_process =
        match DATA_PLANE_REPO.get_data_plane_process_by_id_in_url(data_id).await {
            Ok(data_plane_process) => data_plane_process,
            Err(e) => return (StatusCode::BAD_REQUEST).into_response(),
        };

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer =
                DataPlanePeer::load_model_by_id(dp.id).await.unwrap();
            match data_plane_peer.role {
                ConfigRoles::Consumer => match data_plane_peer.dct_formats.action {
                    FormatAction::Push => match data_plane_peer.dct_formats.protocol {
                        FormatProtocol::FiwareContextBroker => {
                            let res =
                                NgsiLdDataPlane::on_push_data(*data_plane_peer, request, extras)
                                    .await;
                            res.unwrap_or_else(|_| (StatusCode::BAD_REQUEST).into_response())
                        }
                        _ => {
                            todo!("not implemented yet")
                        }
                    },
                    _ => (StatusCode::BAD_REQUEST).into_response(),
                },
                _ => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
