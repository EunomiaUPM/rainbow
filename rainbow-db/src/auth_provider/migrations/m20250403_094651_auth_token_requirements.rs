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

use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250403_094651_auth_token_requirements"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthTokenRequirements::Table)
                    .col(ColumnDef::new(AuthTokenRequirements::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(AuthTokenRequirements::Type).string().not_null())
                    .col(ColumnDef::new(AuthTokenRequirements::Actions).array(ColumnType::Text))
                    .col(ColumnDef::new(AuthTokenRequirements::Locations).array(ColumnType::Text))
                    .col(ColumnDef::new(AuthTokenRequirements::Datatypes).array(ColumnType::Text))
                    .col(ColumnDef::new(AuthTokenRequirements::Identifier).string())
                    .col(ColumnDef::new(AuthTokenRequirements::Privileges).array(ColumnType::Text))
                    .col(ColumnDef::new(AuthTokenRequirements::Label).string())
                    .col(ColumnDef::new(AuthTokenRequirements::Flags).array(ColumnType::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AuthTokenRequirements::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum AuthTokenRequirements {
    Table,
    Id,
    Type,
    Actions,
    Locations,
    Datatypes,
    Identifier,
    Privileges,
    Label,
    Flags,
}
