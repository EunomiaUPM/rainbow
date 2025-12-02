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

use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::config::provider::config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::catalog::dataservice_definition::{
    DataService, DataServiceDcatDeclaration, DataServiceDctDeclaration,
};
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::DatahubDataset;
use rainbow_db::contracts_provider::entities::agreement;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::error;
use urn::Urn;

pub struct DataServiceFacadeServiceForDatahub {
    config: ApplicationProviderConfig,
    client: Client,
}

impl DataServiceFacadeServiceForDatahub {
    pub fn new(config: ApplicationProviderConfig) -> Self {
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
    async fn resolve_data_service_by_agreement_id(
        &self,
        agreement_id: Urn,
        _formats: Option<DctFormats>,
    ) -> anyhow::Result<DataService> {
        let contracts_url = self.config.get_contract_negotiation_host_url().unwrap();
        let catalog_url = self.config.get_catalog_host_url().unwrap();
        let agreement_url = format!(
            "{}/api/v1/contract-negotiation/agreements/{}",
            contracts_url, agreement_id
        );

        // resolve agreement
        let response = self.client.get(&agreement_url).send().await.map_err(|_e| {
            let e = CommonErrors::missing_resource_new(&agreement_id.to_string(), "Agreement not resolvable");
            error!("{}", e.log());
            return e;
        })?;
        let status = response.status();
        if !status.is_success() {
            let e = CommonErrors::missing_resource_new(&agreement_id.to_string(), "Agreement not resolvable");
            error!("{}", e.log());
            bail!(e);
        }
        let agreement = match response.json::<agreement::Model>().await {
            Ok(agreement) => agreement,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Agreement not serializable: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        let agreement = match OdrlAgreement::try_from(agreement) {
            Ok(agreement) => agreement,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("ODRL Agreement not compliant: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        let agreement_target = agreement.target;

        // resolve dataset entity
        let datasets_url = format!(
            "{}/api/v1/datahub/domains/datasets/{}",
            catalog_url, agreement_target
        );
        let response = self.client.get(&datasets_url).send().await.map_err(|_e| {
            let e = CommonErrors::missing_resource_new(&agreement_target.to_string(), "Dataset not resolvable");
            error!("{}", e.log());
            return e;
        })?;
        let status = response.status();
        if !status.is_success() {
            let e = CommonErrors::missing_resource_new(&agreement_target.to_string(), "Dataset not resolvable");
            error!("{}", e.log());
            bail!(e);
        }
        let dataset = match response.json::<DatahubDataset>().await {
            Ok(dataset) => dataset,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Dataset not serializable: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        let endpoint_url = dataset
            .custom_properties
            .iter()
            .find(|c| c.to_owned().0 == "access_url")
            .ok_or_else(|| {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    "No access point defined for this dataset",
                );
                error!("{}", e.log());
                anyhow!(e)
            })?
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
