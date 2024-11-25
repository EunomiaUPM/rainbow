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
use crate::data::entities::{data_plane_field, data_plane_process};
use anyhow::bail;
use axum::async_trait;
use axum::extract::Request;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::TransferRequestMessage;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct DataPlanePeer {
    pub id: Uuid,
    pub role: ConfigRoles,
    pub local_address: Option<String>,
    pub dct_formats: DctFormats,
    pub attributes: HashMap<String, String>,
}

#[async_trait]
pub trait PersistModel<T> {
    async fn persist(self, db_connection: &DbConn) -> anyhow::Result<Box<Self>>;
}

#[async_trait]
pub trait DataPlanePeerDefaultBehavior {
    async fn bootstrap_data_plane_in_consumer(
        transfer_request: TransferRequestMessage,
    ) -> anyhow::Result<DataPlanePeer>;
    async fn bootstrap_data_plane_in_provider(
        transfer_request: TransferRequestMessage,
        provider_pid: Uuid,
    ) -> anyhow::Result<DataPlanePeer>;

    async fn set_data_plane_next_hop(
        data_plane_peer: DataPlanePeer,
        provider_pid: Uuid,
    ) -> anyhow::Result<DataPlanePeer>;

    async fn connect_to_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()>;
    async fn disconnect_from_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()>;
    async fn on_pull_data(data_plane_peer: DataPlanePeer, request: Request) -> anyhow::Result<axum::response::Response>;
    async fn on_push_data(data_plane_peer: DataPlanePeer, request: Request) -> anyhow::Result<axum::response::Response>;
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

impl DataPlanePeerCreationBehavior for DataPlanePeer {
    fn create_data_plane_peer() -> Self {
        Self::default()
    }

    fn create_data_plane_peer_from_inner(inner: DataPlanePeer) -> Self {
        Self::default()
    }

    fn with_role(mut self, role: ConfigRoles) -> Self {
        self.role = role;
        self
    }

    fn with_local_address(mut self, local_address: String) -> Self {
        self.local_address = Some(local_address);
        self
    }

    fn with_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.attributes.extend(attributes);
        self
    }

    fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    fn delete_attribute(mut self, key: String) -> Self {
        self.attributes.remove(&key);
        self
    }

    fn with_action(mut self, action: FormatAction) -> Self {
        self.dct_formats.action = action;
        self
    }

    fn with_protocol(mut self, protocol: FormatProtocol) -> Self {
        self.dct_formats.protocol = protocol;
        self
    }
}

impl Default for DataPlanePeer {
    fn default() -> DataPlanePeer {
        Self {
            id: Uuid::new_v4(),
            role: ConfigRoles::Consumer,
            local_address: None,
            dct_formats: DctFormats {
                protocol: FormatProtocol::FiwareContextBroker,
                action: FormatAction::Pull,
            },
            attributes: HashMap::new(),
        }
    }
}

impl DataPlanePeer {
    pub(crate) async fn load_model_by_id(
        id: Uuid,
        db_connection: &DbConn,
    ) -> anyhow::Result<Box<Self>> {
        let peer = data_plane_process::Entity::find_by_id(id)
            .one(db_connection)
            .await?;
        if peer.is_none() {
            bail!("Could not find dataPlaneDataPlan with id {}", id)
        }
        let peer = peer.unwrap();
        let mut fw_peer = Self {
            id: peer.id,
            role: peer.role.parse()?,
            local_address: Option::from(peer.address),
            dct_formats: DctFormats {
                protocol: peer.dct_action_protocol.parse()?,
                action: peer.dct_action_format.parse()?,
            },
            attributes: Default::default(),
        };

        let attributes = data_plane_field::Entity::find()
            .filter(data_plane_field::Column::DataPlaneProcessId.eq(id))
            .all(db_connection)
            .await?;

        for attr in attributes {
            fw_peer = fw_peer.add_attribute(attr.key.to_string(), attr.value.to_string());
        }

        println!("{:?}", fw_peer);

        Ok(Box::new(fw_peer))
    }
}
