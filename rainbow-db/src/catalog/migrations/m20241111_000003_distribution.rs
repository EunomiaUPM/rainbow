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

use super::m20241111_000002_dataset::Dataset;
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
                    .table(Distribution::Table)
                    .col(ColumnDef::new(Distribution::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Distribution::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(Distribution::DctModified).date_time())
                    .col(ColumnDef::new(Distribution::DctTitle).string())
                    .col(ColumnDef::new(Distribution::DctDescription).string())
                    .col(ColumnDef::new(Distribution::DcatAccessService).uuid().not_null())
                    .col(ColumnDef::new(Distribution::DatasetId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_distribution_dataset")
                            .from(Distribution::Table, Distribution::DatasetId)
                            .to(Dataset::Table, Dataset::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Distribution::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Distribution {
    Table,
    Id,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    DcatAccessService,
    DatasetId,
}
