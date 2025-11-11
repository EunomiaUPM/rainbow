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
        "m20250403_094651_request"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Request::Table)
                    .col(ColumnDef::new(Request::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Request::ParticipantSlug).string().not_null())
                    .col(ColumnDef::new(Request::VcType).string().not_null())
                    .col(ColumnDef::new(Request::Cert).string())
                    .col(ColumnDef::new(Request::VcUri).string())
                    .col(ColumnDef::new(Request::VcIssuing).string())
                    .col(ColumnDef::new(Request::IsVcIssued).boolean())
                    .col(ColumnDef::new(Request::Status).string().not_null())
                    .col(ColumnDef::new(Request::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Request::EndedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Request::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Request {
    Table,
    Id,
    ParticipantSlug,
    VcType,
    Cert,
    VcUri,
    VcIssuing,
    IsVcIssued,
    Status,
    CreatedAt,
    EndedAt,
}
