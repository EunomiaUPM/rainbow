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

use axum::async_trait;
use rainbow_common::adv_protocol::interplane::data_plane_provision::{
    DataPlaneProvisionRequest, DataPlaneProvisionResponse,
};
use rainbow_common::adv_protocol::interplane::data_plane_start::{DataPlaneStart, DataPlaneStartAck};
use rainbow_common::adv_protocol::interplane::data_plane_status::{DataPlaneStatusRequest, DataPlaneStatusResponse};
use rainbow_common::adv_protocol::interplane::data_plane_stop::{DataPlaneStop, DataPlaneStopAck};

pub mod controller_errors;
pub mod controller_service;

#[async_trait]
pub trait DataPlaneControllerTrait {
    async fn data_plane_provision_request(
        &self,
        input: DataPlaneProvisionRequest,
    ) -> anyhow::Result<DataPlaneProvisionResponse>;
    async fn data_plane_start(&self, input: DataPlaneStart) -> anyhow::Result<DataPlaneStartAck>;
    async fn data_plane_stop(&self, input: DataPlaneStop) -> anyhow::Result<DataPlaneStopAck>;
    async fn data_plane_get_status(&self, input: DataPlaneStatusRequest) -> anyhow::Result<DataPlaneStatusResponse>;
}
