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

//! OrchestrationPersistenceForProtocol building the DSP catalog from mocked entity services,
//! one batched call per kind of entity.

use std::sync::Arc;

use catalog_agent::protocols::dsp::orchestrator::protocol::persistence::OrchestrationPersistenceForProtocol;
use catalog_agent::protocols::dsp::types::catalog_definition::CatalogCatalogTypes;
use catalog_agent::services::catalogs::MockCatalogServiceTrait;
use catalog_agent::services::data_services::MockDataServiceServiceTrait;
use catalog_agent::services::datasets::MockDatasetServiceTrait;
use catalog_agent::services::distributions::MockDistributionServiceTrait;
use catalog_agent::services::odrl_policies::MockOdrlPolicyServiceTrait;
use common::paginated_spec::{Paginated, MAX_PAGE_LIMIT};
use common::test_utils::grpc::TENANT;
use common::test_utils::scopes::TestUsers;
use mockall::Sequence;

use catalog_agent::protocols::dsp::types::catalog_definition::{
    CatalogDatasetTypes, CatalogServiceTypes,
};
use catalog_agent::protocols::dsp::types::dataset_definition::DatasetDistributionTypes;

use crate::support::builders::{
    catalog_dto, data_service_dto, dataset_dto, distribution_dto, odrl_policy_dto,
};
use crate::support::fixtures::urn;

/// Every sub-catalog reaches the DSP answer, following the cursor past the first page.
#[tokio::test]
async fn catalog_lists_sub_catalogs_from_every_page() {
    let mut catalogs = MockCatalogServiceTrait::new();
    catalogs
        .expect_get_main_catalog()
        .returning(|_| Ok(Some(catalog_dto(1))));
    let mut seq = Sequence::new();
    catalogs
        .expect_get_all_catalogs()
        .withf(|_, filter, page, _| {
            filter.with_main_catalog == Some(false)
                && page.limit == MAX_PAGE_LIMIT
                && page.cursor.is_none()
        })
        .times(1)
        .in_sequence(&mut seq)
        .returning(|_, _, _, _| {
            let items = (2..=101).map(catalog_dto).collect();
            Ok(Paginated::new(items, Some("next".to_string()), Some(120)))
        });
    catalogs
        .expect_get_all_catalogs()
        .withf(|_, _, page, _| page.cursor.as_deref() == Some("next"))
        .times(1)
        .in_sequence(&mut seq)
        .returning(|_, _, _, _| {
            let items = (102..=121).map(catalog_dto).collect();
            Ok(Paginated::new(items, None, Some(120)))
        });

    let mut data_services = MockDataServiceServiceTrait::new();
    data_services
        .expect_get_main_data_service()
        .returning(|_| Ok(Some(data_service_dto(50))));
    data_services
        .expect_get_data_services_by_catalog_ids()
        .withf(|_, ids| ids.len() == 120)
        .times(1)
        .returning(|_, _| Ok(vec![]));

    let mut datasets = MockDatasetServiceTrait::new();
    datasets
        .expect_get_datasets_by_catalog_ids()
        .withf(|_, ids| ids.len() == 121)
        .times(1)
        .returning(|_, _| Ok(vec![]));

    let persistence = OrchestrationPersistenceForProtocol::new(
        Arc::new(catalogs),
        Arc::new(data_services),
        Arc::new(datasets),
        Arc::new(MockOdrlPolicyServiceTrait::new()),
        Arc::new(MockDistributionServiceTrait::new()),
    );
    let catalog = persistence
        .get_catalog(&TestUsers::alone(TENANT))
        .await
        .unwrap();

    match catalog.catalogs {
        CatalogCatalogTypes::CatalogMultipleMinimized(subs) => assert_eq!(subs.len(), 120),
        CatalogCatalogTypes::CatalogMultipleOriginal(_) => {
            panic!("expected minimized sub-catalogs")
        }
    }
}

/// Datasets of the main and a sub catalog get their offers, distributions and services from
/// one call per kind of entity.
#[tokio::test]
async fn catalog_is_assembled_from_one_batch_per_entity() {
    let mut catalogs = MockCatalogServiceTrait::new();
    catalogs
        .expect_get_main_catalog()
        .returning(|_| Ok(Some(catalog_dto(1))));
    catalogs
        .expect_get_all_catalogs()
        .times(1)
        .returning(|_, _, _, _| Ok(Paginated::new(vec![catalog_dto(2)], None, Some(1))));

    let mut datasets = MockDatasetServiceTrait::new();
    datasets
        .expect_get_datasets_by_catalog_ids()
        .times(1)
        .returning(|_, _| {
            let in_catalog = |n, catalog| {
                let mut dto = dataset_dto(n);
                dto.inner.catalog_id = urn(catalog);
                dto
            };
            Ok(vec![
                in_catalog(10, 1),
                in_catalog(11, 2),
                in_catalog(12, 2),
            ])
        });

    let mut offers = MockOdrlPolicyServiceTrait::new();
    offers
        .expect_get_odrl_offers_by_entities()
        .withf(|_, ids| ids.len() == 3)
        .times(1)
        .returning(|_, _| {
            let on = |n, dataset| {
                let mut dto = odrl_policy_dto(n, "Dataset");
                dto.inner.entity = urn(dataset);
                dto
            };
            Ok(vec![on(40, 10), on(41, 12), on(42, 12)])
        });

    let mut distributions = MockDistributionServiceTrait::new();
    distributions
        .expect_get_distributions_by_dataset_ids()
        .times(1)
        .returning(|_, _| {
            let mut dto = distribution_dto(20);
            dto.inner.dataset_id = urn(11);
            dto.inner.dcat_access_service = urn(30);
            Ok(vec![dto])
        });

    let mut data_services = MockDataServiceServiceTrait::new();
    data_services
        .expect_get_main_data_service()
        .returning(|_| Ok(Some(data_service_dto(50))));
    data_services
        .expect_get_batch_data_services()
        .times(1)
        .returning(|_, _| Ok(vec![data_service_dto(30)]));
    data_services
        .expect_get_data_services_by_catalog_ids()
        .times(1)
        .returning(|_, _| {
            let mut dto = data_service_dto(31);
            dto.inner.catalog_id = urn(2);
            Ok(vec![dto])
        });

    let persistence = OrchestrationPersistenceForProtocol::new(
        Arc::new(catalogs),
        Arc::new(data_services),
        Arc::new(datasets),
        Arc::new(offers),
        Arc::new(distributions),
    );
    let catalog = persistence
        .get_catalog(&TestUsers::alone(TENANT))
        .await
        .unwrap();

    let CatalogDatasetTypes::DatasetMultipleMinimized(main) = catalog.datasets else {
        panic!("expected minimized datasets")
    };
    assert_eq!(main.len(), 1);
    assert_eq!(main[0].id, urn(10));
    assert_eq!(main[0].odrl_offer.len(), 1);

    let CatalogCatalogTypes::CatalogMultipleMinimized(subs) = catalog.catalogs else {
        panic!("expected minimized sub-catalogs")
    };
    let CatalogDatasetTypes::DatasetMultipleMinimized(sub) = &subs[0].datasets else {
        panic!("expected minimized datasets")
    };
    assert_eq!(
        sub.iter().map(|d| d.id.clone()).collect::<Vec<_>>(),
        vec![urn(11), urn(12)]
    );
    assert_eq!(sub[1].odrl_offer.len(), 2);
    let DatasetDistributionTypes::DistributionMultipleMinimized(dists) = &sub[0].distribution
    else {
        panic!("expected minimized distributions")
    };
    assert_eq!(dists.len(), 1);
    let CatalogServiceTypes::ServiceMultiple(services) = &subs[0].data_services else {
        panic!("expected the sub-catalog's services")
    };
    assert_eq!(services.len(), 1);
}
