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

use crate::consumer::core::rainbow_cn_errors::CnErrorConsumer;
use crate::consumer::core::rainbow_cn_types::{
    EditContractNegotiationRequest, NewContractNegotiationRequest,
};
use crate::provider::core::rainbow_cn_api::CNControllerTypes;
use rainbow_db::contracts_consumer::entities::cn_process;
use rainbow_db::contracts_consumer::repo::{CnErrors, CONTRACT_CONSUMER_REPO};
use urn::Urn;

pub async fn get_cn_processes() -> anyhow::Result<Vec<cn_process::Model>> {
    let processes = CONTRACT_CONSUMER_REPO
        .get_all_cn_processes(None, None)
        .await
        .map_err(CnErrorConsumer::DbErr)?;
    Ok(processes)
}

pub async fn get_cn_process_by_id(process_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_cn_id(process_id.clone())
        .await
        .map_err(CnErrorConsumer::DbErr)?
        .ok_or(CnErrorConsumer::NotFound {
            id: process_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn get_cn_process_by_provider(provider_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_provider_id(provider_id.clone())
        .await
        .map_err(CnErrorConsumer::DbErr)?
        .ok_or(CnErrorConsumer::ProviderNotFound {
            provider_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn get_cn_process_by_consumer(consumer_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(consumer_id.clone())
        .await
        .map_err(CnErrorConsumer::DbErr)?
        .ok_or(CnErrorConsumer::ConsumerNotFound {
            consumer_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn post_cn_process(
    input: NewContractNegotiationRequest,
) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_CONSUMER_REPO
        .create_cn_process(input.into())
        .await
        .map_err(CnErrorConsumer::DbErr)?;
    Ok(process)
}

pub async fn put_cn_process(
    process_id: Urn,
    input: EditContractNegotiationRequest,
) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_CONSUMER_REPO
        .put_cn_process(process_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorConsumer::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            _ => CnErrorConsumer::DbErr(err),
        })?;
    Ok(process)
}

pub async fn delete_cn_process(process_id: Urn) -> anyhow::Result<()> {
    let _ =
        CONTRACT_CONSUMER_REPO.delete_cn_process(process_id.clone()).await.map_err(
            |err| match err {
                CnErrors::CNProcessNotFound => CnErrorConsumer::NotFound {
                    id: process_id,
                    entity: CNControllerTypes::Process.to_string(),
                },
                _ => CnErrorConsumer::DbErr(err),
            },
        )?;
    Ok(())
}
