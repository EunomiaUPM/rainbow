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

use crate::dataplane::entities::data_plane_field;
use crate::dataplane::entities::data_plane_process;
use axum::async_trait;
use rainbow_common::adv_protocol::interplane::{DataPlaneProcessDirection, DataPlaneProcessState};
use sea_orm::DatabaseConnection;
use urn::Urn;

pub mod sql;

pub trait DataPlaneRepoFactory: DataPlaneProcessRepo + DataPlaneFieldRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewDataPlaneProcess {
    pub id: Urn,
    pub state: DataPlaneProcessState,
    pub direction: DataPlaneProcessDirection,
}

pub struct EditDataPlaneProcess {
    pub state: Option<DataPlaneProcessState>,
}

#[async_trait]
pub trait DataPlaneProcessRepo {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>>;
    async fn get_data_plane_process_by_id(
        &self,
        data_plane_process_id: Urn,
    ) -> anyhow::Result<Option<data_plane_process::Model>>;

    async fn get_data_plane_process_by_id_in_url(
        &self,
        id: Urn,
    ) -> anyhow::Result<Option<data_plane_process::Model>>;

    async fn put_data_plane_process(
        &self,
        data_plane_process_id: Urn,
        new_data_plane_process: EditDataPlaneProcess,
    ) -> anyhow::Result<data_plane_process::Model>;
    async fn create_data_plane_process(
        &self,
        new_data_plane_process: NewDataPlaneProcess,
    ) -> anyhow::Result<data_plane_process::Model>;
    async fn delete_data_plane_process(&self, data_plane_process_id: Urn) -> anyhow::Result<()>;
}

pub struct NewDataPlaneField {
    pub key: String,
    pub value: String,
    pub data_plane_process_id: Urn,
}

pub struct EditDataPlaneField {
    pub value: Option<String>,
}

#[async_trait]
pub trait DataPlaneFieldRepo {
    async fn get_all_data_plane_fields(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>>;

    async fn get_all_data_plane_fields_by_process(
        &self,
        data_plane_process_id: Urn,
    ) -> anyhow::Result<Vec<data_plane_field::Model>>;

    async fn get_data_plane_field_by_id(
        &self,
        data_plane_field_id: Urn,
    ) -> anyhow::Result<Option<data_plane_field::Model>>;

    async fn put_data_plane_field_by_id(
        &self,
        data_plane_field_id: Urn,
        new_data_plane_field: EditDataPlaneField,
    ) -> anyhow::Result<data_plane_field::Model>;

    async fn create_data_plane_field(
        &self,
        new_data_plane_field: NewDataPlaneField,
    ) -> anyhow::Result<data_plane_field::Model>;

    async fn delete_data_plane_field(&self, data_plane_field_id: Urn) -> anyhow::Result<()>;
}
