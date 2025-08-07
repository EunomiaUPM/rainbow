use std::collections::HashSet;

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

use axum::async_trait;

use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::utils::get_urn;

use sea_orm::sqlx::types::uuid;
use sea_orm::Condition;
use sea_orm::SelectGetableTuple;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect,
};
use sea_orm_migration::async_trait;

use urn::Urn;

use crate::catalog::entities::catalog::Model;

use crate::catalog::entities::reference;
use crate::catalog::entities::{catalog, catalog_record, dataservice, dataset,
    dataset_series, distribution, keyword, odrl_offer, relation, qualified_relation, resource,
    theme
};

use crate::catalog::repo::KeywordThemesRepo;
use crate::catalog::repo::{CatalogRepo, CatalogRepoErrors, CatalogRepoFactory};

use crate::catalog::repo::{
    CatalogRecordRepo, DataServiceRepo, DatasetRepo, DatasetSeriesRepo, DistributionRepo,
    OdrlOfferRepo, RelationRepo, QualifiedRelationRepo, ResourceRepo, ReferenceRepo
};

use crate::catalog::repo::{
    NewCatalogModel, NewCatalogRecordModel, NewDataServiceModel, NewDatasetModel,
    NewDatasetSeriesModel, NewDistributionModel, NewOdrlOfferModel, NewRelationModel, 
    NewQualifiedRelationModel, NewReferenceModel, NewThemeModel, NewKeywordModel
};

use crate::catalog::repo::{
    EditCatalogModel, EditCatalogRecordModel, EditDataServiceModel, EditDatasetModel,
    EditDatasetSeriesModel, EditDistributionModel, EditRelationModel, EditQualifiedRelationModel,
    NewResourceModel, EditResourceModel, EditReferenceModel
};


pub struct CatalogRepoForSql {
    db_connection: DatabaseConnection,
}

impl CatalogRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl CatalogRepoFactory for CatalogRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized,
    {
        Self::new(db_connection)
    }
}

#[async_trait]
impl CatalogRepo for CatalogRepoForSql {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        no_main_catalog: bool,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        let catalogs = match no_main_catalog {
            true => {
                catalog::Entity::find()
                    .filter(catalog::Column::DspaceMainCatalog.eq(false))
                    .limit(limit.unwrap_or(100000))
                    .offset(page.unwrap_or(0))
                    .all(&self.db_connection)
                    .await
            }
            false => {
                catalog::Entity::find()
                    .limit(limit.unwrap_or(100000))
                    .offset(page.unwrap_or(0))
                    .all(&self.db_connection)
                    .await
            }
        };

        match catalogs {
            Ok(catalogs) => Ok(catalogs),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn get_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id).one(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn get_main_catalog(&self) -> anyhow::Result<Option<Model>, CatalogRepoErrors> {
        let catalog = catalog::Entity::find()
            .filter(catalog::Column::DspaceMainCatalog.eq(true))
            .one(&self.db_connection)
            .await
            .map_err(|err| CatalogRepoErrors::ErrorCreatingCatalog(err.into()))?;
        Ok(catalog)
    }

    async fn get_catalogs_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        if themes.is_empty() {
            return Ok(vec![]);
        }
        let themes = theme::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(theme::Column::Theme.is_in(themes))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingThemes(e.into()))?;
        let catalogs_ids: Vec<String> = themes // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if catalogs_ids.is_empty() {
            return Ok(vec![]);
        }
        let catalogs = catalog::Entity::find()
            .filter(catalog::Column::Id.is_in(catalogs_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        Ok(catalogs)
    }

    async fn get_catalogs_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        if keywords.is_empty() {
            return Ok(vec![]);
        }
        let keywords = keyword::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(keyword::Column::Keyword.is_in(keywords))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingKeywords(e.into()))?;
        let catalogs_ids: Vec<String> = keywords // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if catalogs_ids.is_empty() {
            return Ok(vec![]);
        }
        let catalogs = catalog::Entity::find()
            .filter(catalog::Column::Id.is_in(catalogs_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        Ok(catalogs)
    }

    async fn put_catalog_by_id(
        &self,
        catalog_id: Urn,
        edit_catalog_model: EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let old_model = self.get_catalog_by_id(catalog_id.clone()).await?
            .ok_or(CatalogRepoErrors::CatalogNotFound)?;
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
        if let Some(dct_identifier) = edit_catalog_model.dct_identifier {
            old_active_model.dct_identifier = ActiveValue::Set(dct_identifier);
        }
        if let Some(dct_issued) = edit_catalog_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(dspace_participant_id) = edit_catalog_model.dspace_participant_id {
            old_active_model.dspace_participant_id = ActiveValue::Set(Some(dspace_participant_id));
        }
        if let Some(dspace_main_catalog) = edit_catalog_model.dspace_main_catalog {
            old_active_model.dspace_main_catalog = ActiveValue::Set(dspace_main_catalog);
        }
        if let Some(dct_description) = edit_catalog_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        if let Some(dct_access_rights) = edit_catalog_model.dct_access_rights {
            old_active_model.dct_access_rights = ActiveValue::Set(Some(dct_access_rights));
        }
        if let Some(dcat_contact_point) = edit_catalog_model.dcat_contact_point {
            old_active_model.dcat_contact_point = ActiveValue::Set(Some(dcat_contact_point));
        }
        if let Some(ordl_has_policy) = edit_catalog_model.ordl_has_policy {
            old_active_model.ordl_has_policy = ActiveValue::Set(ordl_has_policy);
        }
        if let Some(dcat_landing_page) = edit_catalog_model.dcat_landing_page {
            old_active_model.dcat_landing_page = ActiveValue::Set(Some(dcat_landing_page));
        }
        if let Some(dct_licence) = edit_catalog_model.dct_licence {
            old_active_model.dct_licence = ActiveValue::Set(Some(dct_licence));
        }
        if let Some(dct_publisher) = edit_catalog_model.dct_publisher {
            old_active_model.dct_publisher = ActiveValue::Set(Some(dct_publisher));
        }
        if let Some(prov_qualified_attribution) = edit_catalog_model.prov_qualified_attribution {
            old_active_model.prov_qualified_attribution = ActiveValue::Set(Some(prov_qualified_attribution));
        }
        if let Some(dcat_has_current_version) = edit_catalog_model.dcat_has_current_version {
            old_active_model.dcat_has_current_version = ActiveValue::Set(Some(dcat_has_current_version));
        }
        if let Some(dcat_version) = edit_catalog_model.dcat_version {
            old_active_model.dcat_version = ActiveValue::Set(dcat_version);
        }
        if let Some(dcat_previous_version) = edit_catalog_model.dcat_previous_version {
            old_active_model.dcat_previous_version = ActiveValue::Set(Some(dcat_previous_version));
        }
        if let Some(adms_version_notes) = edit_catalog_model.adms_version_notes {
            old_active_model.adms_version_notes = ActiveValue::Set(Some(adms_version_notes));
        }
        if let Some(dcat_first) = edit_catalog_model.dcat_first {
            old_active_model.dcat_first = ActiveValue::Set(Some(dcat_first));
        }
        if let Some(dcat_last) = edit_catalog_model.dcat_last {
            old_active_model.dcat_last = ActiveValue::Set(Some(dcat_last));
        }
        if let Some(dcat_prev) = edit_catalog_model.dcat_prev {
            old_active_model.dcat_prev = ActiveValue::Set(Some(dcat_prev));
        }
        if let Some(dct_replaces) = edit_catalog_model.dct_replaces {
            old_active_model.dct_replaces = ActiveValue::Set(Some(dct_replaces));
        }
        if let Some(adms_status) = edit_catalog_model.adms_status {
            old_active_model.adms_status = ActiveValue::Set(Some(adms_status));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingCatalog(err.into())),
        }
    }

    async fn create_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let urn = new_catalog_model.id.unwrap_or_else(|| get_urn(None));
        let participant_id = get_urn(None); // TODO create participant global id (create global setup)
        let model = catalog::ActiveModel {
            id:ActiveValue::Set(urn.to_string()),
            foaf_home_page:ActiveValue::Set(new_catalog_model.foaf_home_page),
            dct_conforms_to:ActiveValue::Set(new_catalog_model.dct_conforms_to),
            dct_creator:ActiveValue::Set(new_catalog_model.dct_creator),
            dct_identifier:ActiveValue::Set(urn.to_string()),
            dct_issued:ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified:ActiveValue::Set(None),
            dct_title:ActiveValue::Set(new_catalog_model.dct_title),
            dspace_participant_id:ActiveValue::Set(Some(participant_id.to_string())),
            dspace_main_catalog:ActiveValue::Set(false),
            dct_description:ActiveValue::Set(new_catalog_model.dct_description),
            dct_access_rights:ActiveValue::Set(new_catalog_model.dct_access_rights),
            dcat_contact_point:ActiveValue::Set(new_catalog_model.dcat_contact_point),
            ordl_has_policy:ActiveValue::Set(new_catalog_model.ordl_has_policy),
            dcat_landing_page:ActiveValue::Set(new_catalog_model.dcat_landing_page),
            dct_licence:ActiveValue::Set(new_catalog_model.dct_licence),
            dct_publisher:ActiveValue::Set(new_catalog_model.dct_publisher),
            prov_qualified_attribution:ActiveValue::Set(new_catalog_model.prov_qualified_attribution),
            dcat_has_current_version:ActiveValue::Set(new_catalog_model.dcat_has_current_version),
            dcat_version:ActiveValue::Set(new_catalog_model.dcat_version),
            dcat_previous_version:ActiveValue::Set(new_catalog_model.dcat_previous_version),
            adms_version_notes:ActiveValue::Set(new_catalog_model.adms_version_notes),
            dcat_first:ActiveValue::Set(new_catalog_model.dcat_first),
            dcat_last:ActiveValue::Set(new_catalog_model.dcat_last),
            dcat_prev:ActiveValue::Set(new_catalog_model.dcat_prev),
            dct_replaces:ActiveValue::Set(new_catalog_model.dct_replaces),
            adms_status:ActiveValue::Set(new_catalog_model.adms_status)
        };
        let catalog = catalog::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalog(err.into())),
        }
    }

    async fn create_main_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<Model, CatalogRepoErrors> {
        let main_catalog =
            self.get_main_catalog().await.map_err(|err| CatalogRepoErrors::ErrorCreatingCatalog(err.into()))?;
        if main_catalog.is_some() {
            return Ok(main_catalog.unwrap());
        }

        let urn = new_catalog_model.id.unwrap_or_else(|| get_urn(None));
        let participant_id = get_urn(None); // TODO create participant global id (create global setup)
        let model = catalog::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            foaf_home_page: ActiveValue::Set(new_catalog_model.foaf_home_page),
            dct_conforms_to: ActiveValue::Set(new_catalog_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_catalog_model.dct_creator),
            dct_identifier: ActiveValue::Set(urn.to_string()),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_catalog_model.dct_title),
            dspace_participant_id: ActiveValue::Set(Some(participant_id.to_string())),
            dspace_main_catalog: ActiveValue::Set(true),
            dct_description: ActiveValue::Set(new_catalog_model.dct_description),
            dct_access_rights: ActiveValue::Set(new_catalog_model.dct_access_rights),
            dcat_contact_point: ActiveValue::Set(new_catalog_model.dcat_contact_point),
            ordl_has_policy: ActiveValue::Set(new_catalog_model.ordl_has_policy),
            dcat_landing_page: ActiveValue::Set(new_catalog_model.dcat_landing_page),
            dct_licence: ActiveValue::Set(new_catalog_model.dct_licence),
            dct_publisher: ActiveValue::Set(new_catalog_model.dct_publisher),
            prov_qualified_attribution: ActiveValue::Set(new_catalog_model.prov_qualified_attribution),
            dcat_has_current_version: ActiveValue::Set(new_catalog_model.dcat_has_current_version),
            dcat_version: ActiveValue::Set(new_catalog_model.dcat_version),
            dcat_previous_version: ActiveValue::Set(new_catalog_model.dcat_previous_version),
            adms_version_notes: ActiveValue::Set(new_catalog_model.adms_version_notes),
            dcat_first: ActiveValue::Set(new_catalog_model.dcat_first),
            dcat_last: ActiveValue::Set(new_catalog_model.dcat_last),
            dcat_prev: ActiveValue::Set(new_catalog_model.dcat_prev),
            dct_replaces: ActiveValue::Set(new_catalog_model.dct_replaces),
            adms_status: ActiveValue::Set(new_catalog_model.adms_status),
        };
        let catalog = catalog::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalog(err.into())),
        }
    }

    async fn delete_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::delete_by_id(catalog_id).exec(&self.db_connection).await;
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
        let datasets = dataset::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_datasets_by_catalog_id(
        &self,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let datasets =
            dataset::Entity::find().filter(dataset::Column::CatalogId.eq(catalog_id)).all(&self.db_connection).await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_datasets_by_id(&self, dataset_id: Urn) -> anyhow::Result<Option<dataset::Model>, CatalogRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(&self.db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_datasets_from_dataset_series_by_dataset_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(&self.db_connection).await;
        let dataset = match dataset {
            Ok(dataset) => match dataset {
                Some(dataset) => dataset,
                None => return Err(CatalogRepoErrors::DatasetNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
        };
        let dataset_series_id = match &dataset.dcat_inseries {
            Some(series_id) => series_id.clone(),
            None => return Ok(vec![]),
        };
        let datasets_in_series = dataset::Entity::find()
            .filter(dataset::Column::DcatInseries.eq(dataset_series_id))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        Ok(datasets_in_series)
    }
    
    async fn get_datasets_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        if themes.is_empty() {
            return Ok(vec![]);
        }
        let themes = theme::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(theme::Column::Theme.is_in(themes))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingThemes(e.into()))?;
        let datasets_ids: Vec<String> = themes // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if datasets_ids.is_empty() {
            return Ok(vec![]);
        }
        let datasets = dataset::Entity::find()
            .filter(dataset::Column::Id.is_in(datasets_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        Ok(datasets)
    }

    async fn get_datasets_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        if keywords.is_empty() {
            return Ok(vec![]);
        }
        let keywords = keyword::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(keyword::Column::Keyword.is_in(keywords))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingKeywords(e.into()))?;
        let datasets_ids: Vec<String> = keywords // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if datasets_ids.is_empty() {
            return Ok(vec![]);
        }
        let datasets = dataset::Entity::find()
            .filter(dataset::Column::Id.is_in(datasets_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        Ok(datasets)
    }
    async fn put_datasets_by_id(
        &self,
        dataset_id: Urn,
        edit_dataset_model: EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors> {
        let old_model = self.get_datasets_by_id(dataset_id.clone()).await?
            .ok_or(CatalogRepoErrors::DatasetNotFound)?;
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
        if let Some(dct_identifier) = edit_dataset_model.dct_identifier {
            old_active_model.dct_identifier = ActiveValue::Set(Some(dct_identifier));
        }
        if let Some(dct_issued) = edit_dataset_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(catalog_id) = edit_dataset_model.catalog_id {
            old_active_model.catalog_id = ActiveValue::Set(catalog_id);
        }
        if let Some(dcat_inseries) = edit_dataset_model.dcat_inseries {
            old_active_model.dcat_inseries = ActiveValue::Set(Some(dcat_inseries));
        }
        if let Some(dct_spatial) = edit_dataset_model.dct_spatial {
            old_active_model.dct_spatial = ActiveValue::Set(Some(dct_spatial));
        }
        if let Some(dcat_spatial_resolution_meters) = edit_dataset_model.dcat_spatial_resolution_meters {
            old_active_model.dcat_spatial_resolution_meters = ActiveValue::Set(Some(dcat_spatial_resolution_meters));
        }
        if let Some(dct_temporal) = edit_dataset_model.dct_temporal {
            old_active_model.dct_temporal = ActiveValue::Set(Some(dct_temporal));
        }
        if let Some(dct_temporal_resolution) = edit_dataset_model.dct_temporal_resolution {
            old_active_model.dct_temporal_resolution = ActiveValue::Set(Some(dct_temporal_resolution));
        }
        if let Some(prov_generated_by) = edit_dataset_model.prov_generated_by {
            old_active_model.prov_generated_by = ActiveValue::Set(Some(prov_generated_by));
        }
        if let Some(dct_access_rights) = edit_dataset_model.dct_access_rights {
            old_active_model.dct_access_rights = ActiveValue::Set(Some(dct_access_rights));
        }
        if let Some(dct_license) = edit_dataset_model.dct_license {
            old_active_model.dct_license = ActiveValue::Set(Some(dct_license));
        }
        if let Some(ordl_has_policy) = edit_dataset_model.ordl_has_policy {
            old_active_model.ordl_has_policy = ActiveValue::Set(ordl_has_policy);
        }
        if let Some(dcat_landing_page) = edit_dataset_model.dcat_landing_page {
            old_active_model.dcat_landing_page = ActiveValue::Set(Some(dcat_landing_page));
        }
        if let Some(dcat_contact_point) = edit_dataset_model.dcat_contact_point {
            old_active_model.dcat_contact_point = ActiveValue::Set(Some(dcat_contact_point));
        }
        if let Some(dct_language) = edit_dataset_model.dct_language {
            old_active_model.dct_language = ActiveValue::Set(Some(dct_language));
        }
        if let Some(dct_rights) = edit_dataset_model.dct_rights {
            old_active_model.dct_rights = ActiveValue::Set(Some(dct_rights));
        }
        if let Some(dct_replaces) = edit_dataset_model.dct_replaces {
            old_active_model.dct_replaces = ActiveValue::Set(Some(dct_replaces));
        }
        if let Some(dcat_has_current_version) = edit_dataset_model.dcat_has_current_version {
            old_active_model.dcat_has_current_version = ActiveValue::Set(Some(dcat_has_current_version));
        }
        if let Some(dcat_version) = edit_dataset_model.dcat_version {
            old_active_model.dcat_version = ActiveValue::Set(dcat_version);
        }
        if let Some(dcat_previous_version) = edit_dataset_model.dcat_previous_version {
            old_active_model.dcat_previous_version = ActiveValue::Set(Some(dcat_previous_version));
        }
        if let Some(adms_version_notes) = edit_dataset_model.adms_version_notes {
            old_active_model.adms_version_notes = ActiveValue::Set(Some(adms_version_notes));
        }
        if let Some(dcat_first) = edit_dataset_model.dcat_first {
            old_active_model.dcat_first = ActiveValue::Set(Some(dcat_first));
        }
        if let Some(dcat_last) = edit_dataset_model.dcat_last {
            old_active_model.dcat_last = ActiveValue::Set(Some(dcat_last));
        }
        if let Some(dcat_prev) = edit_dataset_model.dcat_prev {
            old_active_model.dcat_prev = ActiveValue::Set(Some(dcat_prev));
        }
        if let Some(adms_status) = edit_dataset_model.adms_status {
            old_active_model.adms_status = ActiveValue::Set(Some(adms_status));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(&self.db_connection).await;
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
        let catalog = self.get_catalog_by_id(catalog_id.clone()).await?
            .ok_or(CatalogRepoErrors::CatalogNotFound)?;
        let urn = new_dataset_model.id.unwrap_or_else(|| get_urn(None));
        let model = dataset::ActiveModel {
            id:ActiveValue::Set(urn.to_string()),
            dct_conforms_to:ActiveValue::Set(new_dataset_model.dct_conforms_to),
            dct_creator:ActiveValue::Set(new_dataset_model.dct_creator),
            dct_identifier:ActiveValue::Set(Option::from(urn.to_string())),
            dct_issued:ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified:ActiveValue::Set(None),
            dct_title:ActiveValue::Set(new_dataset_model.dct_title),
            dct_description:ActiveValue::Set(new_dataset_model.dct_description),
            catalog_id:ActiveValue::Set(catalog_id.to_string()),
            dcat_inseries:ActiveValue::Set(new_dataset_model.dcat_inseries),
            dct_spatial:ActiveValue::Set(new_dataset_model.dct_spatial),
            dcat_spatial_resolution_meters:ActiveValue::Set(new_dataset_model.dcat_spatial_resolution_meters),
            dct_temporal:ActiveValue::Set(new_dataset_model.dct_temporal),
            dct_temporal_resolution:ActiveValue::Set(new_dataset_model.dct_temporal_resolution),
            prov_generated_by:ActiveValue::Set(new_dataset_model.prov_generated_by),
            dct_access_rights:ActiveValue::Set(new_dataset_model.dct_access_rights),
            dct_license:ActiveValue::Set(new_dataset_model.dct_license),
            ordl_has_policy:ActiveValue::Set(new_dataset_model.ordl_has_policy),
            dcat_landing_page:ActiveValue::Set(new_dataset_model.dcat_landing_page),
            dcat_contact_point:ActiveValue::Set(new_dataset_model.dcat_contact_point),
            dct_language:ActiveValue::Set(new_dataset_model.dct_language),
            dct_rights:ActiveValue::Set(new_dataset_model.dct_rights),
            dct_replaces:ActiveValue::Set(new_dataset_model.dct_replaces),
            dcat_has_current_version:ActiveValue::Set(new_dataset_model.dcat_has_current_version),
            dcat_version:ActiveValue::Set(new_dataset_model.dcat_version),
            dcat_previous_version:ActiveValue::Set(new_dataset_model.dcat_previous_version),
            adms_version_notes:ActiveValue::Set(new_dataset_model.adms_version_notes),
            dcat_first:ActiveValue::Set(new_dataset_model.dcat_first),
            dcat_last:ActiveValue::Set(new_dataset_model.dcat_last),
            dcat_prev:ActiveValue::Set(new_dataset_model.dcat_prev),
            adms_status:ActiveValue::Set(new_dataset_model.adms_status)
        };
        let dataset = dataset::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDataset(err.into())),
        }
    }

    async fn delete_dataset_by_id(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::delete_by_id(dataset_id).exec(&self.db_connection).await;
        match dataset {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::DatasetNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingDataset(err.into())),
        }
    }

    async fn get_datastes_by_dataset_series_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors> {
        let dataset_series_id = dataset_series_id.to_string();
        let dataset_series = dataset_series::Entity::find_by_id(dataset_series_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDatasetSeries(e.into()))?;
        if dataset_series.is_none() {
            return Err(CatalogRepoErrors::DatasetSeriesNotfound);
        }

        let datasets = dataset::Entity::find().filter(dataset::Column::DcatInseries.eq(dataset_series_id)).all(&self.db_connection).await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataset(err.into())),
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
        let distributions = distribution::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
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
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(&self.db_connection).await;
        match dataset {
            Ok(dataset) => match dataset {
                Some(dataset) => {
                    let distributions = distribution::Entity::find()
                        .filter(distribution::Column::DatasetId.eq(dataset.id))
                        .all(&self.db_connection)
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

    async fn get_distribution_by_dataset_id_and_dct_format(&self, dataset_id: Urn, dct_formats: DctFormats) -> anyhow::Result<distribution::Model, CatalogRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(&self.db_connection).await
            .map_err(|err| CatalogRepoErrors::ErrorFetchingDataset(err.into()))?
            .ok_or(CatalogRepoErrors::DatasetNotFound)?;
        let distribution = distribution::Entity::find()
            .filter(
                distribution::Column::DatasetId.eq(dataset_id.clone())
            )
            .filter(
                distribution::Column::DctFormat.eq(dct_formats.to_string())
            )
            .one(&self.db_connection).await
            .map_err(|err| CatalogRepoErrors::ErrorFetchingDistribution(err.into()))?
            .ok_or(CatalogRepoErrors::DistributionNotFound)?;
        Ok(distribution)
    }

    async fn get_distribution_by_id(
        &self,
        distribution_id: Urn,
    ) -> anyhow::Result<Option<distribution::Model>, CatalogRepoErrors> {
        let distribution_id = distribution_id.to_string();
        let distribution = distribution::Entity::find_by_id(distribution_id).one(&self.db_connection).await;
        match distribution {
            Ok(distribution) => Ok(distribution),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDistribution(err.into())),
        }
    }
    async fn get_distributions_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors> {
        if themes.is_empty() {
            return Ok(vec![]);
        }
        let themes = theme::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(theme::Column::Theme.is_in(themes))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingThemes(e.into()))?;
        let distributions_ids: Vec<String> = themes // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if distributions_ids.is_empty() {
            return Ok(vec![]);
        }
        let distributions = distribution::Entity::find()
            .filter(distribution::Column::Id.is_in(distributions_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDistribution(e.into()))?;
        Ok(distributions)
    }

    async fn get_distributions_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors> {
        if keywords.is_empty() {
            return Ok(vec![]);
        }
        let keywords = keyword::Entity::find() // lista de entidades "themes" (id+tema+resourceId)
            .filter(keyword::Column::Keyword.is_in(keywords))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingKeywords(e.into()))?;
        let distributions_ids: Vec<String> = keywords // creamos un vector con los resourceIds de los themes
            .into_iter()
            .map(|tc| tc.dcat_resource)
            .collect();
        if distributions_ids.is_empty() {
            return Ok(vec![]);
        }
        let distributions = distribution::Entity::find()
            .filter(distribution::Column::Id.is_in(distributions_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDistribution(e.into()))?;
        Ok(distributions)
    }
    async fn put_distribution_by_id(
        &self,
        distribution_id: Urn,
        edit_distribution_model: EditDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors> {
        let old_model = self.get_distribution_by_id(distribution_id.clone()).await?
            .ok_or(CatalogRepoErrors::DistributionNotFound)?;
        let mut old_active_model: distribution::ActiveModel = old_model.into();
        if let Some(dct_title) = edit_distribution_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_distribution_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        if let Some(dcat_access_service) = edit_distribution_model.dcat_access_service {
            old_active_model.dcat_access_service = ActiveValue::Set(dcat_access_service.to_string());
        }
        if let Some(dct_issued) = edit_distribution_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(dataset_id) = edit_distribution_model.dataset_id {
            old_active_model.dataset_id = ActiveValue::Set(dataset_id);
        }
        if let Some(dct_format) = edit_distribution_model.dct_format {
            old_active_model.dct_format = ActiveValue::Set(Some(dct_format.to_string()));
        }
        if let Some(dcat_inseries) = edit_distribution_model.dcat_inseries {
            old_active_model.dcat_inseries = ActiveValue::Set(dcat_inseries);
        }
        if let Some(dcat_access_url) = edit_distribution_model.dcat_access_url {
            old_active_model.dcat_access_url = ActiveValue::Set(Some(dcat_access_url));
        }
        if let Some(dcat_download_url) = edit_distribution_model.dcat_download_url {
            old_active_model.dcat_download_url = ActiveValue::Set(Some(dcat_download_url));
        }
        if let Some(dct_access_rights) = edit_distribution_model.dct_access_rights {
            old_active_model.dct_access_rights = ActiveValue::Set(Some(dct_access_rights));
        }
        if let Some(ordl_has_policy) = edit_distribution_model.ordl_has_policy {
            old_active_model.ordl_has_policy = ActiveValue::Set(ordl_has_policy);
        }
        if let Some(dct_conforms_to) = edit_distribution_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_media_type) = edit_distribution_model.dct_media_type {
            old_active_model.dct_media_type = ActiveValue::Set(Some(dct_media_type));
        }
        if let Some(dcat_compress_format) = edit_distribution_model.dcat_compress_format {
            old_active_model.dcat_compress_format = ActiveValue::Set(Some(dcat_compress_format));
        }
        if let Some(dcat_package_format) = edit_distribution_model.dcat_package_format {
            old_active_model.dcat_package_format = ActiveValue::Set(Some(dcat_package_format));
        }
        if let Some(dct_licence) = edit_distribution_model.dct_licence {
            old_active_model.dct_licence = ActiveValue::Set(Some(dct_licence));
        }
        if let Some(dct_rights) = edit_distribution_model.dct_rights {
            old_active_model.dct_rights = ActiveValue::Set(dct_rights);
        }
        if let Some(dct_spatial) = edit_distribution_model.dct_spatial {
            old_active_model.dct_spatial = ActiveValue::Set(Some(dct_spatial));
        }
        if let Some(dct_temporal) = edit_distribution_model.dct_temporal {
            old_active_model.dct_temporal = ActiveValue::Set(Some(dct_temporal));
        }
        if let Some(dcat_spatial_resolution_meters) = edit_distribution_model.dcat_spatial_resolution_meters {
            old_active_model.dcat_spatial_resolution_meters = ActiveValue::Set(Some(dcat_spatial_resolution_meters));
        }
        if let Some(dct_temporal_resolution) = edit_distribution_model.dct_temporal_resolution {
            old_active_model.dct_temporal_resolution = ActiveValue::Set(Some(dct_temporal_resolution));
        }
        if let Some(dcat_byte_size) = edit_distribution_model.dcat_byte_size {
            old_active_model.dcat_byte_size = ActiveValue::Set(Some(dcat_byte_size));
        }
        if let Some(spdc_checksum) = edit_distribution_model.spdc_checksum {
            old_active_model.spdc_checksum = ActiveValue::Set(spdc_checksum);
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(&self.db_connection).await;
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
        let catalog_id = catalog_id.to_string();
        let dataset_id = dataset_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }
        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        if dataset.is_none() {
            return Err(CatalogRepoErrors::DatasetNotFound);
        }
        let data_service = dataservice::Entity::find_by_id(new_distribution_model.dcat_access_service.to_string())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataService(e.into()))?;
        if data_service.is_none() {
            return Err(CatalogRepoErrors::DataServiceNotFound);
        }
        let urn = new_distribution_model.id.unwrap_or_else(|| get_urn(None));
        let model = distribution::ActiveModel {
            id:ActiveValue::Set(urn.to_string()),
            dct_issued:ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified:ActiveValue::Set(None),
            dct_title:ActiveValue::Set(new_distribution_model.dct_title),
            dct_description:ActiveValue::Set(new_distribution_model.dct_description),
            dcat_access_service:ActiveValue::Set(new_distribution_model.dcat_access_service.to_string()),
            dataset_id:ActiveValue::Set(dataset_id),
            dct_format:ActiveValue::Set(new_distribution_model.dct_format.map(|f|f.to_string())),
            dcat_inseries:ActiveValue::Set(new_distribution_model.dcat_inseries),
            dcat_access_url:ActiveValue::Set(new_distribution_model.dcat_access_url),
            dcat_download_url:ActiveValue::Set(new_distribution_model.dcat_download_url),
            dct_access_rights:ActiveValue::Set(new_distribution_model.dct_access_rights),
            ordl_has_policy:ActiveValue::Set(new_distribution_model.ordl_has_policy),
            dct_conforms_to:ActiveValue::Set(new_distribution_model.dct_conforms_to),
            dct_media_type:ActiveValue::Set(new_distribution_model.dct_media_type),
            dcat_compress_format:ActiveValue::Set(new_distribution_model.dcat_compress_format),
            dcat_package_format:ActiveValue::Set(new_distribution_model.dcat_package_format),
            dct_licence:ActiveValue::Set(new_distribution_model.dct_licence),
            dct_rights:ActiveValue::Set(new_distribution_model.dct_rights),
            dct_spatial:ActiveValue::Set(new_distribution_model.dct_spatial),
            dct_temporal:ActiveValue::Set(new_distribution_model.dct_temporal),
            dcat_spatial_resolution_meters:ActiveValue::Set(new_distribution_model.dcat_spatial_resolution_meters),
            dct_temporal_resolution:ActiveValue::Set(new_distribution_model.dct_temporal_resolution),
            dcat_byte_size:ActiveValue::Set(new_distribution_model.dcat_byte_size),
            spdc_checksum:ActiveValue::Set(new_distribution_model.spdc_checksum)
        };
        let distribution = distribution::Entity::insert(model).exec_with_returning(&self.db_connection).await;
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
        let catalog_id = catalog_id.to_string();
        let dataset_id = dataset_id.to_string();
        let distribution_id = distribution_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }

        let dataset = dataset::Entity::find_by_id(dataset_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingDataset(e.into()))?;
        if dataset.is_none() {
            return Err(CatalogRepoErrors::DatasetNotFound);
        }

        let distribution = distribution::Entity::delete_by_id(distribution_id).exec(&self.db_connection).await;
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
        let data_services = dataservice::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match data_services {
            Ok(data_services) => Ok(data_services),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
             return Err(CatalogRepoErrors::CatalogNotFound);
        }
        let data_services = dataservice::Entity::find()
            .filter(dataservice::Column::CatalogId.eq(catalog_id))
            .all(&self.db_connection)
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
        let data_service_id = data_service_id.to_string();
        let data_service = dataservice::Entity::find_by_id(data_service_id).one(&self.db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }
    async fn put_data_service_by_id(
        &self,
        data_service_id: Urn,
        edit_data_service_model: EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors> {
        let old_model = self.get_data_service_by_id(data_service_id.clone()).await?
            .ok_or(CatalogRepoErrors::DataServiceNotFound)?;
        let mut old_active_model: dataservice::ActiveModel = old_model.into();

        if let Some(dcat_endpoint_description) = edit_data_service_model.dcat_endpoint_description {
            old_active_model.dcat_endpoint_description = ActiveValue::Set(Some(dcat_endpoint_description));
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
        if let Some(dct_identifier) = edit_data_service_model.dct_identifier {
            old_active_model.dct_identifier = ActiveValue::Set(Some(dct_identifier));
        }
        if let Some(dct_issued) = edit_data_service_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(catalog_id) = edit_data_service_model.catalog_id {
            old_active_model.catalog_id = ActiveValue::Set(catalog_id);
        }
        if let Some(dcat_serves_dataset) = edit_data_service_model.dcat_serves_dataset {
            old_active_model.dcat_serves_dataset = ActiveValue::Set(dcat_serves_dataset);
        }
        if let Some(dcat_access_rights) = edit_data_service_model.dcat_access_rights {
            old_active_model.dcat_access_rights = ActiveValue::Set(Some(dcat_access_rights));
        }
        if let Some(ordl_has_policy) = edit_data_service_model.ordl_has_policy {
            old_active_model.ordl_has_policy = ActiveValue::Set(ordl_has_policy);
        }
        if let Some(dcat_contact_point) = edit_data_service_model.dcat_contact_point {
            old_active_model.dcat_contact_point = ActiveValue::Set(Some(dcat_contact_point));
        }
        if let Some(dcat_landing_page) = edit_data_service_model.dcat_landing_page {
            old_active_model.dcat_landing_page = ActiveValue::Set(Some(dcat_landing_page));
        }
        if let Some(dct_licence) = edit_data_service_model.dct_licence {
            old_active_model.dct_licence = ActiveValue::Set(Some(dct_licence));
        }
        if let Some(dct_rights) = edit_data_service_model.dct_rights {
            old_active_model.dct_rights = ActiveValue::Set(Some(dct_rights));
        }
        if let Some(dct_publisher) = edit_data_service_model.dct_publisher {
            old_active_model.dct_publisher = ActiveValue::Set(Some(dct_publisher));
        }
        if let Some(prov_qualifed_attribution) = edit_data_service_model.prov_qualifed_attribution {
            old_active_model.prov_qualifed_attribution = ActiveValue::Set(Some(prov_qualifed_attribution));
        }
        if let Some(dcat_has_current_version) = edit_data_service_model.dcat_has_current_version {
            old_active_model.dcat_has_current_version = ActiveValue::Set(Some(dcat_has_current_version));
        }
        if let Some(dcat_version) = edit_data_service_model.dcat_version {
            old_active_model.dcat_version = ActiveValue::Set(dcat_version);
        }
        if let Some(dcat_previous_version) = edit_data_service_model.dcat_previous_version {
            old_active_model.dcat_previous_version = ActiveValue::Set(Some(dcat_previous_version));
        }
        if let Some(adms_version_notes) = edit_data_service_model.adms_version_notes {
            old_active_model.adms_version_notes = ActiveValue::Set(Some(adms_version_notes));
        }
        if let Some(dcat_first) = edit_data_service_model.dcat_first {
            old_active_model.dcat_first = ActiveValue::Set(Some(dcat_first));
        }
        if let Some(dcat_last) = edit_data_service_model.dcat_last {
            old_active_model.dcat_last = ActiveValue::Set(Some(dcat_last));
        }
        if let Some(dcat_prev) = edit_data_service_model.dcat_prev {
            old_active_model.dcat_prev = ActiveValue::Set(Some(dcat_prev));
        }
        if let Some(dct_replaces) = edit_data_service_model.dct_replaces {
            old_active_model.dct_replaces = ActiveValue::Set(Some(dct_replaces));
        }
        if let Some(adms_status) = edit_data_service_model.adms_status {
            old_active_model.adms_status = ActiveValue::Set(Some(adms_status));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(&self.db_connection).await;
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
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }
        let urn = new_data_service_model.id.unwrap_or_else(|| get_urn(None));
        let model = dataservice::ActiveModel {
            id:ActiveValue::Set(urn.to_string()),
            dcat_endpoint_description:ActiveValue::Set(new_data_service_model.dcat_endpoint_description),
            dcat_endpoint_url:ActiveValue::Set(new_data_service_model.dcat_endpoint_url),
            dct_conforms_to:ActiveValue::Set(new_data_service_model.dct_conforms_to),
            dct_creator:ActiveValue::Set(new_data_service_model.dct_creator),
            dct_identifier:ActiveValue::Set(Option::from(urn.to_string())),
            dct_issued:ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified:ActiveValue::Set(None),
            dct_title:ActiveValue::Set(new_data_service_model.dct_title),
            dct_description:ActiveValue::Set(new_data_service_model.dct_description),
            catalog_id:ActiveValue::Set(catalog_id),
            dcat_serves_dataset:ActiveValue::Set(new_data_service_model.dcat_serves_dataset),
            dcat_access_rights:ActiveValue::Set(new_data_service_model.dcat_access_rights),
            ordl_has_policy:ActiveValue::Set(new_data_service_model.ordl_has_policy),
            dcat_contact_point:ActiveValue::Set(new_data_service_model.dcat_contact_point),
            dcat_landing_page:ActiveValue::Set(new_data_service_model.dcat_landing_page),
            dct_licence:ActiveValue::Set(new_data_service_model.dct_licence),
            dct_rights:ActiveValue::Set(new_data_service_model.dct_rights),
            dct_publisher:ActiveValue::Set(new_data_service_model.dct_publisher),
            prov_qualifed_attribution:ActiveValue::Set(new_data_service_model.prov_qualifed_attribution),
            dcat_has_current_version:ActiveValue::Set(new_data_service_model.dcat_has_current_version),
            dcat_version:ActiveValue::Set(new_data_service_model.dcat_version),
            dcat_previous_version:ActiveValue::Set(new_data_service_model.dcat_previous_version),
            adms_version_notes:ActiveValue::Set(new_data_service_model.adms_version_notes),
            dcat_first:ActiveValue::Set(new_data_service_model.dcat_first),
            dcat_last:ActiveValue::Set(new_data_service_model.dcat_last),
            dcat_prev:ActiveValue::Set(new_data_service_model.dcat_prev),
            dct_replaces:ActiveValue::Set(new_data_service_model.dct_replaces),
            adms_status:ActiveValue::Set(new_data_service_model.adms_status)
        };
        let data_service = dataservice::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDataService(err.into())),
        }
    }

    async fn delete_data_service_by_id(
        &self,
        data_service_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let data_service_id = data_service_id.to_string();
        let data_service = dataservice::Entity::delete_by_id(data_service_id).exec(&self.db_connection).await;
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
        let odrl_offers = odrl_offer::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
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
        let entity = entity.to_string();
        let odrl_offers =
            odrl_offer::Entity::find().filter(odrl_offer::Column::Entity.eq(entity)).all(&self.db_connection).await;
        match odrl_offers {
            Ok(odrl_offers) => Ok(odrl_offers),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingOdrlOffer(err.into())),
        }
    }

    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, CatalogRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::find_by_id(odrl_offer_id).one(&self.db_connection).await;
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
        // TODO dynamic typing
        let urn = new_odrl_offer_model.id.unwrap_or_else(|| get_urn(None));
        let model = odrl_offer::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            odrl_offer: ActiveValue::Set(new_odrl_offer_model.odrl_offers),
            entity: ActiveValue::Set(entity_id.to_string()),
            entity_type: ActiveValue::Set(entity_type),
        };
        let odrl_offer = odrl_offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match odrl_offer {
            Ok(odrl_offer) => Ok(odrl_offer),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let odrl_offer_id = odrl_offer_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_by_id(odrl_offer_id).exec(&self.db_connection).await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingOdrlOffer(err.into())),
        }
    }

    async fn delete_odrl_offers_by_entity(&self, entity_id: Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let entity_id = entity_id.to_string();
        let odrl_offer = odrl_offer::Entity::delete_many()
            .filter(odrl_offer::Column::Entity.eq(entity_id))
            .exec(&self.db_connection)
            .await;
        match odrl_offer {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::OdrlOfferNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingOdrlOffer(err.into())),
        }
    }

    async fn get_upstream_offers(&self, entity_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors> {
        todo!("get_upstream_offers")
    }
}

#[async_trait]
impl DatasetSeriesRepo for CatalogRepoForSql {
    async fn get_all_dataset_series(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset_series::Model>, CatalogRepoErrors> {
        let dataset_series = dataset_series::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match dataset_series {
            Ok(dataset_series) => Ok(dataset_series),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDatasetSeries(err.into())),
        }
    }
    async fn get_dataset_series_by_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<Option<dataset_series::Model>, CatalogRepoErrors> {
        let dataset_series_id = dataset_series_id.to_string();
        let dataset_series = dataset_series::Entity::find_by_id(dataset_series_id).one(&self.db_connection).await;
        match dataset_series {
            Ok(dataset_series) => Ok(dataset_series),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingDatasetSeries(err.into())),
        }
    }
    async fn create_dataset_series(
        &self,
        new_dataset_series_model: NewDatasetSeriesModel,
    ) -> anyhow::Result<dataset_series::Model, CatalogRepoErrors> {
        let urn = new_dataset_series_model.id.unwrap_or_else(|| get_urn(None));
        let model = dataset_series::ActiveModel {
            id:ActiveValue::Set(urn.to_string()),
            dct_conforms_to:ActiveValue::Set(new_dataset_series_model.dct_conforms_to),
            dct_creator:ActiveValue::Set(new_dataset_series_model.dct_creator),
            dct_identifier:ActiveValue::Set(urn.to_string()),
            dct_issued:ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified:ActiveValue::Set(None),
            dct_title:ActiveValue::Set(new_dataset_series_model.dct_title),
            dct_description:ActiveValue::Set(new_dataset_series_model.dct_description),
            dct_spatial:ActiveValue::Set(new_dataset_series_model.dct_spatial),
            dcat_spatial_resolution_meters:ActiveValue::Set(new_dataset_series_model.dcat_spatial_resolution_meters),
            dct_temporal:ActiveValue::Set(new_dataset_series_model.dct_temporal),
            dct_temporal_resolution:ActiveValue::Set(new_dataset_series_model.dct_temporal_resolution),
            prov_generated_by:ActiveValue::Set(new_dataset_series_model.prov_generated_by),
            dct_access_rights:ActiveValue::Set(new_dataset_series_model.dct_access_rights),
            ordl_has_policy:ActiveValue::Set(new_dataset_series_model.ordl_has_policy),
            dct_licence:ActiveValue::Set(new_dataset_series_model.dct_licence),
            dcat_inseries:ActiveValue::Set(new_dataset_series_model.dcat_inseries),
            dcat_landing_page:ActiveValue::Set(new_dataset_series_model.dcat_landing_page),
            dcat_contact_point:ActiveValue::Set(new_dataset_series_model.dcat_contact_point),
            dct_language:ActiveValue::Set(new_dataset_series_model.dct_language),
            dct_rights:ActiveValue::Set(new_dataset_series_model.dct_rights),
            dct_publisher:ActiveValue::Set(new_dataset_series_model.dct_publisher),
            dct_type:ActiveValue::Set(new_dataset_series_model.dct_type),
            prov_qualified_attribution:ActiveValue::Set(new_dataset_series_model.prov_qualified_attribution),
            dct_accrual_periodicity:ActiveValue::Set(new_dataset_series_model.dct_accrual_periodicity),
            dcat_has_current_version:ActiveValue::Set(new_dataset_series_model.dcat_has_current_version),
            dcat_version:ActiveValue::Set(new_dataset_series_model.dcat_version),
            dcat_previous_version:ActiveValue::Set(new_dataset_series_model.dcat_previous_version),
            adms_version_notes:ActiveValue::Set(new_dataset_series_model.adms_version_notes),
            dcat_first:ActiveValue::Set(new_dataset_series_model.dcat_first),
            dcat_last:ActiveValue::Set(new_dataset_series_model.dcat_last),
            dcat_prev:ActiveValue::Set(new_dataset_series_model.dcat_prev),
            dct_replaces:ActiveValue::Set(new_dataset_series_model.dct_replaces),
            adms_status:ActiveValue::Set(new_dataset_series_model.adms_status)
        };
        let dataset_series = dataset_series::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match dataset_series {
            Ok(dataset_series) => Ok(dataset_series),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingDatasetSeries(err.into())),
        }
    }

    async fn put_dataset_series_by_id (
        &self,
        dataset_series_id: Urn,
        edit_dataset_series_model: EditDatasetSeriesModel,
    ) -> anyhow::Result<dataset_series::Model, CatalogRepoErrors> {
        let old_model = self.get_dataset_series_by_id(dataset_series_id.clone()).await?
            .ok_or(CatalogRepoErrors::DatasetSeriesNotfound)?;        
        let mut old_active_model: dataset_series::ActiveModel = old_model.into();
        if let Some(dct_conforms_to) = edit_dataset_series_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_dataset_series_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_issued) = edit_dataset_series_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(dct_title) = edit_dataset_series_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_dataset_series_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        if let Some(dct_spatial) = edit_dataset_series_model.dct_spatial {
            old_active_model.dct_spatial = ActiveValue::Set(Some(dct_spatial));
        }
        if let Some(dcat_spatial_resolution_meters) = edit_dataset_series_model.dcat_spatial_resolution_meters {
            old_active_model.dcat_spatial_resolution_meters = ActiveValue::Set(Some(dcat_spatial_resolution_meters));
        }
        if let Some(dct_temporal) = edit_dataset_series_model.dct_temporal {
            old_active_model.dct_temporal = ActiveValue::Set(Some(dct_temporal));
        }
        if let Some(dct_temporal_resolution) = edit_dataset_series_model.dct_temporal_resolution {
            old_active_model.dct_temporal_resolution = ActiveValue::Set(Some(dct_temporal_resolution));
        }
        if let Some(prov_generated_by) = edit_dataset_series_model.prov_generated_by {
            old_active_model.prov_generated_by = ActiveValue::Set(Some(prov_generated_by));
        }
        if let Some(dct_access_rights) = edit_dataset_series_model.dct_access_rights {
            old_active_model.dct_access_rights = ActiveValue::Set(Some(dct_access_rights));
        }
        if let Some(ordl_has_policy) = edit_dataset_series_model.ordl_has_policy {
            old_active_model.ordl_has_policy = ActiveValue::Set(ordl_has_policy);
        }
        if let Some(dct_licence) = edit_dataset_series_model.dct_licence {
            old_active_model.dct_licence = ActiveValue::Set(Some(dct_licence));
        }
        if let Some(dcat_inseries) = edit_dataset_series_model.dcat_inseries {
            old_active_model.dcat_inseries = ActiveValue::Set(Some(dcat_inseries));
        }
        if let Some(dcat_landing_page) = edit_dataset_series_model.dcat_landing_page {
            old_active_model.dcat_landing_page = ActiveValue::Set(Some(dcat_landing_page));
        }
        if let Some(dcat_contact_point) = edit_dataset_series_model.dcat_contact_point {
            old_active_model.dcat_contact_point = ActiveValue::Set(Some(dcat_contact_point));
        }
        if let Some(dct_language) = edit_dataset_series_model.dct_language {
            old_active_model.dct_language = ActiveValue::Set(Some(dct_language));
        }
        if let Some(dct_rights) = edit_dataset_series_model.dct_rights {
            old_active_model.dct_rights = ActiveValue::Set(Some(dct_rights));
        }
        if let Some(dct_publisher) = edit_dataset_series_model.dct_publisher {
            old_active_model.dct_publisher = ActiveValue::Set(dct_publisher);
        }
        if let Some(dct_type) = edit_dataset_series_model.dct_type {
            old_active_model.dct_type = ActiveValue::Set(Some(dct_type));
        }
        if let Some(prov_qualified_attribution) = edit_dataset_series_model.prov_qualified_attribution {
            old_active_model.prov_qualified_attribution = ActiveValue::Set(Some(prov_qualified_attribution));
        }
        if let Some(dct_accrual_periodicity) = edit_dataset_series_model.dct_accrual_periodicity {
            old_active_model.dct_accrual_periodicity = ActiveValue::Set(Some(dct_accrual_periodicity));
        }
        if let Some(dcat_has_current_version) = edit_dataset_series_model.dcat_has_current_version {
            old_active_model.dcat_has_current_version = ActiveValue::Set(Some(dcat_has_current_version));
        }
        if let Some(dcat_version) = edit_dataset_series_model.dcat_version {
            old_active_model.dcat_version = ActiveValue::Set(dcat_version);
        }
        if let Some(dcat_previous_version) = edit_dataset_series_model.dcat_previous_version {
            old_active_model.dcat_previous_version = ActiveValue::Set(Some(dcat_previous_version));
        }
        if let Some(adms_version_notes) = edit_dataset_series_model.adms_version_notes {
            old_active_model.adms_version_notes = ActiveValue::Set(Some(adms_version_notes));
        }
        if let Some(dcat_first) = edit_dataset_series_model.dcat_first {
            old_active_model.dcat_first = ActiveValue::Set(Some(dcat_first));
        }
        if let Some(dcat_last) = edit_dataset_series_model.dcat_last {
            old_active_model.dcat_last = ActiveValue::Set(Some(dcat_last));
        }
        if let Some(dcat_prev) = edit_dataset_series_model.dcat_prev {
            old_active_model.dcat_prev = ActiveValue::Set(Some(dcat_prev));
        }
        if let Some(dct_replaces) = edit_dataset_series_model.dct_replaces {
            old_active_model.dct_replaces = ActiveValue::Set(Some(dct_replaces));
        }
        if let Some(adms_status) = edit_dataset_series_model.adms_status {
            old_active_model.adms_status = ActiveValue::Set(Some(adms_status));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingCatalogRecord(err.into())),
        }
    }

    async fn delete_dataset_series_by_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let dataset_series_id = dataset_series_id.to_string();
        
        let dataset_series = dataset_series::Entity::delete_by_id(dataset_series_id).exec(&self.db_connection).await;
        match dataset_series {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::DatasetSeriesNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingDatasetSeries(err.into())),
        }
    }
}

#[async_trait]
impl CatalogRecordRepo for CatalogRepoForSql {
    async fn get_all_catalog_records(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog_record::Model>, CatalogRepoErrors> {
        let catalog_records = catalog_record::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match catalog_records {
            Ok(catalog_records) => Ok(catalog_records),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalogRecords(err.into())),
        }
    }
    async fn get_catalog_record_by_id (
        &self,
        catalog_record_id: Urn,
    ) -> anyhow::Result<Option<catalog_record::Model>, CatalogRepoErrors> {
        let catalog_record = catalog_record::Entity::find_by_id(catalog_record_id.to_string()).one(&self.db_connection).await;
        match catalog_record {
            Ok(catalog_record) => Ok(catalog_record),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalogRecords(err.into())),
        }
    }
    async fn get_all_catalog_records_by_catalog_id(
        &self,
        catalog_id: Urn,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog_record::Model>, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingCatalog(e.into()))?;
        if catalog.is_none() {
            return Err(CatalogRepoErrors::CatalogNotFound);
        }
        let catalog_records = catalog_record::Entity::find().filter(catalog_record::Column::DcatCatalog.eq(catalog_id)).all(&self.db_connection).await;
        match catalog_records {
            Ok(catalog_records) => Ok(catalog_records),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalogRecords(err.into())),
        }
    }
    async fn create_catalog_record(
        &self,
        new_catalog_record_model: NewCatalogRecordModel,
    ) -> anyhow::Result<catalog_record::Model, CatalogRepoErrors> {
        let model = catalog_record::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            dcat_catalog:ActiveValue::Set(new_catalog_record_model.dcat_catalog),
            dct_title:ActiveValue::Set(new_catalog_record_model.dct_title),
            dct_description:ActiveValue::Set(new_catalog_record_model.dct_description),
            dct_issued:ActiveValue::Set(new_catalog_record_model.dct_issued),
            foaf_primary_topic:ActiveValue::Set(new_catalog_record_model.foaf_primary_topic),
        };
        let catalog_record = catalog_record::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match catalog_record {
            Ok(catalog_record) => Ok(catalog_record),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalogRecord(err.into())),
        }
    }
    async fn put_catalog_record_by_id(
        &self,
        catalog_record_id: Urn,
        edit_catalog_record_model: EditCatalogRecordModel,
    ) -> anyhow::Result<catalog_record::Model, CatalogRepoErrors> {
        let old_model = self.get_catalog_record_by_id(catalog_record_id.clone()).await?
            .ok_or(CatalogRepoErrors::CatalogRecordNotfound)?;
        let mut old_active_model: catalog_record::ActiveModel = old_model.into();

        if let Some(dcat_catalog) = edit_catalog_record_model.dcat_catalog {
            old_active_model.dcat_catalog = ActiveValue::Set(dcat_catalog);
        }
        if let Some(dct_title) = edit_catalog_record_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(dct_title);
        }
        if let Some(dct_description) = edit_catalog_record_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(dct_description);
        }
        if let Some(dct_issued) = edit_catalog_record_model.dct_issued {
            old_active_model.dct_issued = ActiveValue::Set(dct_issued);
        }
        if let Some(foaf_primary_topic) = edit_catalog_record_model.foaf_primary_topic {
            old_active_model.foaf_primary_topic = ActiveValue::Set(foaf_primary_topic);
        }
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingCatalogRecord(err.into()))
        }
    }
    async fn delete_catalog_record_by_id(
        &self,
        catalog_record_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let catalog_record_id = catalog_record_id.to_string();
        let catalog_record = catalog_record::Entity::delete_by_id(catalog_record_id).exec(&self.db_connection).await;
    match catalog_record {
        Ok(delete_result) => match delete_result.rows_affected {
            0 => Err(CatalogRepoErrors::CatalogRecordNotfound),
            _ => Ok(()),
        },
        Err(err) => Err(CatalogRepoErrors::ErrorDeletingCatalogRecord(err.into())),
        }
    }
}

#[async_trait]
impl RelationRepo for CatalogRepoForSql{

    async fn get_all_relations(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<relation::Model>, CatalogRepoErrors> {
        let relations = relation::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match relations {
            Ok(relations) => Ok(relations),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingRelation(err.into())),
        }
    }

    async fn get_relations_by_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_id : Urn,
    ) -> anyhow::Result<Vec<relation::Model>, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let resource = resource::Entity::find_by_id(resource_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
        if resource.is_none() {
            return Err(CatalogRepoErrors::ResourceNotfound);
        }
        let relations_by_resource = relation::Entity::find().filter(
            Condition::any()
                .add(relation::Column::DcatResource1.eq(resource_id.clone()))
                .add(relation::Column::DcatResource2.eq(resource_id))
            )
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match relations_by_resource {
            Ok(relations_by_resource) => Ok(relations_by_resource),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingRelation(err.into())),
        }
    }
    async fn get_resources_by_relation(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation_type: String,
    ) -> anyhow::Result<Vec<(resource::Model, resource::Model)>, CatalogRepoErrors> {
        let relations = relation::Entity::find()
            .filter(relation::Column::DcatRelationship.eq(relation_type))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingRelation(e.into()))?;
        
        let mut result = vec![];
        for rel in relations {
            let res1 = resource::Entity::find_by_id(rel.dcat_resource1.clone())
                .one(&self.db_connection)
                .await
                .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
            let res2 = resource::Entity::find_by_id(rel.dcat_resource2.clone())
                .one(&self.db_connection)
                .await
                .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
            if let (Some(r1), Some(r2)) = (res1, res2) {
                result.push((r1, r2));
            }
        }
    Ok(result)
    }
    async fn get_related_resource_by_relation_and_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation: String,
        resource_id: Urn,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let condition = Condition::all()
            .add(
                Condition::any()
                    .add(relation::Column::DcatResource1.eq(resource_id.clone()))
                    .add(relation::Column::DcatResource2.eq(resource_id)),
            )
            .add(relation::Column::DcatRelationship.eq(relation));
        let relations = relation::Entity::find()
            .filter(condition)
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingRelation(e.into()))?;
        let mut resources_ids: HashSet<String> = HashSet::new();
        for rel in &relations {
            resources_ids.insert(rel.dcat_resource1.clone());
            resources_ids.insert(rel.dcat_resource2.clone());
        }

        let related_resources =  resource::Entity::find()
            .filter(resource::Column::ResourceId.is_in(resources_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
        
        Ok(related_resources)
    }
    async fn put_relation_by_id(
        &self,
        relation_id: Urn,
        edit_ralation_model: EditRelationModel,
    ) -> anyhow::Result<relation::Model, CatalogRepoErrors> {
        let relation_id = relation_id.to_string();
        let old_model = relation::Entity::find_by_id(relation_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::RelationNotfound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingRelation(err.into())),
        };
        let mut old_active_model: relation::ActiveModel = old_model.into();
        if let Some(dcat_resource1) = edit_ralation_model.dcat_resource1 {
            old_active_model.dcat_resource1 = ActiveValue::Set(dcat_resource1);
        }
        if let Some(dcat_resource2) = edit_ralation_model.dcat_resource2 {
            old_active_model.dcat_resource2 = ActiveValue::Set(dcat_resource2);
        }
        if let Some(dcat_relationship) = edit_ralation_model.dcat_relationship {
            old_active_model.dcat_relationship = ActiveValue::Set(dcat_relationship);
        }

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingRelation(err.into())),
        }
    }
    async fn create_relation(
        &self,
        new_relation_model: NewRelationModel,
    ) -> anyhow::Result<relation::Model, CatalogRepoErrors> {
        let model = relation::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            dcat_relationship: ActiveValue::Set(new_relation_model.dcat_relationship),
            dcat_resource1: ActiveValue::Set(new_relation_model.dcat_resource1),
            dcat_resource2: ActiveValue::Set(new_relation_model.dcat_resource2),
        };
        let relation = relation::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match relation {
            Ok(relation) => Ok(relation),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingRelation(err.into())),
        }
    }
    async fn delete_relation_by_id(
        &self,
        relation_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let relation_id = relation_id.to_string();
        let relation = relation::Entity::delete_by_id(relation_id).exec(&self.db_connection).await;
        match relation {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::RelationNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingRelation(err.into())),
        }
    }
    
}
#[async_trait]
impl QualifiedRelationRepo for CatalogRepoForSql{

    async fn get_all_qualified_relations(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<qualified_relation::Model>, CatalogRepoErrors> {
        let relations = qualified_relation::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match relations {
            Ok(relations) => Ok(relations),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingRelation(err.into())),
        }
    }

    async fn get_qualified_relations_by_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_id : Urn,
    ) -> anyhow::Result<Vec<qualified_relation::Model>, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let resource = resource::Entity::find_by_id(resource_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
        if resource.is_none() {
            return Err(CatalogRepoErrors::ResourceNotfound);
        }
        let relations_by_resource = qualified_relation::Entity::find().filter(
            Condition::any()
                .add(qualified_relation::Column::DcatResource1.eq(resource_id.clone()))
                .add(qualified_relation::Column::DcatResource2.eq(resource_id))
            )
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match relations_by_resource {
            Ok(relations_by_resource) => Ok(relations_by_resource),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingQualifiedRelation(err.into())),
        }
    }
    async fn get_resources_by_qualified_relation(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation_type: String,
    ) -> anyhow::Result<Vec<(resource::Model, resource::Model)>, CatalogRepoErrors> {
        let relations = qualified_relation::Entity::find()
            .filter(qualified_relation::Column::DcatQualifiedRelation.eq(relation_type))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingQualifiedRelation(e.into()))?;
        
        let mut result = vec![];
        for rel in relations {
            let res1 = resource::Entity::find_by_id(rel.dcat_resource1.clone())
                .one(&self.db_connection)
                .await
                .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
            let res2 = resource::Entity::find_by_id(rel.dcat_resource2.clone())
                .one(&self.db_connection)
                .await
                .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
            if let (Some(r1), Some(r2)) = (res1, res2) {
                result.push((r1, r2));
            }
        }
    Ok(result)
    }
    async fn get_related_resource_by_qualified_relation_and_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation: String,
        resource_id: Urn,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let condition = Condition::all()
            .add(
                Condition::any()
                    .add(qualified_relation::Column::DcatResource1.eq(resource_id.clone()))
                    .add(qualified_relation::Column::DcatResource2.eq(resource_id)),
            )
            .add(qualified_relation::Column::DcatQualifiedRelation.eq(relation));
        let relations = qualified_relation::Entity::find()
            .filter(condition)
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingRelation(e.into()))?;
        let mut resources_ids: HashSet<String> = HashSet::new();
        for rel in &relations {
            resources_ids.insert(rel.dcat_resource1.clone());
            resources_ids.insert(rel.dcat_resource2.clone());
        }

        let related_resources =  resource::Entity::find()
            .filter(resource::Column::ResourceId.is_in(resources_ids))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| CatalogRepoErrors::ErrorFetchingResource(e.into()))?;
        
        Ok(related_resources)
    }
    async fn put_qualified_relation_by_id(
        &self,
        relation_id: Urn,
        edit_ralation_model: EditQualifiedRelationModel,
    ) -> anyhow::Result<qualified_relation::Model, CatalogRepoErrors> {
        let relation_id = relation_id.to_string();
        let old_model = qualified_relation::Entity::find_by_id(relation_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::QualifiedRelationNotfound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingQualifiedRelation(err.into())),
        };
        let mut old_active_model: qualified_relation::ActiveModel = old_model.into();
        if let Some(dcat_resource1) = edit_ralation_model.dcat_resource1 {
            old_active_model.dcat_resource1 = ActiveValue::Set(dcat_resource1);
        }
        if let Some(dcat_resource2) = edit_ralation_model.dcat_resource2 {
            old_active_model.dcat_resource2 = ActiveValue::Set(dcat_resource2);
        }
        if let Some(dcat_qualified_relation) = edit_ralation_model.dcat_qualified_relation {
            old_active_model.dcat_qualified_relation = ActiveValue::Set(dcat_qualified_relation);
        }

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingQualifiedRelation(err.into())),
        }
    }
    async fn create_qualified_relation(
        &self,
        new_relation_model: NewQualifiedRelationModel,
    ) -> anyhow::Result<qualified_relation::Model, CatalogRepoErrors> {
        let model = qualified_relation::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            dcat_qualified_relation: ActiveValue::Set(new_relation_model.dcat_qualified_relation),
            dcat_resource1: ActiveValue::Set(new_relation_model.dcat_resource1),
            dcat_resource2: ActiveValue::Set(new_relation_model.dcat_resource2),
        };
        let relation = qualified_relation::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match relation {
            Ok(relation) => Ok(relation),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingRelation(err.into())),
        }
    }
    async fn delete_qualified_relation_by_id(
        &self,
        relation_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let relation_id = relation_id.to_string();
        let relation = qualified_relation::Entity::delete_by_id(relation_id).exec(&self.db_connection).await;
        match relation {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::RelationNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingRelation(err.into())),
        }
    }
    
}

#[async_trait]
impl ResourceRepo for CatalogRepoForSql {
    async fn get_all_resources (
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors> {
        let resources = resource::Entity::find() 
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match resources {
            Ok(resources) => Ok(resources),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingResource(err.into())),
        }
    }
    async fn get_resource_by_id (
        &self,
        resource_id: Urn,
    ) -> anyhow::Result<Option<resource::Model>, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let resource = resource::Entity::find_by_id(resource_id)
            .one(&self.db_connection)
            .await;
        match resource {
            Ok(resource) => Ok(resource),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingResource(err.into())),
        }
    }
    async fn get_all_resources_by_type (
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_type: String
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors> {
        let resources = resource::Entity::find()
            .filter(resource::Column::ResourceType.eq(resource_type))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match resources {
            Ok(resources) => Ok(resources),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingResource(err.into())),
        }
    }
    async fn put_resource_by_id (
        &self,
        resource_id: Urn,
        edit_resource_model: EditResourceModel,
    ) -> anyhow::Result<resource::Model, CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let old_model = resource::Entity::find_by_id(resource_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::ResourceNotfound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingResource(err.into())),
        };
        let mut old_active_model: resource::ActiveModel = old_model.into();
        if let Some(resource_id) = edit_resource_model.resource_id {
            old_active_model.resource_id = ActiveValue::Set(resource_id.to_string());
        }
        if let Some(resource_type) = edit_resource_model.resource_type {
            old_active_model.resource_type = ActiveValue::Set(resource_type);
        }
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingResource(err.into())),
        }
    }
    async fn create_resource (
        &self,
        new_reosurce: NewResourceModel,
    ) -> anyhow::Result<resource::Model, CatalogRepoErrors> {
        let model = resource::ActiveModel {
            resource_id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            resource_type: ActiveValue::Set(new_reosurce.resource_type),
        };
        let resource = resource::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match resource {
            Ok(resource) => Ok(resource),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingRelation(err.into())),
        }
    }
    async fn delete_resource_by_id (
        &self,
        resource_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let resource_id = resource_id.to_string();
        let resource = qualified_relation::Entity::delete_by_id(resource_id).exec(&self.db_connection).await;
        match resource {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::ResourceNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingResource(err.into())),
        }
    }
}

#[async_trait]
impl ReferenceRepo for CatalogRepoForSql{
    async fn get_all_references(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<reference::Model>, CatalogRepoErrors>{
        let references = reference::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
    match references {
            Ok(references) => Ok(references),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingReference(err.into())),
       }
    }
    async fn get_all_references_by_referenced_resource (
        &self,
        referenced_resource_id: Urn,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<reference::Model>, CatalogRepoErrors> {
        let resource = self.get_resource_by_id(referenced_resource_id.clone()).await?;
        let referenced_resource_id = referenced_resource_id.to_string();
        let references = reference::Entity::find()
            .filter(reference::Column::ReferencedResourceId.eq(referenced_resource_id))
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match references {
            Ok(references) => Ok(references),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingReference(err.into())),
        }
    }
    async fn put_reference_by_id (
        self,
        reference_id: Urn,
        edit_reference: EditReferenceModel,
    ) -> anyhow::Result<reference::Model, CatalogRepoErrors>{
        let reference_id = reference_id.to_string();
        let old_model = reference::Entity::find_by_id(reference_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::ReferenceNotfound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingReference(err.into())),
        };
        let mut old_active_model: reference::ActiveModel = old_model.into();
        if let Some(referenced_resource_id) = edit_reference.referenced_resource_id {
            old_active_model.referenced_resource_id = ActiveValue::Set(referenced_resource_id.to_string());
        }
        if let Some(reference) = edit_reference.reference {
            old_active_model.reference = ActiveValue::Set(reference);
        }
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingReference(err.into())),
        }
    }
    async fn create_reference (
        self,
        new_reference: NewReferenceModel,
    ) -> anyhow::Result<reference::Model, CatalogRepoErrors> {
        let model = reference::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            referenced_resource_id: ActiveValue::Set(new_reference.referenced_resource_id.to_string()),
            reference: ActiveValue::Set(new_reference.reference),
        };
        let reference = reference::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match reference {
            Ok(reference) => Ok(reference),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingReference(err.into())),
        }
    }
    async fn delete_reference (
        self,
        reference_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let reference_id = reference_id.to_string();
        let reference = reference::Entity::delete_by_id(reference_id).exec(&self.db_connection).await;
        match reference {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::ReferenceNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingReference(err.into())),
        }
    }
}

#[async_trait]
impl KeywordThemesRepo for CatalogRepoForSql {
    async fn get_all_keywords(
        &self,
    ) -> anyhow::Result<Vec<keyword::Model>, CatalogRepoErrors> {
        let keywords = keyword::Entity::find().all(&self.db_connection).await;
        match keywords {
            Ok(keywords) => Ok(keywords),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingKeywords(err.into())),
       }
    }
    async fn get_all_themes(
        &self,
    ) -> anyhow::Result<Vec<theme::Model>, CatalogRepoErrors> {
        let themes = theme::Entity::find().all(&self.db_connection).await;
        match themes {
            Ok(themes) => Ok(themes),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingThemes(err.into()))
        }
    }
    async fn create_keyword(
        &self,
        new_keyword: NewKeywordModel,
    ) -> anyhow::Result<keyword::Model, CatalogRepoErrors> {
        let model = keyword::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            keyword: ActiveValue::Set(new_keyword.keyword),
            dcat_resource: ActiveValue::Set(new_keyword.dcat_resource.to_string()),
        };
        let keyword = keyword::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match keyword {
            Ok(keyword) => Ok(keyword),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingKeyword(err.into())),
        }
    }
    async fn create_theme(
        &self,
        new_theme: NewThemeModel,
    ) -> anyhow::Result<theme::Model, CatalogRepoErrors> {
        let model = theme::ActiveModel {
            id:ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            theme: ActiveValue::Set(new_theme.theme),
            dcat_resource: ActiveValue::Set(new_theme.dcat_resource.to_string()),
        };
        let theme = theme::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match theme {
            Ok(theme) => Ok(theme),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingTheme(err.into())),
        }
    }
    async fn delete_keyword(
        &self,
        keyword_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let keyword_id = keyword_id.to_string();
        let keyword = keyword::Entity::delete_by_id(keyword_id).exec(&self.db_connection).await;
        match keyword {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::KeywordNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingKeyword(err.into())),
        }
    }
    async fn delete_theme(
        &self,
        theme_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors> {
        let theme_id = theme_id.to_string();
        let theme = theme::Entity::delete_by_id(theme_id).exec(&self.db_connection).await;
        match theme {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::ThemeNotfound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingTheme(err.into())),
        }
    }

}