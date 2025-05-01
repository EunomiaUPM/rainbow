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

use sea_orm::sea_query::extension::postgres::Type;
use sea_orm::Iterable;
use sea_orm_migration::prelude::*;
use crate::auth_provider::migrations::m20250403_094651_auth::Auth;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250403_094651_auth_verification"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthVerification::Table)
                    .col(ColumnDef::new(AuthVerification::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(AuthVerification::State).string().not_null())
                    .col(ColumnDef::new(AuthVerification::Nonce).string().not_null())
                    .col(ColumnDef::new(AuthVerification::Audience).string().not_null())
                    .col(ColumnDef::new(AuthVerification::Holder).string())
                    .col(ColumnDef::new(AuthVerification::VPT).string())
                    .col(ColumnDef::new(AuthVerification::Success).boolean())
                    .col(ColumnDef::new(AuthVerification::Status).string())
                    .col(ColumnDef::new(Auth::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Auth::EndedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AuthVerification::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum AuthVerification {
    Table,
    Id,
    State,
    Nonce,
    Audience,
    Holder,
    VPT,
    Success,
    Status,
    CreatedAt,
    EndedAt,
}
