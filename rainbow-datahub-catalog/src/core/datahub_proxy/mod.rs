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

use crate::core::datahub_proxy::datahub_proxy_types::DatahubDomain;
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, Tag};
use async_trait::async_trait;

pub mod datahub_proxy;
pub mod datahub_proxy_types;

#[mockall::automock]
#[async_trait]
pub trait DatahubProxyTrait: Send + Sync + 'static {
    async fn get_datahub_domains(&self) -> anyhow::Result<Vec<DatahubDomain>>;
    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>>;
    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset>;
    async fn get_datahub_tags(&self) -> anyhow::Result<Vec<Tag>>;
}
