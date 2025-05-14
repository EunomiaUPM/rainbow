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

use crate::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::core::ds_protocol::DSProtocolCatalogTrait;
use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError::ConversionError;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{CatalogRepo, CatalogRepoErrors, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;
use urn::Urn;

pub struct DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> DSProtocolCatalogTrait for DSProtocolCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn dataset_request(&self, dataset_id: Urn) -> anyhow::Result<Dataset> {
        let dataset = self.repo
            .get_datasets_by_id(dataset_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?
            .ok_or(DSProtocolCatalogErrors::NotFound {
                id: dataset_id.clone(),
                entity: "Dataset".to_string(),
            })?;

        let mut dataset = Dataset::try_from(dataset)?;

        // policies
        let odrl_offer = self.repo
            .get_all_odrl_offers_by_entity(dataset_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        let odrl_offer = odrl_offer.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect();
        dataset.odrl_offer = odrl_offer;

        // distributions
        let distributions = self.repo
            .get_distributions_by_dataset_id(dataset_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?
            .iter()
            .map(|m| Distribution::try_from(m.clone()).unwrap())
            .collect();
        dataset.distribution = distributions;
        Ok(dataset)
    }

    async fn dataset_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>> {
        let mut datasets_out: Vec<Dataset> = vec![];

        let datasets = self.repo
            .get_datasets_by_catalog_id(catalog_id.clone())
            .await
            .map_err(|e| match e {
                CatalogRepoErrors::CatalogNotFound => DSProtocolCatalogErrors::NotFound {
                    id: catalog_id.clone(),
                    entity: "Catalog".to_string(),
                },
                e => DSProtocolCatalogErrors::DbErr(e)
            })?;

        for dataset in datasets {
            let dataset_id = get_urn_from_string(&dataset.id)?;
            let mut dataset = Dataset::try_from(dataset.clone())?;
            // policies
            let odrl_offer = self.repo
                .get_all_odrl_offers_by_entity(get_urn_from_string(&dataset.id)?)
                .await
                .map_err(DSProtocolCatalogErrors::DbErr)?;
            let odrl_offer = odrl_offer.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect();
            dataset.odrl_offer = odrl_offer;
            let distributions = self.distributions_request_by_dataset(dataset_id, catalog_id.clone()).await?;
            dataset.distribution = distributions;
            datasets_out.push(dataset);
        }

        Ok(datasets_out)
    }

    async fn data_services_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<DataService>> {
        let mut data_services_out: Vec<DataService> = vec![];

        let data_services = self.repo
            .get_data_services_by_catalog_id(catalog_id.clone())
            .await
            .map_err(|e| match e {
                CatalogRepoErrors::CatalogNotFound => DSProtocolCatalogErrors::NotFound {
                    id: catalog_id.clone(),
                    entity: "Catalog".to_string(),
                },
                e => DSProtocolCatalogErrors::DbErr(e)
            })?;

        for data_service in data_services {
            let mut data_service = DataService::try_from(data_service.clone())?;
            // policies
            let odrl_offer = self.repo
                .get_all_odrl_offers_by_entity(get_urn_from_string(&data_service.id)?)
                .await
                .map_err(DSProtocolCatalogErrors::DbErr)?;
            let odrl_offer = odrl_offer.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect();
            data_service.odrl_offer = odrl_offer;
            data_services_out.push(data_service);
        }

        Ok(data_services_out)
    }

    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>> {
        let data_service = self.repo
            .get_data_service_by_id(data_service_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        let data_service = data_service.map(|d| DataService::try_from(d).unwrap());
        Ok(data_service)
    }

    async fn distributions_request_by_dataset(
        &self,
        dataset_id: Urn,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<Distribution>> {
        let mut distributions_out: Vec<Distribution> = vec![];
        let distributions = self.repo
            .get_distributions_by_dataset_id(dataset_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;

        for distribution in distributions {
            let data_service_id = get_urn_from_string(&distribution.dcat_access_service)?;
            let mut distribution = Distribution::try_from(distribution.clone())?;
            // odrl
            let odrl_offer = self.repo
                .get_all_odrl_offers_by_entity(get_urn_from_string(&distribution.id)?)
                .await
                .map_err(DSProtocolCatalogErrors::DbErr)?;
            let odrl_offer = odrl_offer.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect();
            distribution.odrl_offer = odrl_offer;
            distribution.dcat.access_service = self.data_services_request_by_id(data_service_id).await?;
            distributions_out.push(distribution);
        }

        Ok(distributions_out)
    }

    async fn catalog_request(&self) -> anyhow::Result<Catalog> {
        let main_catalog = self.repo
            .get_main_catalog().await.map_err(DSProtocolCatalogErrors::DbErr)?
            .ok_or(DSProtocolCatalogErrors::NoMainCatalog)?;
        let mut main_catalog = Catalog::try_from(main_catalog).map_err(|e| ConversionError(e))?;

        let mut catalogs_out: Vec<Catalog> = vec![];
        let catalogs = self.repo
            .get_all_catalogs(None, None, true)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;

        for catalog in catalogs {
            let mut catalog = Catalog::try_from(catalog)?;
            let id = &catalog.id;
            catalog.odrl_offer = None;
            catalog.datasets = self.dataset_request_by_catalog(id.clone()).await?;
            catalog.data_services = self.data_services_request_by_catalog(id.clone()).await?;
            catalogs_out.push(catalog);
        }

        main_catalog.catalogs = catalogs_out;
        main_catalog.odrl_offer = None;
        main_catalog.datasets = self.dataset_request_by_catalog(main_catalog.id.clone()).await?;
        main_catalog.data_services = self.data_services_request_by_catalog(main_catalog.id.clone()).await?;
        Ok(main_catalog)
    }

    async fn catalog_request_by_id(&self, catalog_id: Urn) -> anyhow::Result<Catalog> {
        let mut catalogs_out: Catalog;
        let catalog = self.repo
            .get_catalog_by_id(catalog_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?
            .ok_or(DSProtocolCatalogErrors::NotFound {
                id: catalog_id,
                entity: "Catalog".to_string(),
            })?;

        catalogs_out = Catalog::try_from(catalog.clone())?;
        let id = get_urn_from_string(&catalog.id)?;
        // odrl
        let odrl_offer = self.repo
            .get_all_odrl_offers_by_entity(id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        let odrl_offer = Some(odrl_offer.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect());
        catalogs_out.odrl_offer = odrl_offer;
        catalogs_out.datasets = self.dataset_request_by_catalog(id.clone()).await?;
        catalogs_out.data_services = self.data_services_request_by_catalog(id).await?;

        Ok(catalogs_out)
    }
}
