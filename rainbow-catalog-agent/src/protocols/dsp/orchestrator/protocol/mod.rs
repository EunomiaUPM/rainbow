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

pub(crate) mod protocol;

use crate::protocols::dsp::protocol_types::{
    CatalogMessageDto, CatalogMessageWrapper, CatalogRequestMessageDto, DatasetMessageDto, DatasetRequestMessage,
};

#[async_trait::async_trait]
pub trait ProtocolOrchestratorTrait: Send + Sync + 'static {
    async fn on_catalog_request(
        &self,
        input: &CatalogMessageWrapper<CatalogRequestMessageDto>,
    ) -> anyhow::Result<CatalogMessageWrapper<CatalogMessageDto>>;
    async fn on_dataset_request(
        &self,
        input: &CatalogMessageWrapper<DatasetRequestMessage>,
    ) -> anyhow::Result<CatalogMessageWrapper<DatasetMessageDto>>;
}
