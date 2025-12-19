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

use crate::protocols::dsp::facades::data_service_resolver_facade::DataServiceFacadeTrait;
use anyhow::bail;
use rainbow_catalog_agent::{DataServiceDto, DatasetDto, DistributionDto};
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::types::HostType;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::http_client::HttpClient;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::utils::get_urn_from_string;
use rainbow_negotiation_agent::AgreementDto;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct DataServiceFacadeServiceForDSProtocol {
    config: Arc<TransferConfig>,
    client: Arc<HttpClient>,
}

impl DataServiceFacadeServiceForDSProtocol {
    pub fn new(config: Arc<TransferConfig>, client: Arc<HttpClient>) -> Self {
        Self { config, client }
    }
}

#[async_trait::async_trait]
impl DataServiceFacadeTrait for DataServiceFacadeServiceForDSProtocol {
    async fn resolve_data_service_by_agreement_id(
        &self,
        agreement_id: &Urn,
        formats: Option<&DctFormats>,
    ) -> anyhow::Result<DataServiceDto> {
        let contracts_url = self.config.contracts().get_host(HostType::Http);
        let catalog_url = self.config.catalog().get_host(HostType::Http);
        let agreement_url = format!(
            "{}/api/v1/negotiation-agent/agreements/{}",
            contracts_url, agreement_id
        );
        let data_service_url = format!("{}/api/v1/catalog-agent/data-services", catalog_url);

        // resolve agreement
        let agreement = self.client.get_json::<AgreementDto>(agreement_url.as_str()).await?;
        let agreement_target = get_urn_from_string(&agreement.inner.target)?;

        // resolve dataset entity
        let datasets_url = format!(
            "{}/api/v1/catalog-agent/datasets/{}",
            catalog_url,
            agreement_target.clone()
        );
        let dataset = self.client.get_json::<DatasetDto>(datasets_url.as_str()).await?;
        let dataset_id = get_urn_from_string(&dataset.inner.id)?;

        // resolve distribution entity
        let distribution_url = format!(
            "{}/api/v1/catalog-agent/distributions/dataset/{}/format/{}",
            catalog_url,
            dataset_id.clone(),
            formats.unwrap().to_string()
        );
        let distribution = self.client.get_json::<DistributionDto>(distribution_url.as_str()).await?;
        let access_service_id = Urn::from_str(distribution.inner.id.as_str())?;

        // resolve Data service entity
        let data_service = self.client.get_json::<DataServiceDto>(data_service_url.as_str()).await?;

        Ok(data_service)
    }
}
