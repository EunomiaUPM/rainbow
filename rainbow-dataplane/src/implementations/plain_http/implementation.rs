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
use crate::DATA_PLANE_HTTP_CLIENT;
use anyhow::bail;
use axum::async_trait;
use axum::body::{to_bytes, Bytes};
use axum::extract::Request;
use axum::response::Response;
use rainbow_common::config::config::{get_provider_url, ConfigRoles};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::forwarding::forward_response;
use rainbow_common::protocol::transfer::TransferRequestMessage;
use rainbow_common::utils::convert_uri_to_uuid;
use reqwest::{Method, StatusCode};
use uuid::Uuid;

#[async_trait]
impl DataPlanePeerDefaultBehavior for HttpDataPlane {
    async fn bootstrap_data_plane_in_consumer(
        transfer_request: TransferRequestMessage,
    ) -> anyhow::Result<DataPlanePeer> {
        let db_connection = get_db_connection().await;
        let local_address_path = match transfer_request.format.action {
            FormatAction::Push => "/data/push",
            FormatAction::Pull => "/data/pull",
        };
        let local_address = format!(
            "{}{}/{}",
            transfer_request.callback_address,
            local_address_path,
            convert_uri_to_uuid(&transfer_request.consumer_pid)?
        );
        let mut fw = HttpDataPlane::create_data_plane_peer()
            .with_role(ConfigRoles::Consumer)
            .with_protocol(transfer_request.format.protocol)
            .with_action(transfer_request.format.action)
            .with_local_address(local_address);

        if let Some(data_address) = transfer_request.data_address {
            fw = fw.add_attribute("endpointUrl".to_string(), data_address.endpoint);
        }

        fw = *fw.persist(db_connection).await?;
        Ok(fw.inner)
    }

    async fn bootstrap_data_plane_in_provider(
        transfer_request: TransferRequestMessage,
        provider_pid: Uuid,
    ) -> anyhow::Result<DataPlanePeer> {
        let db_connection = get_db_connection().await;
        let local_address_path = match transfer_request.format.action {
            FormatAction::Push => "/data/push",
            FormatAction::Pull => "/data/pull",
        };
        let local_address = format!(
            "http://{}{}/{}",
            get_provider_url()?,
            local_address_path,
            provider_pid
        );
        let mut fw = HttpDataPlane::create_data_plane_peer()
            .with_role(ConfigRoles::Provider)
            .with_protocol(transfer_request.format.protocol)
            .with_action(transfer_request.format.action)
            .with_local_address(local_address)
            .add_attribute(
                "consumerCallback".to_string(),
                transfer_request.callback_address,
            );

        fw = *fw.persist(db_connection).await?;
        Ok(fw.inner)
    }

    async fn set_data_plane_next_hop(
        data_plane_peer: DataPlanePeer,
        provider_pid: Uuid,
        consumer_pid: Uuid,
    ) -> anyhow::Result<DataPlanePeer> {
        let db_connection = get_db_connection().await;
        if data_plane_peer.dct_formats.action == FormatAction::Push {
            bail!("Not allowed PUSH for plain HTTP implementation")
        }
        match data_plane_peer.role {
            ConfigRoles::Consumer => {
                let mut fw = HttpDataPlane::create_data_plane_peer_from_inner(data_plane_peer);
                let endpoint_url =
                    format!("http://{}/data/pull/{}", get_provider_url()?, provider_pid);
                let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                fw = *fw.persist(db_connection).await?;
                Ok(fw.inner)
            }
            ConfigRoles::Provider => {
                let mut fw = HttpDataPlane::create_data_plane_peer_from_inner(data_plane_peer);
                let endpoint_url = fw.inner.attributes.get("endpointUrl").unwrap().to_string();
                let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                fw = *fw.persist(db_connection).await?;
                Ok(fw.inner)
            }
            _ => bail!("Not supported data plane peer type..."),
        }
    }

    async fn connect_to_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        // No need for implementation
        Ok(())
    }

    async fn disconnect_from_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        // No need for implementation
        Ok(())
    }

    async fn on_pull_data(
        data_plane_peer: DataPlanePeer,
        mut request: Request,
        extras: Option<String>,
    ) -> anyhow::Result<Response> {
        let next_hop = data_plane_peer.attributes.get("nextHop").unwrap().to_string();
        let query = request.uri().query();
        let next_hop = format!(
            "{}{}{}",
            next_hop,
            extras.map(|ex| format!("/{}", ex)).unwrap_or_default(),
            query.map(|q| format!("?{}", q.to_string())).unwrap_or_default()
        );

        let body = std::mem::take(request.body_mut());
        let body_bytes = to_bytes(body, 2024) // MAX_BUFFER
            .await
            .map_err(|_| StatusCode::BAD_REQUEST);
        let res = DATA_PLANE_HTTP_CLIENT
            .request(Method::try_from(request.method())?, next_hop)
            .body(body_bytes.unwrap_or(Bytes::default()))
            .send()
            .await;
        match res {
            Ok(r) => Ok(forward_response(r).await),
            Err(_) => bail!("Not able to push data from service"),
        }
    }

    async fn on_push_data(
        data_plane_peer: DataPlanePeer,
        request: Request,
        extras: Option<String>,
    ) -> anyhow::Result<Response> {
        // No need for implementation
        Ok(Response::default())
    }
}
