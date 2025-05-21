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
use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use crate::provider::setup::config::TransferProviderApplicationConfig;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::entities::agreement;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub struct DataServiceFacadeServiceForDSProtocol {
    config: TransferProviderApplicationConfig,
    client: Client,
}

impl DataServiceFacadeServiceForDSProtocol {
    pub fn new(config: TransferProviderApplicationConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RainbowRPCCatalogResolveDataServiceRequest {
    #[serde(rename = "dataServiceId")]
    pub data_service_id: Urn,
}

#[async_trait]
impl DataServiceFacadeTrait for DataServiceFacadeServiceForDSProtocol {
    async fn resolve_data_service_by_agreement_id(&self, agreement_id: Urn, formats: Option<DctFormats>) -> anyhow::Result<DataService> {
        let contracts_url = self.config.get_contract_negotiation_host_url().unwrap();
        let catalog_url = self.config.get_catalog_host_url().unwrap();
        let agreement_url = format!(
            "{}/api/v1/contract-negotiation/agreements/{}",
            contracts_url, agreement_id
        );
        let data_service_url = format!(
            "{}/api/v1/catalog/rpc/resolve-data-service",
            catalog_url
        );

        // resolve agreement
        let response = self.client
            .get(&agreement_url)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let status = response.status();
        if !status.is_success() {
            bail!("Agreement not resolvable")
        }
        let agreement = match response.json::<agreement::Model>().await {
            Ok(agreement) => agreement,
            Err(_) => bail!("Agreement not deserializable")
        };
        let agreement = match OdrlAgreement::try_from(agreement) {
            Ok(agreement) => agreement,
            Err(e) => bail!(e)
        };
        let agreement_target = agreement.target;

        // resolve dataset entity
        let datasets_url = format!(
            "{}/api/v1/datasets/{}",
            catalog_url,
            agreement_target
        );
        let response = self.client
            .get(&datasets_url)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let status = response.status();
        if !status.is_success() {
            bail!("Dataset not resolvable")
        }
        let dataset = match response.json::<Dataset>().await {
            Ok(dataset) => dataset,
            Err(_) => bail!("Dataset not deserializable")
        };
        let dataset_id = get_urn_from_string(&dataset.id)?;

        // resolve distribution entity
        let distribution_url = format!(
            "{}/api/v1/datasets/{}/distributions/dct-formats/{}",
            catalog_url,
            dataset_id,
            formats.unwrap().to_string()
        );
        let response = self.client
            .get(&distribution_url)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let status = response.status();
        if !status.is_success() {
            bail!("Distribution not resolvable")
        }
        let distribution = match response.json::<Distribution>().await {
            Ok(distribution) => distribution,
            Err(_) => bail!("Distribution not deserializable")
        };

        let access_service = match distribution.dcat.access_service {
            Some(access_service) => access_service,
            None => bail!("Access service not available in Distribution")
        };
        let access_service_id = get_urn_from_string(&access_service.id)?;

        // resolve Dataservice entity
        let response = self.client
            .post(&data_service_url)
            .json(&RainbowRPCCatalogResolveDataServiceRequest {
                data_service_id: access_service_id
            })
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let status = response.status();
        if !status.is_success() {
            bail!("Dataservice not resolvable")
        }
        let data_service = match response.json::<DataService>().await {
            Ok(distribution) => distribution,
            Err(_) => bail!("DataService not deserializable")
        };

        Ok(data_service)
    }
}
