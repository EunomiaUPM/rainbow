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
use crate::data::entities::data_plane_process;
use crate::implementations::ngsi_ld::NgsiLdDataPlane;
use crate::implementations::plain_http::HttpDataPlane;
use axum::extract::{Path, Request};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub fn provider_dataplane_router() -> Router {
    Router::new()
        .route(
            "/data/pull/:data_id/*extras",
            any(handle_dataplane_pull_extras),
        )
        .route(
            "/data/push/:data_id/*extras",
            any(handle_dataplane_push_extras),
        )
        .route("/data/pull/:data_id", any(handle_dataplane_pull))
        .route("/data/push/:data_id", any(handle_dataplane_push))
}

async fn handle_dataplane_pull_extras(
    Path((data_id, extras)): Path<(Uuid, String)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_pull(data_id, Some(extras), request).await
}

async fn handle_dataplane_push_extras(
    Path((data_id, extras)): Path<(Uuid, String)>,
    request: Request,
) -> impl IntoResponse {
    dataplane_push(data_id, Some(extras), request).await
}

async fn handle_dataplane_pull(Path(data_id): Path<Uuid>, request: Request) -> impl IntoResponse {
    dataplane_pull(data_id, None, request).await
}

async fn handle_dataplane_push(Path(data_id): Path<Uuid>, request: Request) -> impl IntoResponse {
    dataplane_push(data_id, None, request).await
}

async fn dataplane_pull(
    data_id: Uuid,
    extras: Option<String>,
    request: Request,
) -> impl IntoResponse {
    let db_connection = get_db_connection().await;
    // TODO Refactor DB
    let data_plane_process = data_plane_process::Entity::find()
        .filter(data_plane_process::Column::Address.contains(data_id))
        .one(db_connection)
        .await
        .unwrap();

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer =
                DataPlanePeer::load_model_by_id(dp.id, db_connection).await.unwrap();
            println!("{:#?}", data_plane_peer);

            match data_plane_peer.role {
                ConfigRoles::Provider => match data_plane_peer.dct_formats.action {
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
    data_id: Uuid,
    extras: Option<String>,
    request: Request,
) -> impl IntoResponse {
    let db_connection = get_db_connection().await;
    let data_plane_process = data_plane_process::Entity::find()
        .filter(data_plane_process::Column::Address.contains(data_id))
        .one(db_connection)
        .await
        .unwrap();

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer =
                DataPlanePeer::load_model_by_id(dp.id, db_connection).await.unwrap();
            match data_plane_peer.role {
                ConfigRoles::Provider => match data_plane_peer.dct_formats.action {
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
