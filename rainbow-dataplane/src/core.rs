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
use crate::data_plane_peer::DataPlanePeer;
use axum::async_trait;
use axum::extract::Request;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::TransferRequestMessage;
use std::collections::HashMap;
use urn::Urn;

#[async_trait]
pub trait DataPlanePersistenceBehavior<T> {
    async fn persist(self) -> anyhow::Result<Box<Self>>;
}

#[async_trait]
pub trait DataPlanePeerDefaultBehavior: Send + Sync {
    async fn bootstrap_data_plane_in_consumer(
        transfer_request: TransferRequestMessage,
    ) -> anyhow::Result<DataPlanePeer>;
    async fn bootstrap_data_plane_in_provider(
        transfer_request: TransferRequestMessage,
        provider_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer>;

    async fn set_data_plane_next_hop(
        data_plane_peer: DataPlanePeer,
        provider_pid: Urn,
        consumer_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer>;

    async fn connect_to_streaming_service(data_plane_id: Urn) -> anyhow::Result<()>;
    async fn disconnect_from_streaming_service(data_plane_id: Urn) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DataPlanePeerTransferBehavior: Send + Sync {
    async fn on_pull_data(
        data_plane_peer: DataPlanePeer,
        request: Request,
        extras: Option<String>,
    ) -> anyhow::Result<axum::response::Response>;
    async fn on_push_data(
        data_plane_peer: DataPlanePeer,
        request: Request,
        extras: Option<String>,
    ) -> anyhow::Result<axum::response::Response>;
}

pub trait DataPlanePeerCreationBehavior {
    fn create_data_plane_peer() -> Self;
    fn create_data_plane_peer_from_inner(inner: DataPlanePeer) -> Self;
    fn with_role(self, role: ConfigRoles) -> Self;
    fn with_local_address(self, local_address: String) -> Self;
    fn with_attributes(self, attributes: HashMap<String, String>) -> Self;
    fn add_attribute(self, key: String, value: String) -> Self;
    fn delete_attribute(self, key: String) -> Self;
    fn with_action(self, action: FormatAction) -> Self;
    fn with_protocol(self, protocol: FormatProtocol) -> Self;
}
