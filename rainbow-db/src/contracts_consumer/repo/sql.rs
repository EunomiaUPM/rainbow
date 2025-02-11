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

use super::super::entities::cn_process;
use crate::contracts_consumer::repo::{CnErrors, ContractNegotiationConsumerProcessRepo, EditContractNegotiationProcess, NewContractNegotiationProcess};
use axum::async_trait;
use urn::Urn;

pub struct ContractNegotiationRepoForSql {}

#[async_trait]
impl ContractNegotiationConsumerProcessRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_processes_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_processes_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        todo!()
    }

    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        todo!()
    }

    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        todo!()
    }

    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors> {
        todo!()
    }
}