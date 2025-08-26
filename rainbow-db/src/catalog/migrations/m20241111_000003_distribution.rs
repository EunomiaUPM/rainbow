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

use super::m20241111_000002_dataset::CatalogDatasets;
use super::m20250718_000002_datasetseries::DatasetSeries;
use super::m20241111_000004_dataservice::CatalogDataServices;
use super::m20241111_000005_odrl_offers::CatalogODRLOffers;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000002_distribution"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogDistributions::Table)
                    .col(ColumnDef::new(CatalogDistributions::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CatalogDistributions::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(CatalogDistributions::DctModified).date_time())
                    .col(ColumnDef::new(CatalogDistributions::DctTitle).string())
                    .col(ColumnDef::new(CatalogDistributions::DctDescription).string())
                    .col(ColumnDef::new(CatalogDistributions::DctFormat).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatAccessService).string().not_null())
                    .col(ColumnDef::new(CatalogDistributions::DatasetId).string().not_null())
                    .col(ColumnDef::new(CatalogDistributions::DcatInseries).string().not_null())
                    .col(ColumnDef::new(CatalogDistributions::DcatAccessURL).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatDownloadURL).string())
                    .col(ColumnDef::new(CatalogDistributions::DctAccessRights).string())
                    .col(ColumnDef::new(CatalogDistributions::OrdlHasPolicy).string())
                    .col(ColumnDef::new(CatalogDistributions::DctConformsTo).string())
                    .col(ColumnDef::new(CatalogDistributions::DctMediaType).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatCompressFormat).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatPackageFormat).string())
                    .col(ColumnDef::new(CatalogDistributions::DctLicence).string())
                    .col(ColumnDef::new(CatalogDistributions::DctRights).string().not_null())
                    .col(ColumnDef::new(CatalogDistributions::DctSpatial).string())
                    .col(ColumnDef::new(CatalogDistributions::DctTemporal).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatSpatialResolutionMeters).double())
                    .col(ColumnDef::new(CatalogDistributions::DctTemporalResolution).string())
                    .col(ColumnDef::new(CatalogDistributions::DcatByteSize).big_integer())
                    .col(ColumnDef::new(CatalogDistributions::SpdxChecksum).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CatalogDistributions::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CatalogDistributions {
    Table,
    Id,
    DctIdentifier,
    DatasetId,
    DcatAccessService,
    DcatInseries,
    DcatAccessURL,
    DcatDownloadURL,
    DctAccessRights,
    OrdlHasPolicy,
    DctConformsTo,
    DctFormat,
    DctMediaType,
    DcatCompressFormat,
    DcatPackageFormat,
    DctLicence,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    DctRights,
    DctSpatial,
    DctTemporal,
    DcatSpatialResolutionMeters,
    DctTemporalResolution,
    DcatByteSize,
    SpdxChecksum,
}
