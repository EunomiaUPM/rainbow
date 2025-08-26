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

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "catalog_data_services")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dcat_endpoint_description: Option<String>,
    pub dcat_endpoint_url: String,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_identifier: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub catalog_id: String,
    pub dcat_serves_dataset: String,
    pub dcat_access_rights: Option<String>,
    pub ordl_has_policy: Option<String>,
    pub dcat_contact_point: Option<String>,
    pub dcat_landing_page: Option<String>,
    pub dct_licence: Option<String>,
    pub dct_rights: Option<String>,
    pub dct_publisher: Option<String>,
    pub prov_qualified_attribution: Option<String>,
    pub dcat_has_current_version: Option<String>,
    pub dcat_version: String,
    pub dcat_previous_version: Option<String>,
    pub adms_version_notes: Option<String>,
    pub dcat_first: Option<String>,
    pub dcat_last: Option<String>,
    pub dcat_prev: Option<String>,
    pub dcat_replaces: Option<String>,
    pub adms_status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::catalog::Entity",
        from = "Column::CatalogId",
        to = "super::catalog::Column::Id"
    )]
    Catalog,
    #[sea_orm(has_many = "super::odrl_offer::Entity")]
    OdrlOffer,

    #[sea_orm(
        belongs_to = "super::dataset::Entity",
        from = "Column::DcatServesDataset",
        to = "super::dataset::Column::Id"
    )]
    Dataset,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatHasCurrentVersion",
        to = "Column::Id"
    )]
    CurrentVersion,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatPreviousVersion",
        to = "Column::Id"
    )]
    PreviousVersion,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatReplaces",
        to = "Column::Id"
    )]
    Replaces,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatLast",
        to = "Column::Id"
    )]
    Last,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatFirst",
        to = "Column::Id"
    )]
    First,

    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatPrev",
        to = "Column::Id"
    )]
    Prev,
}


pub struct BelongsToCurrentVersion;
pub struct BelongsToPreviousVersion;
pub struct BelongsToReplaces;
pub struct BelongsToLast;
pub struct BelongsToFirst;
pub struct BelongsToPrev;


impl Related<super::catalog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}

impl Related<super::odrl_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OdrlOffer.def()
    }
}

impl Related<super::dataset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dataset.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToCurrentVersion {
    fn to() -> RelationDef {
        Relation::CurrentVersion.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToPreviousVersion {
    fn to() -> RelationDef {
        Relation::PreviousVersion.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToReplaces {
    fn to() -> RelationDef {
        Relation::Replaces.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToLast {
    fn to() -> RelationDef {
        Relation::Last.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToFirst {
    fn to() -> RelationDef {
        Relation::First.def()
    }
}

impl Related<super::dataservice::Entity> for BelongsToPrev {
    fn to() -> RelationDef {
        Relation::Prev.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
