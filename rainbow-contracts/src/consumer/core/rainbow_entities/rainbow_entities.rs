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
use crate::common::CNControllerTypes;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use crate::consumer::core::rainbow_entities::rainbow_entities_types::{
    EditContractNegotiationRequest, NewContractNegotiationRequest,
};
use crate::consumer::core::rainbow_entities::RainbowEntitiesContractNegotiationConsumerTrait;
use axum::async_trait;
use rainbow_db::contracts_consumer::entities::cn_process::Model;
use rainbow_db::contracts_consumer::repo::{CnErrors, ContractNegotiationConsumerProcessRepo};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowEntitiesContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowEntitiesContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowEntitiesContractNegotiationConsumerTrait for RainbowEntitiesContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    async fn get_cn_processes(&self) -> anyhow::Result<Vec<Model>> {
        let processes = self.repo.get_all_cn_processes(None, None).await.map_err(CnErrorConsumer::DbErr)?;
        Ok(processes)
    }

    async fn get_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<Model> {
        let process = self
            .repo
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(CnErrorConsumer::DbErr)?
            .ok_or(CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() })?;
        Ok(process)
    }

    async fn get_cn_process_by_provider(&self, provider_id: Urn) -> anyhow::Result<Model> {
        let process =
            self.repo.get_cn_process_by_provider_id(provider_id.clone()).await.map_err(CnErrorConsumer::DbErr)?.ok_or(
                CnErrorConsumer::ProviderNotFound { provider_id, entity: CNControllerTypes::Process.to_string() },
            )?;
        Ok(process)
    }

    async fn get_cn_process_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<Model> {
        let process =
            self.repo.get_cn_process_by_consumer_id(consumer_id.clone()).await.map_err(CnErrorConsumer::DbErr)?.ok_or(
                CnErrorConsumer::ConsumerNotFound { consumer_id, entity: CNControllerTypes::Process.to_string() },
            )?;
        Ok(process)
    }

    async fn post_cn_process(&self, input: NewContractNegotiationRequest) -> anyhow::Result<Model> {
        let process = self.repo.create_cn_process(input.into()).await.map_err(CnErrorConsumer::DbErr)?;
        Ok(process)
    }

    async fn put_cn_process(&self, process_id: Urn, input: EditContractNegotiationRequest) -> anyhow::Result<Model> {
        let process = self.repo.put_cn_process(process_id.clone(), input.into()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
        })?;
        Ok(process)
    }

    async fn delete_cn_process(&self, process_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_cn_process(process_id.clone()).await.map_err(|err| match err {
            CnErrors::CNProcessNotFound => {
                CnErrorConsumer::NotFound { id: process_id, entity: CNControllerTypes::Process.to_string() }
            }
            _ => CnErrorConsumer::DbErr(err),
        })?;
        Ok(())
    }
}
