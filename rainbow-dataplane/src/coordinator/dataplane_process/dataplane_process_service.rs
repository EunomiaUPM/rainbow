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

use crate::coordinator::dataplane_process::dataplane_process::DataPlaneProcess;
use crate::coordinator::dataplane_process::DataPlaneProcessTrait;
use anyhow::anyhow;
use axum::async_trait;
use rainbow_common::adv_protocol::interplane::DataPlaneProcessState;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::dataplane::repo::{DataPlaneFieldRepo, DataPlaneProcessRepo, EditDataPlaneProcess, NewDataPlaneField, NewDataPlaneProcess};
use std::sync::Arc;
use urn::Urn;

pub struct DataPlaneProcessService<T>
where
    T: DataPlaneProcessRepo + DataPlaneFieldRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> DataPlaneProcessService<T>
where
    T: DataPlaneProcessRepo + DataPlaneFieldRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> DataPlaneProcessTrait for DataPlaneProcessService<T>
where
    T: DataPlaneProcessRepo + DataPlaneFieldRepo + Send + Sync + 'static,
{
    async fn create_dataplane_process(&self, input: DataPlaneProcess) -> anyhow::Result<DataPlaneProcess> {
        self.repo.create_data_plane_process(NewDataPlaneProcess {
            id: input.id.clone(),
            state: DataPlaneProcessState::REQUESTED,
            direction: input.process_direction,
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "ProcessAddressProtocol".to_string(),
            value: input.process_address.protocol,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "ProcessAddressUrl".to_string(),
            value: input.process_address.url,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "ProcessAddressAuth".to_string(),
            value: input.process_address.auth_type,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "ProcessAddressAuthContent".to_string(),
            value: input.process_address.auth_content,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "DownstreamHopProtocol".to_string(),
            value: input.downstream_hop.protocol,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "DownstreamHopUrl".to_string(),
            value: input.downstream_hop.url,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "DownstreamHopAuth".to_string(),
            value: input.downstream_hop.auth_type,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "DownstreamHopAuthContent".to_string(),
            value: input.downstream_hop.auth_content,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "UpstreamHopProtocol".to_string(),
            value: input.upstream_hop.protocol,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "UpstreamHopUrl".to_string(),
            value: input.upstream_hop.url,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "UpstreamHopAuth".to_string(),
            value: input.upstream_hop.auth_type,
            data_plane_process_id: input.id.clone(),
        }).await?;
        self.repo.create_data_plane_field(NewDataPlaneField {
            key: "UpstreamHopAuthContent".to_string(),
            value: input.upstream_hop.auth_content,
            data_plane_process_id: input.id.clone(),
        }).await?;

        let dataplane_process_out = self
            .get_dataplane_process_by_id(input.id.clone()).await?;

        Ok(dataplane_process_out)
    }

    async fn get_dataplane_processes(&self) -> anyhow::Result<Vec<DataPlaneProcess>> {
        let mut dp_processes_out: Vec<DataPlaneProcess> = vec![];
        let dp_processes = self.repo.get_all_data_plane_processes(None, None).await?;
        for dp_process in dp_processes {
            let id = get_urn_from_string(&dp_process.id)?;
            let dp = self.get_dataplane_process_by_id(id).await?;
            dp_processes_out.push(dp);
        }
        Ok(dp_processes_out)
    }

    async fn get_dataplane_process_by_id(&self, id: Urn) -> anyhow::Result<DataPlaneProcess> {
        let mut dataplane_process_out: DataPlaneProcess = DataPlaneProcess::default();
        let dataplane_process = self.repo
            .get_data_plane_process_by_id(id.clone())
            .await?
            .ok_or(anyhow!("no dataplane process"))?;

        dataplane_process_out.id = get_urn_from_string(&dataplane_process.id)?;
        dataplane_process_out.process_direction = dataplane_process.direction.parse()?;
        dataplane_process_out.state = dataplane_process.state.parse()?;
        dataplane_process_out.created_at = dataplane_process.created_at;
        dataplane_process_out.updated_at = dataplane_process.updated_at;

        let dataplane_process_fields = self.repo
            .get_all_data_plane_fields_by_process(id)
            .await?;
        dataplane_process_out.process_address.protocol = dataplane_process_fields
            .iter().find(|f| f.key == "ProcessAddressProtocol")
            .unwrap().value.clone();
        dataplane_process_out.process_address.url = dataplane_process_fields
            .iter().find(|f| f.key == "ProcessAddressUrl")
            .unwrap().value.clone();
        dataplane_process_out.process_address.auth_type = dataplane_process_fields
            .iter().find(|f| f.key == "ProcessAddressAuth")
            .unwrap().value.clone();
        dataplane_process_out.process_address.auth_content = dataplane_process_fields
            .iter().find(|f| f.key == "ProcessAddressAuthContent")
            .unwrap().value.clone();
        dataplane_process_out.downstream_hop.protocol = dataplane_process_fields
            .iter().find(|f| f.key == "DownstreamHopProtocol")
            .unwrap().value.clone();
        dataplane_process_out.downstream_hop.url = dataplane_process_fields
            .iter().find(|f| f.key == "DownstreamHopUrl")
            .unwrap().value.clone();
        dataplane_process_out.downstream_hop.auth_type = dataplane_process_fields
            .iter().find(|f| f.key == "DownstreamHopAuth")
            .unwrap().value.clone();
        dataplane_process_out.downstream_hop.auth_content = dataplane_process_fields
            .iter().find(|f| f.key == "DownstreamHopAuthContent")
            .unwrap().value.clone();
        dataplane_process_out.upstream_hop.protocol = dataplane_process_fields
            .iter().find(|f| f.key == "UpstreamHopProtocol")
            .unwrap().value.clone();
        dataplane_process_out.upstream_hop.url = dataplane_process_fields
            .iter().find(|f| f.key == "UpstreamHopUrl")
            .unwrap().value.clone();
        dataplane_process_out.upstream_hop.auth_type = dataplane_process_fields
            .iter().find(|f| f.key == "UpstreamHopAuth")
            .unwrap().value.clone();
        dataplane_process_out.upstream_hop.auth_content = dataplane_process_fields
            .iter().find(|f| f.key == "UpstreamHopAuthContent")
            .unwrap().value.clone();

        Ok(dataplane_process_out)
    }

    async fn set_dataplane_process_status(
        &self,
        id: Urn,
        new_state: DataPlaneProcessState,
    ) -> anyhow::Result<DataPlaneProcess> {
        let dataplane_process = self.repo
            .put_data_plane_process(id, EditDataPlaneProcess {
                state: Some(new_state)
            }).await?;
        let id = get_urn_from_string(&dataplane_process.id)?;
        let dataplane_process_out = self.
            get_dataplane_process_by_id(id).await?;
        Ok(dataplane_process_out)
    }
}
