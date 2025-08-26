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
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dataset_series")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
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
    pub ordl_has_policy: Option<String>,
    pub dct_license: Option<String>,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
    #[sea_orm(has_many = "super::odrl_offer::Entity")]
    OdrlOffer,
    
    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatInseries",
        to = "super::dataset_series::Column::Id"
    )]
    InSeries,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatPreviousVersion",
        to = "super::dataset_series::Column::Id"
    )]
    PreviousVersion,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DctReplaces",
        to = "super::dataset_series::Column::Id"
    )]
    Replaces,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatLast",
        to = "super::dataset_series::Column::Id"
    )]
    Last,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatFirst",
        to = "super::dataset_series::Column::Id"
    )]
    First,

    #[sea_orm(
        belongs_to = "super::dataset_series::Entity",
        from = "Column::DcatPrev",
        to = "super::dataset_series::Column::Id"
    )]
    Prev,
}

pub struct BelongsToPreviousVersion;
pub struct BelongsToReplaces;
pub struct BelongsToLast;
pub struct BelongsToFirst;
pub struct BelongsToPrev;

impl Related<super::odrl_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OdrlOffer.def()
    }
}

impl Related<super::dataset_series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InSeries.def()
    }
}

impl Related<super::dataset_series::Entity> for BelongsToPreviousVersion {
    fn to() -> RelationDef {
        Relation::PreviousVersion.def()
    }
}

impl Related<super::dataset_series::Entity> for BelongsToReplaces {
    fn to() -> RelationDef {
        Relation::Replaces.def()
    }
}

impl Related<super::dataset_series::Entity> for BelongsToLast {
    fn to() -> RelationDef {
        Relation::Last.def()
    }
}

impl Related<super::dataset_series::Entity> for BelongsToFirst {
    fn to() -> RelationDef {
        Relation::First.def()
    }
}

impl Related<super::dataset_series::Entity> for BelongsToPrev {
    fn to() -> RelationDef {
        Relation::Prev.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}