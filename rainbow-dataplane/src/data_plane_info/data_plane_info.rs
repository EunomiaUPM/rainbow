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
use crate::coordinator::dataplane_process::{DataPlaneProcessAddress, DataPlaneProcessTrait};
use crate::data_plane_info::DataPlaneInfoTrait;
use axum::async_trait;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::config::ConfigRoles;
use std::sync::Arc;
use urn::Urn;

pub struct DataPlaneInfoService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    dataplane_process: Arc<T>,
    config: ApplicationGlobalConfig,
}
impl<T> DataPlaneInfoService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    pub fn new(dataplane_process: Arc<T>, config: ApplicationGlobalConfig) -> Self {
        Self { dataplane_process, config }
    }
}

#[async_trait]
impl<T> DataPlaneInfoTrait for DataPlaneInfoService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    async fn get_data_plane_info_by_session_id(&self, session_id: Urn) -> anyhow::Result<DataPlaneProcess> {
        let mut dataplane = self.dataplane_process
            .get_dataplane_process_by_id(session_id)
            .await?;
        if self.config.role == ConfigRoles::Consumer {
            dataplane.downstream_hop = DataPlaneProcessAddress::default();
            dataplane.upstream_hop = DataPlaneProcessAddress::default();
        }
        Ok(dataplane)
    }
}
