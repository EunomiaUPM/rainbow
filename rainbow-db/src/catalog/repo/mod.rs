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

use std::any;

use super::entities::catalog;
use super::entities::dataservice;
use super::entities::dataset;
use super::entities::distribution;
use super::entities::odrl_offer;
use super::entities::dataset_series;
use super::entities::resource;
use super::entities::keyword;
use super::entities::theme;
use super::entities::relation;
use super::entities::qualified_relation;
use super::entities::reference;
use super::entities::catalog_record;

use crate::transfer_provider::repo::{TransferMessagesRepo, TransferProcessRepo};
use anyhow::Error;
use axum::async_trait;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::catalog_definition;
use sea_orm::sqlx::sqlite::SqliteValueRef;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use thiserror::Error;
use urn::Urn;

pub mod sql;

pub trait CatalogRepoFactory:
    CatalogRepo + DatasetRepo + 
    DistributionRepo + DataServiceRepo + 
    OdrlOfferRepo + DatasetSeriesRepo + 
    CatalogRecordRepo + RelationRepo + 
    QualifiedRelationRepo + ResourceRepo + 
    ReferenceRepo + KeywordThemesRepo +
    Send + Sync + 'static

{
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewCatalogModel {
    pub id: Option<Urn>,
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_identifier: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dspace_participant_id: Option<String>,
    pub dspace_main_catalog: bool,
    pub dct_description: Option<String>,
    pub dct_access_rights: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub ordl_has_policy: String,
    pub dcat_landing_page: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_publisher: Option<String>,
    pub prov_qualified_attribution: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: String,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>,
}

pub struct EditCatalogModel {
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_identifier: Option<String>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub dspace_participant_id: Option<String>,
    pub dspace_main_catalog: Option<bool>,
    pub dct_description: Option<String>,
    pub dct_access_rights: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_publisher: Option<String>,
    pub prov_qualified_attribution: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: Option<String>,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>,
}

#[async_trait]
pub trait CatalogRepo {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        no_main_catalog: bool,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn get_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors>;
    async fn get_main_catalog(&self) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors>;
    async fn get_catalogs_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn get_catalogs_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn put_catalog_by_id(
        &self,
        catalog_id: Urn,
        edit_catalog_model: EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;
    async fn create_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;

    async fn create_main_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;

    async fn delete_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewDatasetModel {
    pub id: Option<Urn>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dct_identifier: Option<String>,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub catalog_id: Urn,
    pub dcat_inseries: Option<String>,
    pub dct_spatial: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal: Option<String>,
    pub dct_temporal_resolution: Option<String>,
    pub prov_generated_by: Option<String>,
    pub dct_access_rights: Option<String>,
    pub dct_license: Option<String>,
    pub ordl_has_policy: String,
    pub dcat_landing_page: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dct_language: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_replaces: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: String,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub adms_status: Option<String>,
}

pub struct EditDatasetModel {
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_identifier: Option<String>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub catalog_id: Option<String>,
    pub dcat_inseries: Option<String>,
    pub dct_spatial: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal: Option<String>,
    pub dct_temporal_resolution: Option<String>,
    pub prov_generated_by: Option<String>,
    pub dct_access_rights: Option<String>,
    pub dct_license: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dct_language: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_replaces: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: Option<String>,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub adms_status: Option<String>,
}

#[async_trait]
pub trait DatasetRepo {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_id(&self, dataset_id: Urn) -> anyhow::Result<Option<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_from_dataset_series_by_dataset_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn put_datasets_by_id(
        &self,
        dataset_id: Urn,
        edit_dataset_model: EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors>;
    async fn create_dataset(
        &self,
        catalog_id: Urn,
        new_dataset_model: NewDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors>;
    async fn delete_dataset_by_id(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn get_datastes_by_dataset_series_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
}

#[derive(Debug)]
pub struct NewDistributionModel {
    pub id: Option<Urn>,
    pub dct_format: Option<DctFormats>,
    pub dct_title: Option<String>,
    pub dcat_access_service: Urn,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_description: Option<String>,
    pub dataset_id: Urn,
    pub dcat_inseries: String,
    pub dcat_access_url: Option<String>,
    pub dcat_download_url: Option<String>,
    pub dct_access_rights: Option<String>,
    pub ordl_has_policy: String,
    pub dct_conforms_to: Option<String>,
    pub dct_media_type: Option<String>,
    pub dcat_compress_format: Option<String>,
    pub dcat_package_format: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_rights: String,
    pub dct_spatial: Option<String>,
    pub dct_temporal: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal_resolution: Option<String>,
    pub dcat_byte_size: Option<i64>,
    pub spdc_checksum: String
}
pub struct EditDistributionModel {
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dcat_access_service: Option<Urn>,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub dataset_id: Option<String>,
    pub dct_format: Option<DctFormats>,
    pub dcat_inseries: Option<String>,
    pub dcat_access_url: Option<String>,
    pub dcat_download_url: Option<String>,
    pub dct_access_rights: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_media_type: Option<String>,
    pub dcat_compress_format: Option<String>,
    pub dcat_package_format: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_spatial: Option<String>,
    pub dct_temporal: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal_resolution: Option<String>,
    pub dcat_byte_size: Option<i64>,
    pub spdc_checksum: Option<String>
}

#[async_trait]
pub trait DistributionRepo {
    async fn get_all_distributions(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn get_distributions_by_dataset_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn get_distribution_by_dataset_id_and_dct_format(
        &self,
        dataset_id: Urn,
        dct_formats: DctFormats,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors>;
    async fn get_distribution_by_id(
        &self,
        distribution_id: Urn,
    ) -> anyhow::Result<Option<distribution::Model>, CatalogRepoErrors>;
    async fn get_distributions_by_themes(
        &self,
        themes: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn get_distributions_by_keywords(
        &self,
        keywords: Vec<String>,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn put_distribution_by_id(
        &self,
        distribution_id: Urn,
        edit_distribution_model: EditDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors>;
    async fn create_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        new_distribution_model: NewDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors>;
    async fn delete_distribution_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewDataServiceModel {
    pub id: Option<Urn>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dcat_endpoint_description: Option<String>,
    pub dcat_endpoint_url: String,
    pub dct_description: Option<String>,
    pub dct_identifier: Option<String>,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub catalog_id: Urn,
    pub dcat_serves_dataset: String,
    pub dcat_access_rights: Option<String>,
    pub ordl_has_policy: String,
    pub dcat_contact_point: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_publisher: Option<String>,
    pub prov_qualifed_attribution: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: String,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>,
}
pub struct EditDataServiceModel {
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dcat_endpoint_description: Option<String>,
    pub dcat_endpoint_url: Option<String>,
    pub dct_description: Option<String>,
    pub dct_identifier: Option<String>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub catalog_id: Option<String>,
    pub dcat_serves_dataset: Option<String>,
    pub dcat_access_rights: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_publisher: Option<String>,
    pub prov_qualifed_attribution: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: Option<String>,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>
}

#[async_trait]
pub trait DataServiceRepo {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors>;

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors>;

    async fn get_data_service_by_id(
        &self,
        data_service_id: Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, CatalogRepoErrors>;
    async fn put_data_service_by_id(
        &self,
        data_service_id: Urn,
        edit_data_service_model: EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors>;
    async fn create_data_service(
        &self,
        catalog_id: Urn,
        new_data_service_model: NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors>;
    async fn delete_data_service_by_id(
        &self,
        data_service_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewOdrlOfferModel {
    pub id: Option<Urn>,
    pub odrl_offers: Option<serde_json::Value>,
    pub entity: Urn,
    pub entity_type: String,
}

#[async_trait]
pub trait OdrlOfferRepo {
    async fn get_all_odrl_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
    async fn get_all_odrl_offers_by_entity(
        &self,
        entity: Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, CatalogRepoErrors>;
    async fn create_odrl_offer(
        &self,
        entity_id: Urn,
        entity_type: String, // TODO EntityTypes
        new_odrl_offer_model: NewOdrlOfferModel,
    ) -> anyhow::Result<odrl_offer::Model, CatalogRepoErrors>;
    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn delete_odrl_offers_by_entity(&self, entity_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn get_upstream_offers(&self, entity_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
}

pub struct NewDatasetSeriesModel {
    pub id: Option<Urn>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_identifier: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dct_spatial: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal: Option<String>,
    pub dct_temporal_resolution: Option<String>,
    pub prov_generated_by: Option<String>,
    pub dct_access_rights: Option<String>,
    pub ordl_has_policy: String,
    pub dct_licence: Option<String>,
    pub dcat_inseries: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dct_language: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_publisher: String,
    pub dct_type: Option<String>,
    pub prov_qualified_attribution: Option<String>,
    pub dct_accrual_periodicity: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: String,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>,
}
pub struct EditDatasetSeriesModel {
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dct_spatial: Option<String>,
    pub dcat_spatial_resolution_meters: Option<f64>,
    pub dct_temporal: Option<String>,
    pub dct_temporal_resolution: Option<String>,
    pub prov_generated_by: Option<String>,
    pub dct_access_rights: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dct_licence: Option<String>,
    pub dcat_inseries: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dct_language: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_publisher: Option<String>,
    pub dct_type: Option<String>,
    pub prov_qualified_attribution: Option<String>,
    pub dct_accrual_periodicity: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: Option<String>,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dct_replaces: Option<String>,
    pub adms_status: Option<String>,
}

#[async_trait]
pub trait DatasetSeriesRepo {
    async fn get_all_dataset_series(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset_series::Model>, CatalogRepoErrors>;
    async fn get_dataset_series_by_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<Option<dataset_series::Model>, CatalogRepoErrors>;
    async fn create_dataset_series(
        &self,
        new_dataset_series_model: NewDatasetSeriesModel,
    ) -> anyhow::Result<dataset_series::Model, CatalogRepoErrors>;
    async fn put_dataset_series_by_id(
        &self,
        dataset_series_id: Urn,
        update_dataset_series: EditDatasetSeriesModel,
        
    ) -> anyhow::Result<dataset_series::Model, CatalogRepoErrors>;
    async fn delete_dataset_series_by_id(
        &self,
        dataset_series_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}


pub struct NewCatalogRecordModel {
    pub id: Option<Urn>,
    pub dcat_catalog: String,
    pub dct_title: String,
    pub dct_description: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub foaf_primary_topic: String
}

pub struct EditCatalogRecordModel {
    pub dcat_catalog: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dct_issued: Option<chrono::NaiveDateTime>,
    pub foaf_primary_topic: Option<String>
}

#[async_trait]
pub trait CatalogRecordRepo {
    async fn get_all_catalog_records(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog_record::Model>, CatalogRepoErrors>;
    async fn get_catalog_record_by_id (
        &self,
        catalog_record_id: Urn,
    ) -> anyhow::Result<Option<catalog_record::Model>, CatalogRepoErrors>;
    async fn get_all_catalog_records_by_catalog_id(
        &self,
        catalog_id: Urn,
        limit: Option<u64>,
        page: Option<u64>
    ) -> anyhow::Result<Vec<catalog_record::Model>, CatalogRepoErrors>;
    async fn create_catalog_record(
        &self,
        new_catalog_record_model: NewCatalogRecordModel,
    ) -> anyhow::Result<catalog_record::Model, CatalogRepoErrors>;
    async fn put_catalog_record_by_id(
        &self,
        catalog_record_id: Urn,
        edit_catalog_record_model: EditCatalogRecordModel
    ) -> anyhow::Result<catalog_record::Model, CatalogRepoErrors>;
    async fn delete_catalog_record_by_id(
        &self,
        catalog_record_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}


pub struct NewRelationModel {
    pub id: Option<Urn>,
    pub dcat_relationship: String,
    pub dcat_resource1: String,
    pub dcat_resource2: String,
}

pub struct EditRelationModel {
    pub dcat_relationship: Option<String>,
    pub dcat_resource1: Option<String>,
    pub dcat_resource2: Option<String>,
}

#[async_trait]
pub trait RelationRepo {
    async fn get_all_relations(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<relation::Model>, CatalogRepoErrors>;
    async fn get_relations_by_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_id: Urn
    ) -> anyhow::Result<Vec<relation::Model>, CatalogRepoErrors>;
    async fn get_resources_by_relation(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation_type: String,
    ) -> anyhow::Result<Vec<(resource::Model, resource::Model)>, CatalogRepoErrors>;
    async fn get_related_resource_by_relation_and_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation: String,
        resource_id: Urn,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors>;
    async fn put_relation_by_id(
        &self,
        relation_id: Urn,
        relation_model: EditRelationModel,
    ) -> anyhow::Result<relation::Model, CatalogRepoErrors>;
    async fn create_relation(
        &self,
        relation_model: NewRelationModel,
    ) -> anyhow::Result<relation::Model, CatalogRepoErrors>;
    async fn delete_relation_by_id(
        &self,
        relation_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}


pub struct NewQualifiedRelationModel {
    pub id: Option<Urn>,
    pub dcat_qualified_relation: String,
    pub dcat_resource1: String,
    pub dcat_resource2: String,
}

pub struct EditQualifiedRelationModel {
    pub dcat_qualified_relation: Option<String>,
    pub dcat_resource1: Option<String>,
    pub dcat_resource2: Option<String>,
}
#[async_trait]
pub trait QualifiedRelationRepo {
    async fn get_all_qualified_relations(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<qualified_relation::Model>, CatalogRepoErrors>;
    async fn get_qualified_relations_by_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_id: Urn
    ) -> anyhow::Result<Vec<qualified_relation::Model>, CatalogRepoErrors>;
    async fn get_resources_by_qualified_relation(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation_type: String,
    ) -> anyhow::Result<Vec<(resource::Model, resource::Model)>, CatalogRepoErrors>;
    async fn get_related_resource_by_qualified_relation_and_resource(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        relation: String,
        resource_id: Urn,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors>;
    async fn put_qualified_relation_by_id(
        &self,
        relation_id: Urn,
        relation_model: EditQualifiedRelationModel,
    ) -> anyhow::Result<qualified_relation::Model, CatalogRepoErrors>;
    async fn create_qualified_relation(
        &self,
        relation_model: NewQualifiedRelationModel,
    ) -> anyhow::Result<qualified_relation::Model, CatalogRepoErrors>;
    async fn delete_qualified_relation_by_id(
        &self,
        relation_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewResourceModel {
    pub resource_id: Urn,
    pub resource_type: String
}

pub struct EditResourceModel {
    pub resource_id: Option<Urn>,
    pub resource_type: Option<String>
}

#[async_trait]
pub trait ResourceRepo {
    async fn get_all_resources (
        &self,
        limit: Option<u64>,
        page: Option<u64>
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors>;
    async fn get_resource_by_id (
        &self,
        resource_id: Urn,
    ) -> anyhow::Result<Option<resource::Model>, CatalogRepoErrors>;
    async fn get_all_resources_by_type (
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        resource_type: String,
    ) -> anyhow::Result<Vec<resource::Model>, CatalogRepoErrors>;
    async fn put_resource_by_id (
        &self,
        resource_id: Urn,
        edit_resource_model: EditResourceModel
    ) -> anyhow::Result<resource::Model, CatalogRepoErrors>;
    async fn create_resource (
        &self,
        new_resource: NewResourceModel,
    ) -> anyhow::Result<resource::Model, CatalogRepoErrors>;
    async fn delete_resource_by_id (
        &self,
        resource_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}


pub struct NewReferenceModel {
    pub id: Urn,
    pub referenced_resource_id: Urn,
    pub reference: String,
}
pub struct EditReferenceModel {
    pub referenced_resource_id: Option<Urn>,
    pub reference: Option<String>,
}

#[async_trait]
pub trait ReferenceRepo {
    async fn get_all_references(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<reference::Model>, CatalogRepoErrors>;
    async fn get_all_references_by_referenced_resource (
        &self,
        referenced_resource_id: Urn,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<reference::Model>, CatalogRepoErrors>;
    async fn put_reference_by_id (
        self,
        reference_id: Urn,
        edit_reference: EditReferenceModel,
    ) -> anyhow::Result<reference::Model, CatalogRepoErrors>;
    async fn create_reference (
        self,
        new_reference: NewReferenceModel,
    ) -> anyhow::Result<reference::Model, CatalogRepoErrors>;
    async fn delete_reference (
        self,
        reference_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewKeywordModel {
    pub id: Urn,
    pub keyword: String,
    pub dcat_resource: Urn,
}
pub struct NewThemeModel {
    pub id: Urn,
    pub theme: String,
    pub dcat_resource: Urn,
}

#[async_trait]
pub trait KeywordThemesRepo {
    async fn get_all_keywords(
        &self,
    ) -> anyhow::Result<Vec<keyword::Model>, CatalogRepoErrors>;
    async fn get_all_themes(
        &self,
    ) -> anyhow::Result<Vec<theme::Model>, CatalogRepoErrors>;
    async fn create_keyword(
        &self,
        new_keyword: NewKeywordModel,
    ) -> anyhow::Result<keyword::Model, CatalogRepoErrors>;
    async fn create_theme(
        &self,
        new_theme: NewThemeModel,
    ) -> anyhow::Result<theme::Model, CatalogRepoErrors>;
    async fn delete_keyword(
        &self,
        keyword_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn delete_theme(
        &self,
        theme_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}
#[derive(Error, Debug)]
pub enum CatalogRepoErrors {
    #[error("Catalog not found")]
    CatalogNotFound,
    #[error("Dataset not found")]
    DatasetNotFound,
    #[error("Distribution not found")]
    DistributionNotFound,
    #[error("DataService not found")]
    DataServiceNotFound,
    #[error("OdrlOffer not found")]
    OdrlOfferNotFound,
    #[error("Dataset Seroes not found")]
    DatasetSeriesNotfound,
    #[error("Catalog record not found")]
    CatalogRecordNotfound,
    #[error("Theme not found")]
    ThemeNotfound,
    #[error("Keyword not found")]
    KeywordNotfound,
    #[error("Relation not found")]
    RelationNotfound,
    #[error("Qualified Relation not found")]
    QualifiedRelationNotfound,
    #[error("Resource not found")]
    ResourceNotfound,
    #[error("Reference not found")]
    ReferenceNotfound,

    #[error("Error fetching catalog. {0}")]
    ErrorFetchingCatalog(Error),
    #[error("Error fetching dataset. {0}")]
    ErrorFetchingDataset(Error),
    #[error("Error fetching distribution. {0}")]
    ErrorFetchingDistribution(Error),
    #[error("Error fetching data service. {0}")]
    ErrorFetchingDataService(Error),
    #[error("Error fetching odrl offer. {0}")]
    ErrorFetchingOdrlOffer(Error),
    #[error("Error fetching dataset series. {0}")]
    ErrorFetchingDatasetSeries(Error),
    #[error("Error fetching catalog records. {0}")]
    ErrorFetchingCatalogRecords(Error),
    #[error("Error fetching themes. {0}")]
    ErrorFetchingThemes(Error),
    #[error("Error fetching keywords. {0}")]
    ErrorFetchingKeywords(Error),
    #[error("Error fetching relation. {0}")]
    ErrorFetchingRelation(Error),
    #[error("Error fetching qualified relation. {0}")]
    ErrorFetchingQualifiedRelation(Error),
    #[error("Error fetching resource. {0}")]
    ErrorFetchingResource(Error),
    #[error("Error fetching reference. {0}")]
    ErrorFetchingReference(Error),

    #[error("Error creating catalog. {0}")]
    ErrorCreatingCatalog(Error),
    #[error("Error creating dataset. {0}")]
    ErrorCreatingDataset(Error),
    #[error("Error creating distribution. {0}")]
    ErrorCreatingDistribution(Error),
    #[error("Error creating data service. {0}")]
    ErrorCreatingDataService(Error),
    #[error("Error creating odrl offer. {0}")]
    ErrorCreatingOdrlOffer(Error),
    #[error("Error creating dataset series. {0}")]
    ErrorCreatingDatasetSeries(Error),
    #[error("Error creating catalog record. {0}")]
    ErrorCreatingCatalogRecord(Error),
    #[error("Error creating theme. {0}")]
    ErrorCreatingTheme(Error),
    #[error("Error creating keyword. {0}")]
    ErrorCreatingKeyword(Error),
    #[error("Error creating realtion. {0}")]
    ErrorCreatingRelation(Error),
    #[error("Error creating qualified realtion. {0}")]
    ErrorCreatingQualifiedRelation(Error),
    #[error("Error creating resource. {0}")]
    ErrorCreatingResource(Error),
    #[error("Error creating reference. {0}")]
    ErrorCreatingReference(Error),
    
    #[error("Error deleting catalog. {0}")]
    ErrorDeletingCatalog(Error),
    #[error("Error deleting dataset. {0}")]
    ErrorDeletingDataset(Error),
    #[error("Error deleting distribution. {0}")]
    ErrorDeletingDistribution(Error),
    #[error("Error deleting data service. {0}")]
    ErrorDeletingDataService(Error),
    #[error("Error deleting odrl offer. {0}")]
    ErrorDeletingOdrlOffer(Error),
    #[error("Error deleting dataset series. {0}")]
    ErrorDeletingDatasetSeries(Error),
    #[error("Error deleting catalog record. {0}")]
    ErrorDeletingCatalogRecord(Error),
    #[error("Error deleting theme. {0}")]
    ErrorDeletingTheme(Error),
    #[error("Error deleting keyword. {0}")]
    ErrorDeletingKeyword(Error),
    #[error("Error deleting relation. {0}")]
    ErrorDeletingRelation(Error),
    #[error("Error deleting qualified relation. {0}")]
    ErrorDeletingQualifiedRelation(Error),
    #[error("Error deleting resource. {0}")]
    ErrorDeletingResource(Error),
    #[error("Error deleting reference. {0}")]
    ErrorDeletingReference(Error),

    #[error("Error updating catalog. {0}")]
    ErrorUpdatingCatalog(Error),
    #[error("Error updating dataset. {0}")]
    ErrorUpdatingDataset(Error),
    #[error("Error updating distribution. {0}")]
    ErrorUpdatingDistribution(Error),
    #[error("Error updating data service. {0}")]
    ErrorUpdatingDataService(Error),
    #[error("Error updating odrl offer. {0}")]
    ErrorUpdatingOdrlOffer(Error),
    #[error("Error updating dataset series. {0}")]
    ErrorUpdatingDatasetSeries(Error),
    #[error("Error updating catalog record. {0}")]
    ErrorUpdatingCatalogRecord(Error),
    #[error("Error updating theme. {0}")]
    ErrorUpdatingThemes(Error),
    #[error("Error updating relation. {0}")]
    ErrorUpdatingRelation(Error),
    #[error("Error updating qualified relation. {0}")]
    ErrorUpdatingQualifiedRelation(Error),
    #[error("Error updating resource. {0}")]
    ErrorUpdatingResource(Error),
    #[error("Error updating reference. {0}")]
    ErrorUpdatingReference(Error),

    #[error("Error fetching offer ids. {missing_ids:?}")]
    SomeOdrlOffersNotFound { missing_ids: String },
}