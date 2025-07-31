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
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251807_000001_catalog_record"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogRecord::Table)
                    .col(ColumnDef::new(CatalogRecord::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CatalogRecord::DcatCatalog).string().not_null())
                    .col(ColumnDef::new(CatalogRecord::DctTitle).string().not_null())
                    .col(ColumnDef::new(CatalogRecord::DctDescription).string().not_null())
                    .col(ColumnDef::new(CatalogRecord::DctIssued).timestamp().not_null())
                    .col(ColumnDef::new(CatalogRecord::FoafPrimaryTopic).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CatalogRecord::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CatalogRecord {
    Table,
    Id,
    DcatCatalog,
    DctTitle,
    DctDescription,
    DctIssued,
    FoafPrimaryTopic,
}