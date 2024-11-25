use crate::core::{DataPlanePeer, DataPlanePeerDefaultBehavior};
use crate::data::entities::data_plane_process;
use crate::implementations::fiware_context_broker::FiwareDataPlane;
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
        .route("/data/pull/:data_id", any(handle_dataplane_pull))
        .route("/data/push/:data_id", any(handle_dataplane_push))
}

async fn handle_dataplane_pull(Path(data_id): Path<Uuid>, request: Request) -> impl IntoResponse {
    let db_connection = get_db_connection().await;
    // TODO Refactor DB
    let data_plane_process = data_plane_process::Entity::find()
        .filter(data_plane_process::Column::Address.contains(data_id)
        )
        .one(db_connection)
        .await
        .unwrap();

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer = DataPlanePeer::load_model_by_id(dp.id, db_connection)
                .await
                .unwrap();
            println!("{:#?}", data_plane_peer);

            match data_plane_peer.role {
                ConfigRoles::Provider => match data_plane_peer.dct_formats.action {
                    FormatAction::Pull => match data_plane_peer.dct_formats.protocol {
                        FormatProtocol::FiwareContextBroker => {
                            let res = FiwareDataPlane::on_pull_data(*data_plane_peer, request).await;
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

async fn handle_dataplane_push(Path(data_id): Path<Uuid>, request: Request) -> impl IntoResponse {
    let db_connection = get_db_connection().await;
    let data_plane_process = data_plane_process::Entity::find()
        .filter(data_plane_process::Column::Address.contains(data_id)
        )
        .one(db_connection)
        .await
        .unwrap();

    match data_plane_process {
        Some(dp) => {
            let data_plane_peer = DataPlanePeer::load_model_by_id(dp.id, db_connection)
                .await
                .unwrap();
            match data_plane_peer.role {
                ConfigRoles::Provider => match data_plane_peer.dct_formats.action {
                    FormatAction::Push => match data_plane_peer.dct_formats.protocol {
                        FormatProtocol::FiwareContextBroker => {
                            let res = FiwareDataPlane::on_push_data(*data_plane_peer, request).await;
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
