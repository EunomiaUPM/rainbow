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
use rainbow_common::protocol::catalog::dataservice_definition::{DataService, DataServiceDcatDeclaration, DataServiceDctDeclaration};
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::DatahubDataset;
use rainbow_db::contracts_provider::entities::agreement;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub struct DataServiceFacadeServiceForDatahub {
    config: TransferProviderApplicationConfig,
    client: Client,
}

impl DataServiceFacadeServiceForDatahub {
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
impl DataServiceFacadeTrait for DataServiceFacadeServiceForDatahub {
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
            "{}/api/v1/datahub/domains/datasets/{}",
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
        let dataset = match response.json::<DatahubDataset>().await {
            Ok(dataset) => dataset,
            Err(_) => bail!("Dataset not deserializable")
        };
        let endpoint_url = dataset.custom_properties.iter().find(|c| c.to_owned().0 == "access_url")
            .ok_or(anyhow!("No access point defined for this dataset"))?
            .clone()
            .1;

        // TODO define rest of fields for datahub
        let data_service = DataService {
            context: ContextField::default(),
            _type: "DataService".to_string(),
            id: dataset.urn,
            dcat: DataServiceDcatDeclaration {
                theme: "".to_string(),
                keyword: "".to_string(),
                endpoint_description: "".to_string(),
                endpoint_url,
            },
            dct: DataServiceDctDeclaration {
                conforms_to: None,
                creator: None,
                identifier: "".to_string(),
                issued: Default::default(),
                modified: None,
                title: None,
                description: vec![],
            },
            odrl_offer: vec![],
            extra_fields: Default::default(),
        };


        Ok(data_service)
    }
}
