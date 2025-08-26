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

use super::m20241111_000001_catalog::CatalogCatalogs;
use super::m20250718_000002_datasetseries::DatasetSeries;
use super::m20241111_000005_odrl_offers::CatalogODRLOffers;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000002_dataset"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogDatasets::Table)
                    .col(ColumnDef::new(CatalogDatasets::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CatalogDatasets::DctConformsTo).string())
                    .col(ColumnDef::new(CatalogDatasets::DctCreator).string())
                    .col(ColumnDef::new(CatalogDatasets::DctIdentifier).string())
                    .col(ColumnDef::new(CatalogDatasets::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(CatalogDatasets::DctModified).date_time())
                    .col(ColumnDef::new(CatalogDatasets::DctTitle).string())
                    .col(ColumnDef::new(CatalogDatasets::DctDescription).string())
                    .col(ColumnDef::new(CatalogDatasets::CatalogId).string().not_null())
                    .col(ColumnDef::new(CatalogDatasets::DcatInseries).string())
                    .col(ColumnDef::new(CatalogDatasets::DctSpatial).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatSpatialResolutionMeters).double())
                    .col(ColumnDef::new(CatalogDatasets::DctTemporal).string())
                    .col(ColumnDef::new(CatalogDatasets::DctTemporalResolution).string())
                    .col(ColumnDef::new(CatalogDatasets::ProvGeneratedBy).string())
                    .col(ColumnDef::new(CatalogDatasets::DctAccessRights).string())
                    .col(ColumnDef::new(CatalogDatasets::DctLicense).string())
                    .col(ColumnDef::new(CatalogDatasets::OrdlHasPolicy).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatLandingPage).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatContactPoint).string())
                    .col(ColumnDef::new(CatalogDatasets::DctLanguage).string())
                    .col(ColumnDef::new(CatalogDatasets::DctRights).string())
                    .col(ColumnDef::new(CatalogDatasets::DctPublisher).string())
                    .col(ColumnDef::new(CatalogDatasets::DctType).string())
                    .col(ColumnDef::new(CatalogDatasets::ProvQualifiedAttribution).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatVersion).string().not_null())
                    .col(ColumnDef::new(CatalogDatasets::DcatHasCurrentVersion).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatPreviousVersion).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatFirst).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatLast).string())
                    .col(ColumnDef::new(CatalogDatasets::DcatPrev).string())
                    .col(ColumnDef::new(CatalogDatasets::DctReplaces).string())
                    .col(ColumnDef::new(CatalogDatasets::AdmsStatus).string())
                    .col(ColumnDef::new(CatalogDatasets::AdmsVersionNotes).string())
                    .to_owned(),
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CatalogDatasets::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CatalogDatasets {
    Table,
    Id,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    CatalogId,
    DcatInseries,
    DctSpatial,
    DcatSpatialResolutionMeters,
    DctTemporal,
    DctTemporalResolution,
    ProvGeneratedBy,
    DctAccessRights,
    DctLicense,
    OrdlHasPolicy,
    DcatLandingPage,
    DcatContactPoint,
    DctLanguage,
    DctRights,
    DctPublisher,
    DctType,
    ProvQualifiedAttribution,
    DcatVersion,
    DcatHasCurrentVersion,
    DcatPreviousVersion,
    DcatFirst,
    DcatLast,
    DcatPrev,
    DctReplaces,
    AdmsStatus,
    AdmsVersionNotes,
}
