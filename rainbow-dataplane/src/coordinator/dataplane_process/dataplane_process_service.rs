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
use crate::coordinator::dataplane_process::dataplane_process::DataPlaneProcess;
use crate::coordinator::dataplane_process::DataPlaneProcessTrait;
use crate::entities::data_plane_process::{
    DataPlaneProcessEntitiesTrait, EditDataPlaneProcessDto, NewDataPlaneProcessDto,
};
use axum::async_trait;
use rainbow_common::adv_protocol::interplane::DataPlaneProcessState;
use std::collections::HashMap;
use std::sync::Arc;
use urn::{Urn, UrnBuilder};

pub struct DataPlaneProcessService {
    dataplane_process_entities: Arc<dyn DataPlaneProcessEntitiesTrait>,
}

impl DataPlaneProcessService {
    pub fn new(dataplane_process_entities: Arc<dyn DataPlaneProcessEntitiesTrait>) -> Self {
        Self { dataplane_process_entities }
    }
}

#[async_trait]
impl DataPlaneProcessTrait for DataPlaneProcessService {
    async fn create_dataplane_process(&self, input: &DataPlaneProcess) -> anyhow::Result<DataPlaneProcess> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert(
            "ProcessAddressProtocol",
            input.process_address.protocol.clone(),
        );
        fields.insert("ProcessAddressUrl", input.process_address.url.clone());
        fields.insert(
            "ProcessAddressAuth",
            input.process_address.auth_type.clone(),
        );
        fields.insert(
            "ProcessAddressAuthContent",
            input.process_address.auth_content.clone(),
        );
        fields.insert(
            "DownstreamHopProtocol",
            input.downstream_hop.protocol.clone(),
        );
        fields.insert("DownstreamHopUrl", input.downstream_hop.url.clone());
        fields.insert("DownstreamHopAuth", input.downstream_hop.auth_type.clone());
        fields.insert(
            "DownstreamHopAuthContent",
            input.downstream_hop.auth_content.clone(),
        );
        fields.insert("UpstreamHopProtocol", input.upstream_hop.protocol.clone());
        fields.insert("UpstreamHopUrl", input.upstream_hop.url.clone());
        fields.insert("UpstreamHopAuth", input.upstream_hop.auth_type.clone());
        fields.insert(
            "UpstreamHopAuthContent",
            input.upstream_hop.auth_content.clone(),
        );
        let fields: HashMap<String, String> = fields.iter().map(|f| (f.0.to_string(), f.1.clone())).collect();

        let urn = UrnBuilder::new(
            "dataplane-process",
            uuid::Uuid::new_v4().to_string().as_str(),
        )
        .build()?;

        let dto = self
            .dataplane_process_entities
            .create_data_plane_process(&NewDataPlaneProcessDto {
                id: urn,
                direction: input.process_direction.to_string(),
                state: DataPlaneProcessState::REQUESTED.to_string(),
                fields: Some(fields),
            })
            .await?;

        let dataplane_process = DataPlaneProcess::try_from(dto)?;

        Ok(dataplane_process)
    }

    async fn get_dataplane_processes(&self) -> anyhow::Result<Vec<DataPlaneProcess>> {
        let mut dp_processes_out: Vec<DataPlaneProcess> = vec![];
        let dp_processes = self.dataplane_process_entities.get_all_data_plane_processes(None, None).await?;
        for dp_process in dp_processes {
            let dp_process_out = DataPlaneProcess::try_from(dp_process)?;
            dp_processes_out.push(dp_process_out);
        }
        Ok(dp_processes_out)
    }

    async fn get_dataplane_process_by_id(&self, id: &Urn) -> anyhow::Result<DataPlaneProcess> {
        let dp_processes = self
            .dataplane_process_entities
            .get_data_plane_process_by_id(&id)
            .await?
            .ok_or(anyhow::anyhow!("No process with ID {}", id))?;
        let dp_process_out = DataPlaneProcess::try_from(dp_processes)?;
        Ok(dp_process_out)
    }

    async fn set_dataplane_process_status(
        &self,
        id: &Urn,
        new_state: &DataPlaneProcessState,
    ) -> anyhow::Result<DataPlaneProcess> {
        let dataplane_process = self
            .dataplane_process_entities
            .put_data_plane_process(
                &id,
                &EditDataPlaneProcessDto { state: Some(new_state.to_string()), fields: None },
            )
            .await?;
        let dp_process_out = DataPlaneProcess::try_from(dataplane_process)?;
        Ok(dp_process_out)
    }
}
