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

use crate::consumer::core::data_plane_facade::DataPlaneConsumerFacadeTrait;
use axum::async_trait;

pub struct DataPlaneConsumerFacadeImpl {}
impl DataPlaneConsumerFacadeImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DataPlaneConsumerFacadeTrait for DataPlaneConsumerFacadeImpl {
    async fn on_transfer_request(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_start(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_suspension(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_completion(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_termination(&self) -> anyhow::Result<()> {
        Ok(())
    }
}