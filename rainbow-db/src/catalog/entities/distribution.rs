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

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "catalog_distributions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dcat_access_service: String,
    pub dataset_id: String,
    pub dct_format: Option<String>,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dataset::Entity",
        from = "Column::DatasetId",
        to = "super::dataset::Column::Id"
    )]
    Dataset,
    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatAccessService",
        to = "super::dataservice::Column::Id"
    )]
    DataService,
    #[sea_orm(has_many = "super::odrl_offer::Entity")]
    OdrlOffer,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatInseries",
        to = "super::dataset_series::Column::Id"
    )]
    DatsetSeries,
}

impl Related<super::dataset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dataset.def()
    }
}

impl Related<super::dataservice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataService.def()
    }
}

impl Related<super::odrl_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OdrlOffer.def()
    }
}

impl Related<super::dataset_series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DatsetSeries.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
