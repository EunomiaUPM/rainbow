use crate::catalog::entities::catalog;
use crate::catalog::entities::dataservice;
use crate::catalog::entities::dataset;
use crate::catalog::entities::distribution;
use crate::catalog::entities::odrl_offer;

use crate::catalog::repo::{
    CatalogRepo, CatalogRepoErrors, DataServiceRepo, DatasetRepo, DistributionRepo,
    EditCatalogModel, EditDataServiceModel, EditDatasetModel, EditDistributionModel,
    NewCatalogModel, NewDataServiceModel, NewDatasetModel, NewDistributionModel, NewOdrlOfferModel,
    OdrlOfferRepo,
};
use axum::async_trait;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::utils::get_urn;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect,
};
use urn::Urn;

pub struct CatalogRepoForSql {}

#[async_trait]
impl CatalogRepo for CatalogRepoForSql {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalogs = catalog::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match catalogs {
            Ok(catalogs) => Ok(catalogs),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn get_catalog_by_id(
        &self,
        catalog_id: Urn,
    ) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id).one(db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn put_catalog_by_id(
        &self,
        catalog_id: Urn,
        edit_catalog_model: EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let old_model = catalog::Entity::find_by_id(catalog_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::CatalogNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        };

        let mut old_active_model: catalog::ActiveModel = old_model.into();
        if let Some(foaf_home_page) = edit_catalog_model.foaf_home_page {
            old_active_model.foaf_home_page = ActiveValue::Set(Some(foaf_home_page));
        }
        if let Some(dct_conforms_to) = edit_catalog_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_catalog_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_catalog_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingCatalog(err.into())),
        }
    }

    async fn create_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let urn = new_catalog_model.id.unwrap_or_else(|| get_urn(None));
        let participant_id = get_urn(None); // TODO create participant global id (create global setup)
        let model = catalog::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            foaf_home_page: ActiveValue::Set(new_catalog_model.foaf_home_page),
            dct_conforms_to: ActiveValue::Set(new_catalog_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_catalog_model.dct_creator),
            dct_identifier: ActiveValue::Set(Some(urn.to_string())),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_catalog_model.dct_title),
            dspace_participant_id: ActiveValue::Set(Some(participant_id.to_string())),
        };
        let catalog = catalog::Entity::insert(model).exec_with_returning(db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalog(err.into())),
        }
    }

    async fn delete_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::delete_by_id(catalog_id).exec(db_connection).await;
        match catalog {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::CatalogNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingCatalog(err.into())),
        }
    }
}

#[async_trait]
impl DatasetRepo for CatalogRepoForSql {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let datasets = dataset::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_datasets_by_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Option<dataset::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn put_datasets_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        edit_dataset_model: EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let dataset_id = dataset_id.to_string();
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let old_model = dataset::Entity::find_by_id(dataset_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::DatasetNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        };

        let mut old_active_model: dataset::ActiveModel = old_model.into();
        if let Some(dct_conforms_to) = edit_dataset_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_dataset_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_dataset_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_dataset_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingDataset(err.into())),
        }
    }

    async fn create_dataset(
        &self,
        catalog_id: Urn,
        new_dataset_model: NewDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let urn = new_dataset_model.id.unwrap_or_else(|| get_urn(None));
        let model = dataset::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            dct_conforms_to: ActiveValue::Set(new_dataset_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_dataset_model.dct_creator),
            dct_identifier: ActiveValue::Set(Option::from(urn.to_string())),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_dataset_model.dct_title),
            dct_description: ActiveValue::Set(new_dataset_model.dct_description),
            catalog_id: ActiveValue::Set(catalog_id),
        };
        let dataset = dataset::Entity::insert(model).exec_with_returning(db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDataset(err.into())),
        }
    }

    async fn delete_dataset_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let dataset_id = dataset_id.to_string();
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::delete_by_id(dataset_id).exec(db_connection).await;
        match dataset {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::DatasetNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingDataset(err.into())),
        }
    }
}

#[async_trait]
impl DistributionRepo for CatalogRepoForSql {
    async fn get_all_distributions(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let distributions = distribution::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match distributions {
            Ok(distributions) => Ok(distributions),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDistribution(err.into())),
        }
    }

    async fn get_distributions_by_dataset_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(db_connection).await;
        match dataset {
            Ok(dataset) => match dataset {
                Some(dataset) => {
                    let distributions = distribution::Entity::find()
                        .filter(distribution::Column::DatasetId.eq(dataset.id))
                        .all(db_connection)
                        .await;
                    match distributions {
                        Ok(distributions) => Ok(distributions),
                        Err(err) => Err(CatalogRepoErrors::ErrorFetchingDistribution(err.into())),
                    }
                }
                None => Err(CatalogRepoErrors::DatasetNotFound),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_distribution_by_id(
        &self,
        distribution_id: Urn,
    ) -> anyhow::Result<Option<distribution::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let distribution_id = distribution_id.to_string();
        let distribution =
            distribution::Entity::find_by_id(distribution_id).one(db_connection).await;
        match distribution {
            Ok(distribution) => Ok(distribution),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDistribution(err.into())),
        }
    }

    async fn put_distribution_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
        edit_distribution_model: EditDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let distribution_id = distribution_id.to_string();
        let catalog_id = catalog_id.to_string();
        let dataset_id = dataset_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        if dataset.is_none() {
            return Err(CatalogRepoErrors::DatasetNotFound);
        }

        let old_model = distribution::Entity::find_by_id(distribution_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::DistributionNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingDistribution(err.into())),
        };
        let mut old_active_model: distribution::ActiveModel = old_model.into();
        if let Some(dct_title) = edit_distribution_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_distribution_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        if let Some(dcat_access_service) = edit_distribution_model.dcat_access_service {
            old_active_model.dcat_access_service = ActiveValue::Set(dcat_access_service);
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingDistribution(err.into())),
        }
    }

    async fn create_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        new_distribution_model: NewDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let dataset_id = dataset_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        if dataset.is_none() {
            return Err(CatalogRepoErrors::DatasetNotFound);
        }

        let urn = new_distribution_model.id.unwrap_or_else(|| get_urn(None));
        let model = distribution::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_distribution_model.dct_title),
            dct_description: ActiveValue::Set(new_distribution_model.dct_description),
            dcat_access_service: ActiveValue::Set(new_distribution_model.dcat_access_service),
            dataset_id: ActiveValue::Set(dataset_id),
        };
        let distribution =
            distribution::Entity::insert(model).exec_with_returning(db_connection).await;
        match distribution {
            Ok(distribution) => Ok(distribution),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDistribution(err.into())),
        }
    }

    async fn delete_distribution_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let dataset_id = dataset_id.to_string();
        let distribution_id = distribution_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        if dataset.is_none() {
            return Err(CatalogRepoErrors::DatasetNotFound);
        }

        let distribution =
            distribution::Entity::delete_by_id(distribution_id).exec(db_connection).await;
        match distribution {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::DistributionNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingDistribution(err.into())),
        }
    }
}

#[async_trait]
impl DataServiceRepo for CatalogRepoForSql {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let data_services = dataservice::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match data_services {
            Ok(data_services) => Ok(data_services),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn get_data_service_by_id(
        &self,
        data_service_id: Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let data_service_id = data_service_id.to_string();
        let data_service =
            dataservice::Entity::find_by_id(data_service_id).one(db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn put_data_service_by_id(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        edit_data_service_model: EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let data_service_id = data_service_id.to_string();
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let old_model = dataservice::Entity::find_by_id(data_service_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::DataServiceNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingDataService(err.into())),
        };
        let mut old_active_model: dataservice::ActiveModel = old_model.into();
        if let Some(dcat_endpoint_description) = edit_data_service_model.dcat_endpoint_description {
            old_active_model.dcat_endpoint_description =
                ActiveValue::Set(Some(dcat_endpoint_description));
        }
        if let Some(dcat_endpoint_url) = edit_data_service_model.dcat_endpoint_url {
            old_active_model.dcat_endpoint_url = ActiveValue::Set(dcat_endpoint_url);
        }
        if let Some(dct_conforms_to) = edit_data_service_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_data_service_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_data_service_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_data_service_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }

        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingDataService(err.into())),
        }
    }

    async fn create_data_service(
        &self,
        catalog_id: Urn,
        new_data_service_model: NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let catalog_id = catalog_id.to_string();
        let urn = new_data_service_model.id.unwrap_or_else(|| get_urn(None));
        let model = dataservice::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            dcat_endpoint_description: ActiveValue::Set(
                new_data_service_model.dcat_endpoint_description,
            ),
            dcat_endpoint_url: ActiveValue::Set(new_data_service_model.dcat_endpoint_url),
            dct_conforms_to: ActiveValue::Set(new_data_service_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_data_service_model.dct_creator),
            dct_identifier: ActiveValue::Set(Option::from(urn.to_string())),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_data_service_model.dct_title),
            dct_description: ActiveValue::Set(new_data_service_model.dct_description),
            catalog_id: ActiveValue::Set(catalog_id),
        };
        let data_service =
            dataservice::Entity::insert(model).exec_with_returning(db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDataService(err.into())),
        }
    }

    async fn delete_data_service_by_id(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let data_service_id = data_service_id.to_string();
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let data_service =
            dataservice::Entity::delete_by_id(data_service_id).exec(db_connection).await;
        match data_service {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::DataServiceNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingDataService(err.into())),
        }
    }
}

#[async_trait]
impl OdrlOfferRepo for CatalogRepoForSql {
    async fn get_all_odrl_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let odrl_offers = odrl_offer::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn get_all_odrl_offers_by_entity(
        &self,
        entity: Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let entity = entity.to_string();
        let odrl_offers = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(entity))
            .all(db_connection)
            .await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::find_by_id(odrl_offer_id).one(db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn create_odrl_offer(
        &self,
        entity_id: Urn,
        entity_type: String,
        new_odrl_offer_model: NewOdrlOfferModel,
    ) -> anyhow::Result<odrl_offer::Model, CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        // TODO dynamic typing
        let urn = new_odrl_offer_model.id.unwrap_or_else(|| get_urn(None));
        let model = odrl_offer::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            odrl_offer: ActiveValue::Set(new_odrl_offer_model.odrl_offers),
            entity: ActiveValue::Set(entity_id.to_string()),
            entity_type: ActiveValue::Set(entity_type),
        };
        let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offer_by_id(
        &self,
        odrl_offer_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_by_id(odrl_offer_id).exec(db_connection).await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offers_by_entity(
        &self,
        entity_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let db_connection = get_db_connection().await;
        let entity_id = entity_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_many()
            .filter(odrl_offer::Column::Entity.eq(entity_id))
            .exec(db_connection)
            .await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingOdrlOffer(err.into())),
        }
    }

    async fn get_upstream_offers(
        &self,
        entity_id: Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors> {
        todo!("get_upstream_offers")
    }
}
