use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241117_000001_transfer_callbacks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TransferCallbacks::Table)
                    .col(ColumnDef::new(TransferCallbacks::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(TransferCallbacks::ConsumerPid).uuid().not_null())
                    .col(ColumnDef::new(TransferCallbacks::ProviderPid).uuid())
                    .col(ColumnDef::new(TransferCallbacks::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TransferCallbacks::UpdatedAt).date_time())
                    .col(ColumnDef::new(TransferCallbacks::DataPlaneId).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferCallbacks::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum TransferCallbacks {
    Table,
    Id,
    ProviderPid,
    ConsumerPid,
    DataAddress,
    CreatedAt,
    UpdatedAt,
    DataPlaneId,
}
