/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::coordinator::dataplane_process::{DataPlaneDefaultBehaviour, DataPlaneProcessAddress, DataPlaneProcessDirection, DataPlaneProcessRequest, DataPlaneProcessState};
use crate::coordinator::transfer_event::TransferEvent;
use axum::async_trait;
use rainbow_common::utils::get_urn;
use serde::Serialize;
use urn::Urn;

#[derive(Debug, Serialize)]
pub struct DataPlaneProcess {
    pub id: Urn,
    pub process_direction: DataPlaneProcessDirection,
    pub upstream_hop: DataPlaneProcessAddress,
    pub downstream_hop: DataPlaneProcessAddress,
    pub process_address: DataPlaneProcessAddress,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub state: DataPlaneProcessState,
}

impl Default for DataPlaneProcess {
    fn default() -> Self {
        Self {
            id: get_urn(None),
            process_direction: DataPlaneProcessDirection::PUSH,
            upstream_hop: DataPlaneProcessAddress::default(),
            downstream_hop: DataPlaneProcessAddress::default(),
            process_address: DataPlaneProcessAddress::default(),
            created_at: Default::default(),
            updated_at: None,
            state: DataPlaneProcessState::REQUESTED,
        }
    }
}

#[async_trait]
impl DataPlaneDefaultBehaviour for DataPlaneProcess {
    async fn create_dataplane_process(input: DataPlaneProcessRequest) -> anyhow::Result<DataPlaneProcess> {
        let process_address_protocol = input.process_address.protocol;
        let process_address_url = input.process_address.url;
        let process_address_auth_type = input.process_address.auth_type;
        let process_address_auth_content = input.process_address.auth_content;
        let downstream_hop_protocol = input.downstream_hop.protocol;
        let downstream_hop_url = input.downstream_hop.url;
        let downstream_hop_auth_type = input.downstream_hop.auth_type;
        let downstream_hop_auth_content = input.downstream_hop.auth_content;

        let data_plane_process = DataPlaneProcess {
            id: input.session_id,
            process_direction: DataPlaneProcessDirection::PULL,
            upstream_hop: DataPlaneProcessAddress {
                protocol: "".to_string(),
                url: "".to_string(),
                auth_type: "".to_string(),
                auth_content: "".to_string(),
            },
            downstream_hop: DataPlaneProcessAddress {
                protocol: downstream_hop_protocol,
                url: downstream_hop_url,
                auth_type: downstream_hop_auth_type,
                auth_content: downstream_hop_auth_content,
            },
            process_address: DataPlaneProcessAddress {
                protocol: process_address_protocol,
                url: process_address_url,
                auth_type: process_address_auth_type,
                auth_content: process_address_auth_content,
            },
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            state: DataPlaneProcessState::REQUESTED,
        };

        Ok(data_plane_process)
    }

    async fn get_dataplane_by_id(&self, _dataplane_id: Urn) -> anyhow::Result<DataPlaneProcess> {
        todo!()
    }


    async fn on_pull_data(&self, _dataplane_id: Urn, _event: TransferEvent) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_push_data(&self, _dataplane_id: Urn, _event: TransferEvent) -> anyhow::Result<()> {
        todo!()
    }

    async fn tear_down_data_plane(&self, _dataplane_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn connect_to_streaming_service(&self, _dataplane_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn disconnect_from_streaming_service(&self, _dataplane_id: Urn) -> anyhow::Result<()> {
        todo!()
    }
}

