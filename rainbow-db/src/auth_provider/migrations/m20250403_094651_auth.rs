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

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250403_094651_auth"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Auth::Table)
                    .col(ColumnDef::new(Auth::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Auth::Consumer).string().not_null())
                    .col(ColumnDef::new(Auth::Actions).json().not_null())
                    .col(ColumnDef::new(Auth::Status).string().not_null())
                    .col(ColumnDef::new(Auth::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Auth::EndedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Auth::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Auth {
    Table,
    Id,
    Consumer,
    Actions,
    Status,
    CreatedAt,
    EndedAt,
}
