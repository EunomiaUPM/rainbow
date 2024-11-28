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

use crate::catalog::migrations::m20241111_000001_catalog::Catalog;
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
                    .table(DataServices::Table)
                    .col(ColumnDef::new(DataServices::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(DataServices::DcatEndpointDescription).string())
                    .col(ColumnDef::new(DataServices::DcatEndpointURL).string().not_null())
                    .col(ColumnDef::new(DataServices::DctConformsTo).string())
                    .col(ColumnDef::new(DataServices::DctCreator).string())
                    .col(ColumnDef::new(DataServices::DctIdentifier).string())
                    .col(ColumnDef::new(DataServices::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(DataServices::DctModified).date_time())
                    .col(ColumnDef::new(DataServices::DctTitle).string())
                    .col(ColumnDef::new(DataServices::DctDescription).string())
                    .col(ColumnDef::new(DataServices::CatalogId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dataservice_catalog")
                            .from(DataServices::Table, DataServices::CatalogId)
                            .to(Catalog::Table, Catalog::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DataServices::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum DataServices {
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
}
