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

use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{
    EditCatalogModel, EditDataServiceModel, EditDatasetModel, EditDistributionModel,
    NewCatalogModel, NewDataServiceModel, NewDatasetModel, NewDistributionModel,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCatalogRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "foaf:homepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foaf_home_page: Option<String>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dct:issued")]
    pub dct_issued: chrono::NaiveDateTime,
    #[serde(rename = "dct:modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:identifier")]
    pub dct_identifier: String,
    #[serde(rename = "dspace:participantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dspace_participant_id: Option<String>,
    #[serde(rename = "dspace:mainCatalog")]
    pub dspace_main_catalog: bool,
    #[serde(rename = "dct:description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dct:accessRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "dcat:contactPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "ordl:hasPolicy")]
    pub ordl_has_policy: String,
    #[serde(rename = "dcat:landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dct:licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct:publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_publisher: Option<String>,
    #[serde(rename = "prov:qualifiedAttribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_qualified_attribution: Option<String>,
    #[serde(rename = "dcat:hasCurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat:version")]
    pub dcat_version: String,
    #[serde(rename = "dcat:previousVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms:versionNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat:first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat:last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat:prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "dct:replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "adms:status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>,
}

impl Into<NewCatalogModel> for NewCatalogRequest {
    fn into(self) -> NewCatalogModel {
        NewCatalogModel {
            id: self.id,
            foaf_home_page: self.foaf_home_page,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_identifier: self.dct_identifier,
            dct_issued: self.dct_issued,
            dct_modified: self.dct_modified,
            dspace_participant_id: self.dspace_participant_id,
            dspace_main_catalog: self.dspace_main_catalog,
            dct_description: self.dct_description,
            dct_access_rights: self.dct_access_rights,
            dcat_contact_point: self.dcat_contact_point,
            ordl_has_policy: self.ordl_has_policy,
            dcat_landing_page: self.dcat_landing_page,
            dct_licence: self.dct_licence,
            dct_publisher: self.dct_publisher,
            prov_qualified_attribution: self.prov_qualified_attribution,
            dcat_has_current_version: self.dcat_has_current_version,
            dcat_version: self.dcat_version,
            dcat_previous_version: self.dcat_previous_version,
            adms_version_notes: self.adms_version_notes,
            dcat_first: self.dcat_first,
            dcat_last: self.dcat_last,
            dcat_prev: self.dcat_prev,
            dct_replaces: self.dct_replaces,
            adms_status: self.adms_status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditCatalogRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "foaf:homepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foaf_home_page: Option<String>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dct:modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_identifier: Option<String>,
    #[serde(rename = "dct:issued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_issued: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dspace:participantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dspace_participant_id: Option<String>,
    #[serde(rename = "dspace:mainCatalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dspace_main_catalog: Option<bool>,
    #[serde(rename = "dct:description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dct:accessRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "dcat:contactPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "ordl:hasPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordl_has_policy: Option<String>,
    #[serde(rename = "dcat:landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dct:licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct:publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_publisher: Option<String>,
    #[serde(rename = "prov:qualifiedAttribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_qualified_attribution: Option<String>,
    #[serde(rename = "dcat:hasCurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat:version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_version: Option<String>,
    #[serde(rename = "dcat:previousVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms:versionNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat:first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat:last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat:prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "dct:replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "adms:status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>,
}

impl Into<EditCatalogModel> for EditCatalogRequest {
    fn into(self) -> EditCatalogModel {
        EditCatalogModel {
            foaf_home_page: self.foaf_home_page,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_modified: self.dct_modified, 
            dct_identifier: self.dct_identifier, 
            dct_issued: self.dct_issued, 
            dspace_participant_id: self.dspace_participant_id, 
            dspace_main_catalog: self.dspace_main_catalog, 
            dct_description: self.dct_description, 
            dct_access_rights: self.dct_access_rights, 
            dcat_contact_point: self.dcat_contact_point, 
            ordl_has_policy: self.ordl_has_policy, 
            dcat_landing_page: self.dcat_landing_page, 
            dct_licence: self.dct_licence, 
            dct_publisher: self.dct_publisher, 
            prov_qualified_attribution: self.prov_qualified_attribution, 
            dcat_has_current_version: self.dcat_has_current_version, 
            dcat_version: self.dcat_version, 
            dcat_previous_version: self.dcat_previous_version, 
            adms_version_notes: self.adms_version_notes, 
            dcat_first: self.dcat_first, 
            dcat_last: self.dcat_last, 
            dcat_prev: self.dcat_prev, 
            dct_replaces: self.dct_replaces, 
            adms_status: self.adms_status, 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDatasetRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename= "dct:identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_identifier: Option<String>,
    #[serde(rename = "dct:issued")]
    pub dct_issued: chrono::NaiveDateTime,
    #[serde(rename = "dct:modified")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "catalog_id")]
    pub catalog_id: Urn,
    #[serde(rename = "dcat:inseries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_inseries: Option<String>,
    #[serde(rename = "dct:spatial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_spatial: Option<String>,
    #[serde(rename = "dcat:spatialResolutionMeters")]
    pub dcat_spatial_resolution_meters: Option<f64>,
    #[serde(rename = "dct:temporal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal: Option<String>,
    #[serde(rename = "dct:temporalResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal_resolution: Option<String>,
    #[serde(rename = "prov:generatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_generated_by: Option<String>,
    #[serde(rename = "dct:accessRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "dct:license")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_license: Option<String>,
    #[serde(rename = "ordl:hasPolicy")]
    pub ordl_has_policy: String,
    #[serde(rename = "dcat:landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dcat:contactPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "dct:language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_language: Option<String>,
    #[serde(rename = "dct:rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_rights: Option<String>,
    #[serde(rename = "dct:replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "dcat:hasCurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat:version")]
    pub dcat_version: String,
    #[serde(rename = "dcat:previousVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms:versionNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat:first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat:last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat:prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "adms:status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>,
}

impl Into<NewDatasetModel> for NewDatasetRequest {
    fn into(self) -> NewDatasetModel {
        NewDatasetModel {
            id: self.id,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_description: None,
            dct_identifier: self.dct_identifier,
            dct_issued: self.dct_issued,
            dct_modified: self.dct_modified,
            catalog_id: self.catalog_id,
            dcat_inseries: self.dcat_inseries,
            dct_spatial: self.dct_spatial,
            dcat_spatial_resolution_meters: self.dcat_spatial_resolution_meters,
            dct_temporal: self.dct_temporal,
            dct_temporal_resolution: self.dct_temporal_resolution,
            prov_generated_by: self.prov_generated_by,
            dct_access_rights: self.dct_access_rights,
            dct_license: self.dct_license,
            ordl_has_policy: self.ordl_has_policy,
            dcat_landing_page: self.dcat_landing_page,
            dcat_contact_point: self.dcat_contact_point,
            dct_language: self.dct_language,
            dct_rights: self.dct_rights,
            dct_replaces: self.dct_replaces,
            dcat_has_current_version: self.dcat_has_current_version,
            dcat_version: self.dcat_version,
            dcat_previous_version: self.dcat_previous_version,
            adms_version_notes: self.adms_version_notes,
            dcat_first: self.dcat_first,
            dcat_last: self.dcat_last,
            dcat_prev: self.dcat_prev,
            adms_status: self.adms_status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDatasetRequest {
    #[serde(rename = "dct_conforms_to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct_creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct_title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dct_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dct_modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct_identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_identifier: Option<String>,
    #[serde(rename = "dct_issued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_issued: Option<chrono::NaiveDateTime>,
    #[serde(rename = "catalog_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "dcat_inseries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_inseries: Option<String>,
    #[serde(rename = "dct_spatial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_spatial: Option<String>,
    #[serde(rename = "dcat_spatial_resolution_meters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_spatial_resolution_meters: Option<f64>,
    #[serde(rename = "dct_temporal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal: Option<String>,
    #[serde(rename = "dct_temporal_resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal_resolution: Option<String>,
    #[serde(rename = "prov_generated_by")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_generated_by: Option<String>,
    #[serde(rename = "dct_access_rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "dct_license")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_license: Option<String>,
    #[serde(rename = "ordl_has_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordl_has_policy: Option<String>,
    #[serde(rename = "dcat_landing_page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dcat_contact_point")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "dct_language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_language: Option<String>,
    #[serde(rename = "dct_rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_rights: Option<String>,
    #[serde(rename = "dct_replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "dcat_has_current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_version: Option<String>,
    #[serde(rename = "dcat_previous_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms_version_notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat_first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat_last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat_prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "adms_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>,
}

impl Into<EditDatasetModel> for EditDatasetRequest {
    fn into(self) -> EditDatasetModel {
        EditDatasetModel {
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_description: None,
            dct_modified: self.dct_modified,
            dct_identifier: self.dct_identifier,
            dct_issued: self.dct_issued,
            catalog_id: self.catalog_id,
            dcat_inseries: self.dcat_inseries,
            dct_spatial: self.dct_spatial,
            dcat_spatial_resolution_meters: self.dcat_spatial_resolution_meters,
            dct_temporal: self.dct_temporal,
            dct_temporal_resolution: self.dct_temporal_resolution,
            prov_generated_by: self.prov_generated_by,
            dct_access_rights: self.dct_access_rights,
            dct_license: self.dct_license,
            ordl_has_policy: self.ordl_has_policy,
            dcat_landing_page: self.dcat_landing_page,
            dcat_contact_point: self.dcat_contact_point,
            dct_language: self.dct_language,
            dct_rights: self.dct_rights,
            dct_replaces: self.dct_replaces,
            dcat_has_current_version: self.dcat_has_current_version,
            dcat_version: self.dcat_version,
            dcat_previous_version: self.dcat_previous_version,
            adms_version_notes: self.adms_version_notes,
            dcat_first: self.dcat_first,
            dcat_last: self.dcat_last,
            dcat_prev: self.dcat_prev,
            adms_status: self.adms_status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDataServiceRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    pub dcat_endpoint_url: String,
    #[serde(rename = "dct_identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_identifier: Option<String>,
    #[serde(rename = "dct_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dct_issued")]
    pub dct_issued: chrono::NaiveDateTime,
    #[serde(rename = "dct_modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "catalog_id")]
    pub catalog_id: Urn,
    #[serde(rename = "dcat_serves_dataset")]
    pub dcat_serves_dataset: String,
    #[serde(rename = "dcat_access_rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_rights: Option<String>,
    #[serde(rename = "ordl_has_policy")]
    pub ordl_has_policy: String,
    #[serde(rename = "dcat_contact_point")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "dcat_landing_page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dct_licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct_rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_rights: Option<String>,
    #[serde(rename = "dct_publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_publisher: Option<String>,
    #[serde(rename = "prov_qualifed_attribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_qualifed_attribution: Option<String>,
    #[serde(rename = "dcat_has_current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat_version")]
    pub dcat_version: String,
    #[serde(rename = "dcat_previous_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms_version_notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat_first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat_last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat_prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "dct_replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "adms_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>,
}

impl Into<NewDataServiceModel> for NewDataServiceRequest {
    fn into(self) -> NewDataServiceModel {
        NewDataServiceModel {
            id: self.id,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dcat_endpoint_description: self.dcat_endpoint_description,
            dcat_endpoint_url: self.dcat_endpoint_url,
            dct_description: None,
            dct_identifier: self.dct_identifier,
            dct_issued: self.dct_issued,
            dct_modified: self.dct_modified,
            catalog_id: self.catalog_id,
            dcat_serves_dataset: self.dcat_serves_dataset,
            dcat_access_rights: self.dcat_access_rights,
            ordl_has_policy: self.ordl_has_policy,
            dcat_contact_point: self.dcat_contact_point,
            dcat_landing_page: self.dcat_landing_page,
            dct_licence: self.dct_licence,
            dct_rights: self.dct_rights,
            dct_publisher: self.dct_publisher,
            prov_qualifed_attribution: self.prov_qualifed_attribution,
            dcat_has_current_version: self.dcat_has_current_version,
            dcat_version: self.dcat_version,
            dcat_previous_version: self.dcat_previous_version,
            adms_version_notes: self.adms_version_notes,
            dcat_first: self.dcat_first,
            dcat_last: self.dcat_last,
            dcat_prev: self.dcat_prev,
            dct_replaces: self.dct_replaces,
            adms_status: self.adms_status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDataServiceRequest {
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_url: Option<String>,
    #[serde(rename = "dct:description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dct:identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_identifier: Option<String>,
    #[serde(rename = "dct:issued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_issued: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "catalog_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "dcat:servesDataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_serves_dataset: Option<String>,
    #[serde(rename = "dcat:accessRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_rights: Option<String>,
    #[serde(rename = "ordl:hasPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordl_has_policy: Option<String>,
    #[serde(rename = "dcat:contactPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_contact_point: Option<String>,
    #[serde(rename = "dcat:landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_landing_page: Option<String>,
    #[serde(rename = "dct:licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct:rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_rights: Option<String>,
    #[serde(rename = "dct:publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_publisher: Option<String>,
    #[serde(rename = "prov:qualifedAttribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prov_qualifed_attribution: Option<String>,
    #[serde(rename = "dcat:has_currentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_has_current_version: Option<String>,
    #[serde(rename = "dcat:version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_version: Option<String>,
    #[serde(rename = "dcat:previousVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_previous_version: Option<String>,
    #[serde(rename = "adms:versionNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_version_notes: Option<String>,
    #[serde(rename = "dcat:first")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_first: Option<String>,
    #[serde(rename = "dcat:last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_last: Option<String>,
    #[serde(rename = "dcat:prev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_prev: Option<String>,
    #[serde(rename = "dct:replaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_replaces: Option<String>,
    #[serde(rename = "adms:status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adms_status: Option<String>
}

impl Into<EditDataServiceModel> for EditDataServiceRequest {
    fn into(self) -> EditDataServiceModel {
        EditDataServiceModel {
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dcat_endpoint_description: self.dcat_endpoint_description,
            dcat_endpoint_url: self.dcat_endpoint_url,
            dct_description: None,
            dct_identifier: self.dct_identifier,
            dct_issued: self.dct_issued,
            dct_modified: self.dct_modified,
            catalog_id: self.catalog_id,
            dcat_serves_dataset: self.dcat_serves_dataset,
            dcat_access_rights: self.dcat_access_rights,
            ordl_has_policy: self.ordl_has_policy,
            dcat_contact_point: self.dcat_contact_point,
            dcat_landing_page: self.dcat_landing_page,
            dct_licence: self.dct_licence,
            dct_rights: self.dct_rights,
            dct_publisher: self.dct_publisher,
            prov_qualifed_attribution: self.prov_qualifed_attribution,
            dcat_has_current_version: self.dcat_has_current_version,
            dcat_version: self.dcat_version,
            dcat_previous_version: self.dcat_previous_version,
            adms_version_notes: self.adms_version_notes,
            dcat_first: self.dcat_first,
            dcat_last: self.dcat_last,
            dcat_prev: self.dcat_prev,
            dct_replaces: self.dct_replaces,
            adms_status: self.adms_status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewDistributionRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "dct:formats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_formats: Option<DctFormats>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:accessService")]
    pub dcat_access_service: Urn,
    #[serde(rename = "dct:issued")]
    pub dct_issued: chrono::NaiveDateTime,
    #[serde(rename = "dct:modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dataset:id")]
    pub dataset_id: Urn,
    #[serde(rename = "dcat:inseries")]
    pub dcat_inseries: String,
    #[serde(rename = "dcat:access_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_url: Option<String>,
    #[serde(rename = "dcat:download_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_download_url: Option<String>,
    #[serde(rename = "dct:access_rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "ordl:has_policy")]
    pub ordl_has_policy: String,
    #[serde(rename = "dct:conforms_to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:media_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_media_type: Option<String>,
    #[serde(rename = "dcat:compress_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_compress_format: Option<String>,
    #[serde(rename = "dcat:package_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_package_format: Option<String>,
    #[serde(rename = "dct:licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct:rights")]
    pub dct_rights: String,
    #[serde(rename = "dct:spatial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_spatial: Option<String>,
    #[serde(rename = "dct:temporal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal: Option<String>,
    #[serde(rename = "dcat:spatial_resolution_meters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_spatial_resolution_meters: Option<f64>,
    #[serde(rename = "dct:temporal_resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal_resolution: Option<String>,
    #[serde(rename = "dcat:byte_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_byte_size: Option<i64>,
    #[serde(rename = "spdc:checksum")]
    pub spdc_checksum: String,
}

impl Into<NewDistributionModel> for NewDistributionRequest {
    fn into(self) -> NewDistributionModel {
        let id = self.id.as_ref().map(|x| get_urn_from_string(&x).unwrap());
        NewDistributionModel {
            id,
            dct_title: self.dct_title,
            dct_description: None,
            dct_format: self.dct_formats,
            dcat_access_service: self.dcat_access_service,
            dct_issued: self.dct_issued,
            dct_modified: self.dct_modified,
            dataset_id: self.dataset_id,
            dcat_inseries: self.dcat_inseries,
            dcat_access_url: self.dcat_access_url,
            dcat_download_url: self.dcat_download_url,
            dct_access_rights: self.dct_access_rights,
            ordl_has_policy: self.ordl_has_policy,
            dct_conforms_to: self.dct_conforms_to,
            dct_media_type: self.dct_media_type,
            dcat_compress_format: self.dcat_compress_format,
            dcat_package_format: self.dcat_package_format,
            dct_licence: self.dct_licence,
            dct_rights: self.dct_rights,
            dct_spatial: self.dct_spatial,
            dct_temporal: self.dct_temporal,
            dcat_spatial_resolution_meters: self.dcat_spatial_resolution_meters,
            dct_temporal_resolution: self.dct_temporal_resolution,
            dcat_byte_size: self.dcat_byte_size,
            spdc_checksum: self.spdc_checksum,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditDistributionRequest {
    #[serde(rename = "dcttitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dct:description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_description: Option<String>,
    #[serde(rename = "dcat:accessService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_service: Option<Urn>,
    #[serde(rename = "dct:odified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:issued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_issued: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dat:aset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    #[serde(rename = "dct:formats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_formats: Option<DctFormats>,
    #[serde(rename = "dcat:inseries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_inseries: Option<String>,
    #[serde(rename = "dcat:accessUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_url: Option<String>,
    #[serde(rename = "dcat:downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_download_url: Option<String>,
    #[serde(rename = "dct:accessRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_access_rights: Option<String>,
    #[serde(rename = "ordl:hasPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordl_has_policy: Option<String>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_media_type: Option<String>,
    #[serde(rename = "dcat:compressFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_compress_format: Option<String>,
    #[serde(rename = "dcat:packageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_package_format: Option<String>,
    #[serde(rename = "dct:licence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_licence: Option<String>,
    #[serde(rename = "dct:rights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_rights: Option<String>,
    #[serde(rename = "dct:spatial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_spatial: Option<String>,
    #[serde(rename = "dct:temporal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal: Option<String>,
    #[serde(rename = "dcat:spatialResolutionMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_spatial_resolution_meters: Option<f64>,
    #[serde(rename = "dct:temporalResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_temporal_resolution: Option<String>,
    #[serde(rename = "dcat:byteSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_byte_size: Option<i64>,
    pub spdc_checksum: Option<String>
}

impl Into<EditDistributionModel> for EditDistributionRequest {
    fn into(self) -> EditDistributionModel {
        EditDistributionModel {
            dct_title: self.dct_title,
            dct_description: None,
            dct_format: self.dct_formats,
            dct_issued: self.dct_issued,
            dcat_access_service: self.dcat_access_service,
            dct_modified: self.dct_modified,
            dataset_id: self.dataset_id,
            dcat_inseries: self.dcat_inseries,
            dcat_access_url: self.dcat_access_url,
            dcat_download_url: self.dcat_download_url,
            dct_access_rights: self.dct_access_rights,
            ordl_has_policy: self.ordl_has_policy,
            dct_conforms_to: self.dct_conforms_to,
            dct_media_type: self.dct_media_type,
            dcat_compress_format: self.dcat_compress_format,
            dcat_package_format: self.dcat_package_format,
            dct_licence: self.dct_licence,
            dct_rights: self.dct_rights,
            dct_spatial: self.dct_spatial,
            dct_temporal: self.dct_temporal,
            dcat_spatial_resolution_meters: self.dcat_spatial_resolution_meters,
            dct_temporal_resolution: self.dct_temporal_resolution,
            dcat_byte_size: self.dcat_byte_size,
            spdc_checksum: self.spdc_checksum,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct EditDistributionRequest {
//     #[serde(rename = "dct:title")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub dct_title: Option<String>,
//     #[serde(rename = "dcat:accessService")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub dcat_access_service: Option<Urn>,
// }

// impl Into<EditDistributionModel> for EditDistributionRequest {
//     fn into(self) -> EditDistributionModel {
//         EditDistributionModel {
//             dct_title: self.dct_title,
//             dct_description: None,
//             dcat_access_service: self.dcat_access_service.map(|a| a.to_string()),
//         }
//     }
// }
