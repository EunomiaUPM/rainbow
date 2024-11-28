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

use rainbow_common::protocol::transfer::TransferStateForDb;
use sea_orm::sea_query::extension::postgres::Type;
use sea_orm::ActiveEnum;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241116_000001_transfer_processes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("transfer_state"))
                    .values([
                        Alias::new("dspace:REQUESTED"),
                        Alias::new("dspace:STARTED"),
                        Alias::new("dspace:TERMINATED"),
                        Alias::new("dspace:COMPLETED"),
                        Alias::new("dspace:SUSPENDED"),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransferProcesses::Table)
                    .col(
                        ColumnDef::new(TransferProcesses::ProviderPid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TransferProcesses::ConsumerPid).uuid())
                    .col(ColumnDef::new(TransferProcesses::AgreementId).uuid())
                    .col(ColumnDef::new(TransferProcesses::DataPlaneId).uuid())
                    .col(
                        ColumnDef::new(TransferProcesses::State)
                            .custom(TransferStateForDb::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(TransferProcesses::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TransferProcesses::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferProcesses::Table).to_owned()).await?;
        manager
            .drop_type(Type::drop().name(TransferStateForDb::name()).if_exists().to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TransferProcesses {
    Table,
    ProviderPid,
    ConsumerPid,
    AgreementId,
    DataPlaneId,
    State,
    CreatedAt,
    UpdatedAt,
}
