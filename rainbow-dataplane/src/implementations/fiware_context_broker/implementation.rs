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

use crate::core::{DataPlanePeer, DataPlanePeerCreationBehavior, DataPlanePeerDefaultBehavior, PersistModel};
use crate::implementations::fiware_context_broker::FiwareDataPlane;
use crate::DATA_PLANE_HTTP_CLIENT;
use anyhow::bail;
use axum::async_trait;
use axum::body::to_bytes;
use axum::extract::Request;
use rainbow_common::config::config::{get_provider_url, ConfigRoles};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::forwarding::forward_response;
use rainbow_common::protocol::transfer::TransferRequestMessage;
use rainbow_common::utils::convert_uri_to_uuid;
use reqwest::{Method, StatusCode};
use serde_json::Value;
use uuid::Uuid;

#[async_trait]
impl DataPlanePeerDefaultBehavior for FiwareDataPlane {
    async fn bootstrap_data_plane_in_consumer(transfer_request: TransferRequestMessage) -> anyhow::Result<DataPlanePeer> {
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
        let mut fw = FiwareDataPlane::create_data_plane_peer()
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

    async fn bootstrap_data_plane_in_provider(transfer_request: TransferRequestMessage, provider_pid: Uuid) -> anyhow::Result<DataPlanePeer> {
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
        let mut fw = FiwareDataPlane::create_data_plane_peer()
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

    async fn set_data_plane_next_hop(data_plane_peer: DataPlanePeer, provider_pid: Uuid, consumer_pid: Uuid) -> anyhow::Result<DataPlanePeer> {
        let db_connection = get_db_connection().await;
        match data_plane_peer.role {
            ConfigRoles::Consumer => {
                let mut fw = FiwareDataPlane::create_data_plane_peer_from_inner(data_plane_peer);
                match fw.inner.dct_formats.action {
                    // And action push
                    FormatAction::Push => {
                        let endpoint_url =
                            fw.inner.attributes.get("endpointUrl").unwrap().to_string();
                        let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                        fw = *fw.persist(db_connection).await?;
                        Ok(fw.inner)
                    }
                    // Or action pull
                    FormatAction::Pull => {
                        let endpoint_url = format!(
                            "http://{}/data/pull/{}",
                            get_provider_url()?,
                            provider_pid
                        );
                        let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                        fw = *fw.persist(db_connection).await?;
                        Ok(fw.inner)
                    }
                }
            }
            ConfigRoles::Provider => {
                let mut fw = FiwareDataPlane::create_data_plane_peer_from_inner(data_plane_peer);
                match fw.inner.dct_formats.action {
                    FormatAction::Push => {
                        let consumer_callback = fw
                            .inner
                            .attributes
                            .get("consumerCallback")
                            .unwrap()
                            .to_string();
                        let endpoint_url = format!(
                            "{}/data/push/{}",
                            consumer_callback,
                            consumer_pid
                        );
                        let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                        fw = *fw.persist(db_connection).await?;
                        Ok(fw.inner)
                    }
                    FormatAction::Pull => {
                        let endpoint_url =
                            fw.inner.attributes.get("endpointUrl").unwrap().to_string();
                        let mut fw = fw.add_attribute("nextHop".to_string(), endpoint_url);
                        fw = *fw.persist(db_connection).await?;
                        Ok(fw.inner)
                    }
                }
            }
            _ => bail!("Not supported data plane peer type..."),
        }
    }

    async fn connect_to_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id, db_connection).await?;

        let mut fw = FiwareDataPlane::create_data_plane_peer_from_inner(*data_plane_peer);
        let description = fw
            .inner
            .attributes
            .get("endpointDescription")
            .unwrap()
            .to_string();
        let url = fw.inner.attributes.get("endpointUrl").unwrap().to_string();
        let mut description_as_json = serde_json::from_str::<Value>(description.as_str())?;
        // let local_address = fw
        //     .inner
        //     .local_address
        //     .clone()
        //     .unwrap()
        //     .replace(get_provider_url()?.as_str(), "host.docker.internal:1234");
        let local_address = fw
            .inner
            .local_address.clone()
            .unwrap();

        if let Some(url) = description_as_json
            .get_mut("notification")
            .and_then(|notification| notification.get_mut("http"))
            .and_then(|http| http.get_mut("url"))
        {
            *url = serde_json::json!(local_address);
        } else {
            bail!("Key path 'notification.http.url' not found");
        }

        let res = DATA_PLANE_HTTP_CLIENT
            .post(url)
            .json(&description_as_json)
            .send()
            .await?;
        if res.status() != StatusCode::CREATED {
            bail!("not able to connect to streaming service")
        }

        let suscription_id = res.headers().get("location");
        if suscription_id.is_none() {
            // TODO error
            bail!("not able to connect to streaming service")
        }
        let suscription_id = suscription_id.unwrap().to_str()?;
        let mut fw = fw.add_attribute("suscriptionId".to_string(), suscription_id.to_string());

        *fw.persist(db_connection).await?;
        Ok(())
    }

    async fn disconnect_from_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id, db_connection).await?;
        let fw = FiwareDataPlane::create_data_plane_peer_from_inner(*data_plane_peer);
        let endpoint_url = fw
            .inner
            .attributes
            .get("endpointUrl")
            .unwrap()
            .to_string()
            .replace("/v2/subscriptions", "");
        let endpoint_path = fw
            .inner
            .attributes
            .get("suscriptionId")
            .unwrap()
            .to_string();
        let endpoint = format!("{}{}", endpoint_url, endpoint_path);
        let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint).send().await?;
        let fw = fw.delete_attribute("suscriptionId".to_string());

        fw.persist(db_connection).await?;
        Ok(())
    }

    async fn on_pull_data(data_plane_peer: DataPlanePeer, request: Request, extras: Option<String>) -> anyhow::Result<axum::response::Response> {
        let next_hop = data_plane_peer.attributes.get("nextHop").unwrap();
        match request.method() {
            &Method::GET => {
                let res = DATA_PLANE_HTTP_CLIENT.get(next_hop).send().await;
                match res {
                    Ok(r) => Ok(forward_response(r).await),
                    Err(_) => bail!("Not able to pull data from service"),
                }
            }
            _ => bail!("Method not supported"),
        }
    }

    async fn on_push_data(data_plane_peer: DataPlanePeer, mut request: Request, extras: Option<String>) -> anyhow::Result<axum::response::Response> {
        let next_hop = data_plane_peer.attributes.get("nextHop").unwrap();
        match request.method() {
            &Method::POST => {
                let body = std::mem::take(request.body_mut());
                let body_bytes = to_bytes(body, 2024) // MAX_BUFFER
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST);
                let res = DATA_PLANE_HTTP_CLIENT.post(next_hop).body(body_bytes.unwrap()).send().await;
                match res {
                    Ok(r) => Ok(forward_response(r).await),
                    Err(_) => bail!("Not able to push data from service"),
                }
            }
            _ => bail!("Method not supported"),
        }
    }
}