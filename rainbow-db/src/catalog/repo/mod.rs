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

// TODO end up with this repo file and entities

use super::entities::catalog;
use super::entities::dataset;
use axum::async_trait;
use urn::Urn;

pub mod sql;

pub struct NewCatalogModel {
    pub id: Option<Urn>,
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

pub struct EditCatalogModel {
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

#[async_trait]
pub trait CatalogRepo {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>>;
    async fn get_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<Option<catalog::Model>>;
    async fn put_catalog_by_id(&self, catalog_id: Urn, edit_catalog_model: EditCatalogModel) -> anyhow::Result<catalog::Model>;
    async fn create_catalog(&self, catalog_id: Urn, new_catalog_model: NewCatalogModel) -> anyhow::Result<catalog::Model>;
    async fn delete_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<()>;
}


pub struct NewDatasetModel {
    pub id: Option<Urn>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}

pub struct EditDatasetModel {
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}

#[async_trait]
pub trait DatasetRepo {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>>;
    async fn get_datasets_by_id(&self, dataset_id: Urn) -> anyhow::Result<Option<dataset::Model>>;
    async fn put_datasets_by_id(&self, dataset_id: Urn, edit_dataset_model: EditDatasetModel) -> anyhow::Result<dataset::Model>;
    async fn create_dataset(&self, catalog_id: Urn, new_dataset_model: NewDatasetModel) -> anyhow::Result<dataset::Model>;
    async fn delete_dataset_by_id(&self, dataset_id: Urn) -> anyhow::Result<()>;
}
