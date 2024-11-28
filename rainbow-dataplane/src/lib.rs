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

use crate::core::{
    DataPlanePeer, DataPlanePeerCreationBehavior, DataPlanePeerDefaultBehavior, PersistModel,
};
use crate::implementations::plain_http::HttpDataPlane;
use anyhow::bail;
use implementations::fiware_context_broker::FiwareDataPlane;
use once_cell::sync::Lazy;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::config::{get_provider_url, ConfigRoles};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::{TransferProcessMessage, TransferRequestMessage};
use rainbow_common::utils::convert_uri_to_uuid;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::time::Duration;
use tracing::info;
use uuid::Uuid;

pub mod core;
pub mod data;
pub mod implementations;
pub mod proxy;

pub static DATA_PLANE_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client")
});

pub async fn bootstrap_data_plane_in_consumer(
    transfer_request: TransferRequestMessage,
) -> anyhow::Result<DataPlanePeer> {
    info!("Bootstraping consumer data plane");

    match transfer_request.format.protocol {
        FormatProtocol::FiwareContextBroker => {
            Ok(FiwareDataPlane::bootstrap_data_plane_in_consumer(transfer_request).await?)
        }
        FormatProtocol::Http => {
            Ok(HttpDataPlane::bootstrap_data_plane_in_consumer(transfer_request).await?)
        }
        _ => {
            todo!("Not implemented yet...")
        }
    }
}

pub async fn bootstrap_data_plane_in_provider(
    transfer_request: TransferRequestMessage,
    provider_pid: Uuid,
) -> anyhow::Result<DataPlanePeer> {
    info!("Bootstraping provider data plane");

    match transfer_request.format.protocol {
        FormatProtocol::FiwareContextBroker => Ok(
            FiwareDataPlane::bootstrap_data_plane_in_provider(transfer_request, provider_pid)
                .await?,
        ),
        FormatProtocol::Http => Ok(HttpDataPlane::bootstrap_data_plane_in_provider(
            transfer_request,
            provider_pid,
        )
            .await?),
        _ => {
            todo!("Not implemented yet...")
        }
    }
}

pub async fn get_data_plane_peer(data_plane_id: Uuid) -> anyhow::Result<DataPlanePeer> {
    let db_connection = get_db_connection().await;
    let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id, db_connection).await;
    match data_plane_peer {
        Ok(dp) => Ok(*dp),
        Err(_) => bail!("Failed to load data plane. Data plane not found."),
    }
}

pub async fn set_data_plane_next_hop(
    data_plane_peer: DataPlanePeer,
    provider_pid: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<DataPlanePeer> {
    info!("Setting next hop");

    match data_plane_peer.dct_formats.protocol {
        FormatProtocol::FiwareContextBroker => Ok(FiwareDataPlane::set_data_plane_next_hop(
            data_plane_peer,
            provider_pid,
            consumer_pid,
        )
            .await?),
        FormatProtocol::Http => {
            Ok(
                HttpDataPlane::set_data_plane_next_hop(data_plane_peer, provider_pid, consumer_pid)
                    .await?,
            )
        }
        _ => {
            todo!("Not implemented yet...")
        }
    }
}

pub async fn connect_to_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
    info!("Setup Connection to streaming service");
    let db_connection = get_db_connection().await;
    let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id, db_connection).await?;

    // Prevent connections on Consumer or Pull cases
    match data_plane_peer.role {
        ConfigRoles::Provider => {}
        _ => {
            return Ok(());
        }
    }
    match data_plane_peer.dct_formats.action {
        FormatAction::Push => {}
        FormatAction::Pull => {
            return Ok(());
        }
    }

    match data_plane_peer.dct_formats.protocol {
        FormatProtocol::FiwareContextBroker => {
            Ok(FiwareDataPlane::connect_to_streaming_service(data_plane_id).await?)
        }
        FormatProtocol::Http => Ok(()),
        _ => {
            todo!("Not implemented yet...")
        }
    }
}

pub async fn disconnect_from_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
    info!("Disconnecting from streaming service");
    let db_connection = get_db_connection().await;
    let peer = DataPlanePeer::load_model_by_id(data_plane_id, db_connection).await?;

    // Prevent disconnection on Consumer or Pull cases
    match peer.role {
        ConfigRoles::Provider => {}
        _ => {
            return Ok(());
        }
    }
    match peer.dct_formats.action {
        FormatAction::Push => {}
        FormatAction::Pull => {
            return Ok(());
        }
    }
    match peer.dct_formats.protocol {
        FormatProtocol::FiwareContextBroker => {
            Ok(FiwareDataPlane::disconnect_from_streaming_service(data_plane_id).await?)
        }
        FormatProtocol::Http => Ok(()),
        _ => {
            todo!("Not implemented yet...")
        }
    }
}

// ROUTES IN AXUM
// TESTS AND CLEANUP
