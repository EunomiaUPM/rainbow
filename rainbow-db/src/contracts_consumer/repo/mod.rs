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

use super::entities::cn_process;
use anyhow::Error;
use axum::async_trait;
use thiserror::Error;
use urn::Urn;

mod sql;

// pub trait CombinedRepo: ContractNegotiationConsumerProcessRepo {}
// impl<T> CombinedRepo for T
// where
//     T: ContractNegotiationConsumerProcessRepo,
// {}
//
// pub static CONTRACT_CONSUMER_REPO: Lazy<Box<dyn CombinedRepo + Send + Sync>> = Lazy::new(|| {
//     let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();
//     match repo_type.as_str() {
//         "postgres" => Box::new(ContractNegotiationRepoForSql {}),
//         "memory" => Box::new(ContractNegotiationRepoForSql {}),
//         "mysql" => Box::new(ContractNegotiationRepoForSql {}),
//         _ => panic!("Unknown REPO_TYPE: {}", repo_type),
//     }
// });


pub struct NewContractNegotiationProcess {
    pub provider_id: Option<Urn>,
    pub consumer_id: Option<Urn>,
}

pub struct EditContractNegotiationProcess {}

#[async_trait]
pub trait ContractNegotiationConsumerProcessRepo {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors>;
    async fn get_cn_process_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn get_cn_process_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors>;
    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors>;
    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors>;
    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors>;
}

#[derive(Error, Debug)]
pub enum CnErrors {
    #[error("Contract Negotiation Process not found")]
    CNProcessNotFound,
    #[error("Error fetching Contract Negotiation Process. {0}")]
    ErrorFetchingCNProcess(Error),
    #[error("Error creating Contract Negotiation Process. {0}")]
    ErrorCreatingCNProcess(Error),
    #[error("Error deleting Contract Negotiation Process. {0}")]
    ErrorDeletingCNProcess(Error),
    #[error("Error updating Contract Negotiation Process. {0}")]
    ErrorUpdatingCNProcess(Error),
}