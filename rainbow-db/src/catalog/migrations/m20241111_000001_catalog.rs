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
        "m20241111_000001_catalog"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogCatalogs::Table)
                    .col(ColumnDef::new(CatalogCatalogs::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CatalogCatalogs::FoafHomePage).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctConformsTo).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctCreator).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctIdentifier).string().not_null())
                    .col(ColumnDef::new(CatalogCatalogs::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(CatalogCatalogs::DctModified).date_time())
                    .col(ColumnDef::new(CatalogCatalogs::DctTitle).string())
                    .col(ColumnDef::new(CatalogCatalogs::DspaceParticipantId).string())
                    .col(ColumnDef::new(CatalogCatalogs::DspaceMainCatalog).boolean().not_null().default(false))
                    .col(ColumnDef::new(CatalogCatalogs::DctDescription).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctAccessRights).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatContactPoint).string())
                    .col(ColumnDef::new(CatalogCatalogs::OrdlHasPolicy).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatLandingPage).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctLicence).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctPublisher).string())
                    .col(ColumnDef::new(CatalogCatalogs::ProvQualifiedAttribution).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatHasCurrentVersion).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatVersion).string().not_null())
                    .col(ColumnDef::new(CatalogCatalogs::DcatPreviousVersion).string())
                    .col(ColumnDef::new(CatalogCatalogs::AdmsVersionNotes).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatFirst).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatLast).string())
                    .col(ColumnDef::new(CatalogCatalogs::DcatPrev).string())
                    .col(ColumnDef::new(CatalogCatalogs::DctReplaces).string())
                    .col(ColumnDef::new(CatalogCatalogs::AdmsStatus).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CatalogCatalogs::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CatalogCatalogs {
    Table,
    Id,
    FoafHomePage,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DspaceParticipantId,
    DspaceMainCatalog,
    DctDescription,
    DctAccessRights,
    DcatContactPoint,
    OrdlHasPolicy,
    DcatLandingPage,
    DctLicence,
    DctPublisher,
    ProvQualifiedAttribution,
    DcatHasCurrentVersion,
    DcatVersion,
    DcatPreviousVersion,
    AdmsVersionNotes,
    DcatFirst,
    DcatLast,
    DcatPrev,
    DctReplaces,
    AdmsStatus,
}
