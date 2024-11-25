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
use crate::data::migrations::m20241123_0000001_data_plane_process::DataPlaneProcess;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241123_0000002_data_plane_fields"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DataPlaneFields::Table)
                    .col(ColumnDef::new(DataPlaneFields::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(DataPlaneFields::Key).string().not_null())
                    .col(ColumnDef::new(DataPlaneFields::Value).string())
                    .col(
                        ColumnDef::new(DataPlaneFields::DataPlaneProcessId).uuid().not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dataplane_fields_dataplane_process")
                            .from(DataPlaneFields::Table, DataPlaneFields::DataPlaneProcessId)
                            .to(DataPlaneProcess::Table, DataPlaneProcess::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DataPlaneFields::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum DataPlaneFields {
    Table,
    Id,
    Key,
    Value,
    DataPlaneProcessId,
}
