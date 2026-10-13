/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::entities::catalogs::CatalogDto;
use crate::entities::data_services::DataServiceDto;
use crate::entities::datasets::DatasetDto;
use crate::entities::distributions::DistributionDto;
use crate::entities::filters::CatalogFilter;
use crate::entities::odrl_policies::OdrlPolicyDto;
use crate::protocols::dsp::types::catalog_definition::{
    Catalog, CatalogCatalogTypes, CatalogDSpaceDeclaration, CatalogDatasetTypes,
    CatalogDcatDeclaration, CatalogDctDeclaration, CatalogFoafDeclaration, CatalogMinimized,
    CatalogServiceTypes,
};
use crate::protocols::dsp::types::dataservice_definition::{
    DataService, DataServiceDcatDeclaration, DataServiceDctDeclaration,
};
use crate::protocols::dsp::types::dataset_definition::{
    Dataset, DatasetDcatDeclaration, DatasetDctDeclaration, DatasetDistributionTypes,
};
use crate::protocols::dsp::types::distribution_definition::{
    Distribution, DistributionDcatDeclaration, DistributionDctDeclaration,
};
use crate::services::catalogs::CatalogServiceTrait;
use crate::services::data_services::DataServiceServiceTrait;
use crate::services::datasets::DatasetServiceTrait;
use crate::services::distributions::DistributionServiceTrait;
use crate::services::odrl_policies::OdrlPolicyServiceTrait;
use common::oauth::UserInfo;
use common::dsp_common::context_field::ContextField;
use common::dsp_common::odrl::{OdrlOffer, OdrlPolicyInfo, OdrlTypes};
use common::paginated_spec::{Page, MAX_PAGE_LIMIT};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

pub struct OrchestrationPersistenceForProtocol {
    pub catalog_entities_service: Arc<dyn CatalogServiceTrait>,
    pub data_service_entities_service: Arc<dyn DataServiceServiceTrait>,
    pub dataset_entities_service: Arc<dyn DatasetServiceTrait>,
    pub odrl_policies_service: Arc<dyn OdrlPolicyServiceTrait>,
    pub distributions_entity_service: Arc<dyn DistributionServiceTrait>,
}

impl OrchestrationPersistenceForProtocol {
    pub fn new(
        catalog_entities_service: Arc<dyn CatalogServiceTrait>,
        data_service_entities_service: Arc<dyn DataServiceServiceTrait>,
        dataset_entities_service: Arc<dyn DatasetServiceTrait>,
        odrl_policies_service: Arc<dyn OdrlPolicyServiceTrait>,
        distributions_entity_service: Arc<dyn DistributionServiceTrait>,
    ) -> Self {
        Self {
            catalog_entities_service,
            data_service_entities_service,
            dataset_entities_service,
            odrl_policies_service,
            distributions_entity_service,
        }
    }

    /// The main catalog with its sub-catalogs, datasets and services: one query per kind of
    /// entity, whatever the number of datasets.
    #[tracing::instrument(level = "info", skip_all, err, fields(user = %user.id()))]
    pub async fn get_catalog(&self, user: &UserInfo) -> Outcome<Catalog> {
        let main_catalog_dto = self.fetch_main_catalog_dto(user).await?;
        let main_catalog_urn = Urn::from_str(&main_catalog_dto.inner.id)?;
        let main_dataservice = self.map_data_service(self.fetch_main_dataservice_dto(user).await?);
        let sub_catalogs = self.fetch_sub_catalogs(user, &main_catalog_urn).await?;
        let sub_ids = sub_catalogs
            .iter()
            .map(|c| Urn::from_str(&c.inner.id))
            .collect::<Result<Vec<_>, _>>()?;
        let catalog_ids: Vec<Urn> = std::iter::once(main_catalog_urn).chain(sub_ids.clone()).collect();

        let dataset_dtos = self
            .dataset_entities_service
            .get_datasets_by_catalog_ids(user, &catalog_ids)
            .await?;
        let owners: Vec<String> = dataset_dtos.iter().map(|d| d.inner.catalog_id.clone()).collect();
        let mut datasets_by_catalog: HashMap<String, Vec<Dataset>> = HashMap::new();
        for (catalog_id, dataset) in owners.into_iter().zip(self.build_datasets(user, dataset_dtos).await?) {
            datasets_by_catalog.entry(catalog_id).or_default().push(dataset);
        }
        let mut services_by_catalog: HashMap<String, Vec<DataService>> = HashMap::new();
        for dto in self
            .data_service_entities_service
            .get_data_services_by_catalog_ids(user, &sub_ids)
            .await?
        {
            let catalog_id = dto.inner.catalog_id.clone();
            services_by_catalog.entry(catalog_id).or_default().push(self.map_data_service(dto));
        }

        let subs = sub_catalogs
            .into_iter()
            .map(|dto| {
                let datasets = datasets_by_catalog.remove(&dto.inner.id).unwrap_or_default();
                let services = services_by_catalog.remove(&dto.inner.id).unwrap_or_default();
                self.map_subcatalog(dto, services, datasets)
            })
            .collect();
        let datasets = datasets_by_catalog
            .remove(&main_catalog_dto.inner.id)
            .unwrap_or_default();
        Ok(self.map_main_catalog(main_catalog_dto, main_dataservice, subs, datasets))
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(user = %user.id()))]
    pub async fn get_dataset(&self, user: &UserInfo, dataset_id: &Urn) -> Outcome<Dataset> {
        let dataset_dto = self.fetch_dataset_dto(user, dataset_id).await?;
        self.build_datasets(user, vec![dataset_dto])
            .await?
            .pop()
            .ok_or_else(|| Errors::crazy("a dataset was built from nothing", None))
    }

    /// Every catalog but the main one; the DSP answer lists them all, so every page is read.
    async fn fetch_sub_catalogs(&self, user: &UserInfo, main_id: &Urn) -> Outcome<Vec<CatalogDto>> {
        let filter = CatalogFilter {
            with_main_catalog: Some(false),
            ..Default::default()
        };
        let mut page = Page {
            limit: MAX_PAGE_LIMIT,
            ..Default::default()
        };
        let mut subs = Vec::new();
        loop {
            let catalogs = self
                .catalog_entities_service
                .get_all_catalogs(user, &filter, &page, &Default::default())
                .await?;
            for catalog_dto in catalogs.items {
                if &Urn::from_str(&catalog_dto.inner.id)? != main_id {
                    subs.push(catalog_dto);
                }
            }
            match catalogs.next_cursor {
                Some(cursor) => page.cursor = Some(cursor),
                None => break,
            }
        }
        Ok(subs)
    }

    /// The datasets with their offers, distributions and access services, in their order.
    async fn build_datasets(&self, user: &UserInfo, dtos: Vec<DatasetDto>) -> Outcome<Vec<Dataset>> {
        if dtos.is_empty() {
            return Ok(vec![]);
        }
        let ids = dtos
            .iter()
            .map(|d| Urn::from_str(&d.inner.id))
            .collect::<Result<Vec<_>, _>>()?;
        let mut offers: HashMap<String, Vec<OdrlOffer>> = HashMap::new();
        // An offer that cannot be read leaves its dataset without offers, as before.
        for dto in self
            .odrl_policies_service
            .get_odrl_offers_by_entities(user, &ids)
            .await
            .unwrap_or_default()
        {
            let entity = dto.inner.entity.clone();
            offers.entry(entity).or_default().push(self.map_odrl_policy(dto)?);
        }
        let mut distributions = self.build_distributions(user, &ids).await?;
        Ok(dtos
            .into_iter()
            .map(|dto| {
                let id = dto.inner.id.clone();
                let offers = offers.remove(&id).unwrap_or_default();
                let distributions = distributions.remove(&id).unwrap_or_default();
                self.map_dataset(dto, offers, distributions)
            })
            .collect())
    }

    /// Distributions of the datasets by dataset id, each with its access service.
    async fn build_distributions(
        &self,
        user: &UserInfo,
        dataset_ids: &[Urn],
    ) -> Outcome<HashMap<String, Vec<Distribution>>> {
        let dtos = self
            .distributions_entity_service
            .get_distributions_by_dataset_ids(user, dataset_ids)
            .await?;
        let service_ids: Vec<Urn> = dtos
            .iter()
            .filter_map(|d| Urn::from_str(&d.inner.dcat_access_service).ok())
            .collect();
        let services: HashMap<String, DataService> = if service_ids.is_empty() {
            HashMap::new()
        } else {
            self.data_service_entities_service
                .get_batch_data_services(user, &service_ids)
                .await?
                .into_iter()
                .map(|dto| (dto.inner.id.clone(), self.map_data_service(dto)))
                .collect()
        };
        let mut by_dataset: HashMap<String, Vec<Distribution>> = HashMap::new();
        for dto in dtos {
            let service = services.get(&dto.inner.dcat_access_service).cloned();
            let dataset_id = dto.inner.dataset_id.clone();
            by_dataset
                .entry(dataset_id)
                .or_default()
                .push(self.map_distribution(dto, service)?);
        }
        Ok(by_dataset)
    }

    async fn fetch_main_catalog_dto(&self, user: &UserInfo) -> Outcome<CatalogDto> {
        match self
            .catalog_entities_service
            .get_main_catalog(user)
            .await?
        {
            Some(c) => Ok(c),
            None => {
                let err = Errors::missing_resource("", "Main catalog not found", None);
                Err(err)
            }
        }
    }

    async fn fetch_main_dataservice_dto(&self, user: &UserInfo) -> Outcome<DataServiceDto> {
        match self
            .data_service_entities_service
            .get_main_data_service(user)
            .await?
        {
            Some(c) => Ok(c),
            None => {
                let err = Errors::missing_resource("", "Main dataservice not found", None);
                Err(err)
            }
        }
    }

    async fn fetch_dataset_dto(
        &self,
        user: &UserInfo,
        dataset_id: &Urn,
    ) -> Outcome<DatasetDto> {
        self.dataset_entities_service
            .get_dataset_by_id(user, dataset_id)
            .await
    }

    fn map_main_catalog(
        &self,
        dto: CatalogDto,
        main_dataservice_dto: DataService,
        catalogs: Vec<CatalogMinimized>,
        datasets: Vec<Dataset>,
    ) -> Catalog {
        Catalog {
            context: ContextField::default(),
            _type: "Catalog".to_string(),
            id: Urn::from_str(&dto.inner.id)
                .unwrap_or_else(|_| Urn::from_str("urn:error").unwrap()),
            foaf: CatalogFoafDeclaration {
                homepage: dto.inner.foaf_home_page,
            },
            dcat: CatalogDcatDeclaration {
                theme: None,
                keyword: None,
            },
            dct: CatalogDctDeclaration {
                conforms_to: dto.inner.dct_conforms_to,
                creator: dto.inner.dct_creator,
                identifier: dto.inner.id.clone(),
                issued: dto.inner.dct_issued.naive_utc(),
                modified: dto.inner.dct_modified.map(|d| d.naive_utc()),
                title: dto.inner.dct_title,
                description: vec![],
            },
            dspace: CatalogDSpaceDeclaration {
                participant_id: dto.inner.dspace_participant_id.clone(),
            },
            odrl_offer: None,
            extra_fields: Default::default(),
            catalogs: CatalogCatalogTypes::CatalogMultipleMinimized(catalogs),
            datasets: CatalogDatasetTypes::DatasetMultipleMinimized(
                datasets.iter().map(|d| d.into()).collect(),
            ),
            data_services: CatalogServiceTypes::ServiceMultipleMinimized(vec![
                main_dataservice_dto.into(),
            ]),
        }
    }

    fn map_subcatalog(
        &self,
        dto: CatalogDto,
        main_dataservice_dto: Vec<DataService>,
        datasets: Vec<Dataset>,
    ) -> CatalogMinimized {
        CatalogMinimized {
            _type: "Catalog".to_string(),
            id: Urn::from_str(&dto.inner.id)
                .unwrap_or_else(|_| Urn::from_str("urn:error").unwrap()),
            foaf: CatalogFoafDeclaration {
                homepage: dto.inner.foaf_home_page,
            },
            dcat: CatalogDcatDeclaration {
                theme: None,
                keyword: None,
            },
            dct: CatalogDctDeclaration {
                conforms_to: dto.inner.dct_conforms_to,
                creator: dto.inner.dct_creator,
                identifier: dto.inner.id.clone(),
                issued: dto.inner.dct_issued.naive_utc(),
                modified: dto.inner.dct_modified.map(|d| d.naive_utc()),
                title: dto.inner.dct_title,
                description: vec![],
            },
            dspace: CatalogDSpaceDeclaration {
                participant_id: None,
            },
            odrl_offer: None,
            extra_fields: Default::default(),
            datasets: CatalogDatasetTypes::DatasetMultipleMinimized(
                datasets.iter().map(|d| d.into()).collect(),
            ),
            data_services: CatalogServiceTypes::ServiceMultiple(main_dataservice_dto),
        }
    }

    fn map_dataset(
        &self,
        dto: DatasetDto,
        policies: Vec<OdrlOffer>,
        distributions: Vec<Distribution>,
    ) -> Dataset {
        Dataset {
            context: ContextField::default(),
            _type: "Dataset".to_string(),
            id: dto.inner.id.clone(),
            dcat: DatasetDcatDeclaration {
                theme: None,
                keyword: None,
            },
            dct: DatasetDctDeclaration {
                conforms_to: dto.inner.dct_conforms_to,
                creator: dto.inner.dct_creator,
                identifier: dto.inner.id.clone(),
                issued: dto.inner.dct_issued.naive_utc(),
                modified: dto.inner.dct_modified.map(|d| d.naive_utc()),
                title: dto.inner.dct_title,
                description: vec![],
            },
            odrl_offer: policies
                .into_iter()
                .map(OdrlOffer::without_empty_rules)
                .collect(),
            extra_fields: Default::default(),
            distribution: DatasetDistributionTypes::DistributionMultipleMinimized(
                distributions.iter().map(|d| d.into()).collect(),
            ),
        }
    }

    fn map_distribution(
        &self,
        dto: DistributionDto,
        service: Option<DataService>,
    ) -> Outcome<Distribution> {
        Ok(Distribution {
            context: ContextField::default(),
            _type: "Distribution".to_string(),
            id: dto.inner.id.clone(),
            dcat: DistributionDcatDeclaration {
                access_service: service.map(|d| CatalogServiceTypes::ServiceMinimized(d.into())),
            },
            dct: DistributionDctDeclaration {
                issued: dto.inner.dct_issued.naive_utc(),
                modified: dto.inner.dct_modified.map(|d| d.naive_utc()),
                title: dto.inner.dct_title,
                description: vec![],
                formats: dto.inner.dct_format.unwrap_or("".to_string()),
            },
            odrl_offer: vec![],
            extra_fields: Default::default(),
        })
    }

    fn map_data_service(&self, dto: DataServiceDto) -> DataService {
        DataService {
            context: ContextField::default(),
            _type: "DataService".to_string(),
            id: dto.inner.id.clone(),
            dcat: DataServiceDcatDeclaration {
                theme: None,
                keyword: None,
                endpoint_description: dto.inner.dcat_endpoint_description,
                endpoint_url: dto.inner.dcat_endpoint_url,
            },
            dct: DataServiceDctDeclaration {
                conforms_to: dto.inner.dct_conforms_to,
                creator: dto.inner.dct_creator,
                identifier: dto.inner.id.clone(),
                issued: dto.inner.dct_issued.naive_utc(),
                modified: dto.inner.dct_modified.map(|d| d.naive_utc()),
                title: dto.inner.dct_title,
                description: vec![],
            },
            odrl_offer: vec![],
            extra_fields: Default::default(),
        }
    }

    fn map_odrl_policy(&self, dto: OdrlPolicyDto) -> Outcome<OdrlOffer> {
        let odrl_info = serde_json::from_value::<OdrlPolicyInfo>(dto.inner.odrl_offer)?;

        Ok(OdrlOffer {
            id: Urn::from_str(&dto.inner.id)?,
            profile: None,
            permission: odrl_info.permission,
            obligation: odrl_info.obligation,
            _type: OdrlTypes::Offer,
            prohibition: odrl_info.prohibition,
            target: None,
            description: dto.inner.description,
        })
    }
}
