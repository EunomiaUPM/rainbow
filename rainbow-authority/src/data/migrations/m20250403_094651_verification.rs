/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250403_094651_verification"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .col(ColumnDef::new(Verification::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Verification::State).string().not_null())
                    .col(ColumnDef::new(Verification::Nonce).string().not_null())
                    .col(ColumnDef::new(Verification::VcType).string().not_null())
                    .col(ColumnDef::new(Verification::Audience).string().not_null())
                    .col(ColumnDef::new(Verification::Holder).string())
                    .col(ColumnDef::new(Verification::Vpt).string())
                    .col(ColumnDef::new(Verification::Success).boolean())
                    .col(ColumnDef::new(Verification::Status).string().not_null())
                    .col(ColumnDef::new(Verification::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Verification::EndedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Verification::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Verification {
    Table,
    Id,
    State,
    Nonce,
    VcType,
    Audience,
    Holder,
    Vpt,
    Success,
    Status,
    CreatedAt,
    EndedAt,
}
