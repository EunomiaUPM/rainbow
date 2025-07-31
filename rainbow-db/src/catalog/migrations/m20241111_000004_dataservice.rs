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
use super::m20241111_000002_dataset::CatalogDatasets;
use super::m20241111_000005_odrl_offers::CatalogODRLOffers;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000004_dataservice"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogDataServices::Table)
                    .col(ColumnDef::new(CatalogDataServices::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CatalogDataServices::DcatEndpointDescription).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatEndpointURL).string().not_null())
                    .col(ColumnDef::new(CatalogDataServices::DctConformsTo).string())
                    .col(ColumnDef::new(CatalogDataServices::DctCreator).string())
                    .col(ColumnDef::new(CatalogDataServices::DctIdentifier).string())
                    .col(ColumnDef::new(CatalogDataServices::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(CatalogDataServices::DctModified).date_time())
                    .col(ColumnDef::new(CatalogDataServices::DctTitle).string())
                    .col(ColumnDef::new(CatalogDataServices::DctDescription).string())
                    .col(ColumnDef::new(CatalogDataServices::CatalogId).string().not_null())
                    .col(ColumnDef::new(CatalogDataServices::DcatServesDataset).string().not_null())
                    .col(ColumnDef::new(CatalogDataServices::DctAccessRights).string())
                    .col(ColumnDef::new(CatalogDataServices::OrdlHasPolicy).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatContactPoint).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatLandingPage).string())
                    .col(ColumnDef::new(CatalogDataServices::DctLicence).string())
                    .col(ColumnDef::new(CatalogDataServices::DctRights).string())
                    .col(ColumnDef::new(CatalogDataServices::DctPublisher).string())
                    .col(ColumnDef::new(CatalogDataServices::ProvQualifiedAttribution).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatHasCurrentVersion).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatVersion).string().not_null())
                    .col(ColumnDef::new(CatalogDataServices::DcatPreviousVersion).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatReplaces).string())
                    .col(ColumnDef::new(CatalogDataServices::AmdsStatus).string())
                    .col(ColumnDef::new(CatalogDataServices::AdmsVersionNotes).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatFirst).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatLast).string())
                    .col(ColumnDef::new(CatalogDataServices::DcatPrev).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CatalogDataServices::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CatalogDataServices {
    Table,
    Id,
    DcatEndpointDescription,
    DcatEndpointURL,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    CatalogId,
    DcatServesDataset,
    DctAccessRights,
    OrdlHasPolicy,
    DcatContactPoint,
    DcatLandingPage,
    DctLicence,
    DctRights,
    DctPublisher,
    ProvQualifiedAttribution,
    DcatHasCurrentVersion,
    DcatVersion,
    DcatPreviousVersion,
    DcatReplaces,
    AmdsStatus,
    AdmsVersionNotes,
    DcatFirst,
    DcatLast,
    DcatPrev,
}

