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

use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use urn::Urn;

pub mod data_plane_facade;

#[mockall::automock]
#[async_trait::async_trait]
#[allow(unused)]
pub trait DataPlaneProviderFacadeTrait: Send + Sync {
    async fn get_dataplane_address(&self, session_id: Urn) -> anyhow::Result<DataAddress>;
    async fn on_transfer_request(
        &self,
        session_id: Urn,
        data_service: DataService,
        format: DctFormats,
    ) -> anyhow::Result<()>;
    async fn on_transfer_start(&self, session_id: Urn) -> anyhow::Result<()>;
    async fn on_transfer_suspension(&self, session_id: Urn) -> anyhow::Result<()>;
    async fn on_transfer_completion(&self, session_id: Urn) -> anyhow::Result<()>;
    async fn on_transfer_termination(&self, session_id: Urn) -> anyhow::Result<()>;
}
