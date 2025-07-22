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

use super::m20241111_000005_odrl_offers::CatalogODRLOffers;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251807_000002_dataset_series"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DatasetSeries::Table)
                    .col(ColumnDef::new(DatasetSeries::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(DatasetSeries::DctConformsTo).string())
                    .col(ColumnDef::new(DatasetSeries::DctCreator).string())
                    .col(ColumnDef::new(DatasetSeries::DctIdentifier).string())
                    .col(ColumnDef::new(DatasetSeries::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(DatasetSeries::DctModified).date_time())
                    .col(ColumnDef::new(DatasetSeries::DctTitle).string())
                    .col(ColumnDef::new(DatasetSeries::DctDescription).string())
                    .col(ColumnDef::new(DatasetSeries::DcatDatasetSeries).string())
                    .col(ColumnDef::new(DatasetSeries::DctSpatial).string())
                    .col(ColumnDef::new(DatasetSeries::DcatSpatialResolutionMeters).float())
                    .col(ColumnDef::new(DatasetSeries::DctTemporal).string())
                    .col(ColumnDef::new(DatasetSeries::DctTemporalResolution).string())
                    .col(ColumnDef::new(DatasetSeries::ProvGeneratedBy).string())
                    .col(ColumnDef::new(DatasetSeries::DctAccessRights).string())
                    .col(ColumnDef::new(DatasetSeries::DctLicense).string())
                    .col(ColumnDef::new(DatasetSeries::OrdlHasPolicy).string())
                    .col(ColumnDef::new(DatasetSeries::DcatInSeries).string())
                    .col(ColumnDef::new(DatasetSeries::DcatLandingPage).string())
                    .col(ColumnDef::new(DatasetSeries::DcatContactPoint).string())
                    .col(ColumnDef::new(DatasetSeries::DctLanguage).string())
                    .col(ColumnDef::new(DatasetSeries::DctRights).string())
                    .col(ColumnDef::new(DatasetSeries::DctPublisher).string())
                    .col(ColumnDef::new(DatasetSeries::DctType).string())
                    .col(ColumnDef::new(DatasetSeries::ProvQualifiedAttribution).string())
                    .col(ColumnDef::new(DatasetSeries::AccrualPeriodicity).string())
                    .col(ColumnDef::new(DatasetSeries::DcatVersion).string().not_null())
                    .col(ColumnDef::new(DatasetSeries::DcatHasCurrentVersion).string().not_null())
                    .col(ColumnDef::new(DatasetSeries::DcatPreviousVersion).string())
                    .col(ColumnDef::new(DatasetSeries::DcatFirst).string())
                    .col(ColumnDef::new(DatasetSeries::DcatLast).string())
                    .col(ColumnDef::new(DatasetSeries::DcatPrev).string())
                    .col(ColumnDef::new(DatasetSeries::DctReplaces).string())
                    .col(ColumnDef::new(DatasetSeries::AdmsStatus).string())
                    .col(ColumnDef::new(DatasetSeries::AdmsVersionNotes).string())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_dataset_series_ordl_policy")
                    //         .from(DatasetSeries::Table, DatasetSeries::OrdlHasPolicy)
                    //         .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    //         .on_delete(ForeignKeyAction::Cascade),
                    // )
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_first_dataset")
                    //         .from(DatasetSeries::Table, DatasetSeries::DcatFirst)
                    //         .to(DatasetSeries::Table, DatasetSeries::Id)
                    //         .on_delete(ForeignKeyAction::Cascade),
                    // )
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_last_dataset")
                    //         .from(DatasetSeries::Table, DatasetSeries::DcatLast)
                    //         .to(DatasetSeries::Table, DatasetSeries::Id)
                    //         .on_delete(ForeignKeyAction::Cascade),
                    // )
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_prev_dataset")
                    //         .from(DatasetSeries::Table, DatasetSeries::DcatPrev)
                    //         .to(DatasetSeries::Table, DatasetSeries::Id)
                    //         .on_delete(ForeignKeyAction::Cascade),
                    // )
                    .to_owned(),
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DatasetSeries::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum DatasetSeries {
    Table,
    Id,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    DcatDatasetSeries,
    DctSpatial,
    DcatSpatialResolutionMeters,
    DctTemporal,
    DctTemporalResolution,
    ProvGeneratedBy,
    DctAccessRights,
    DctLicense,
    OrdlHasPolicy,
    DcatInSeries,
    DcatLandingPage,
    DcatContactPoint,
    DctLanguage,
    DctRights,
    DctPublisher,
    DctType,
    ProvQualifiedAttribution,
    AccrualPeriodicity,
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