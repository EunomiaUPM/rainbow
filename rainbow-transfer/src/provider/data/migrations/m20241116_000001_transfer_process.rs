use crate::protocol::messages::TransferStateForDb;
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
                    .col(ColumnDef::new(TransferProcesses::SubscriptionId).string())
                    .col(
                        ColumnDef::new(TransferProcesses::State)
                            .custom(TransferStateForDb::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(TransferProcesses::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TransferProcesses::UpdatedAt).date_time())
                    .col(ColumnDef::new(TransferProcesses::DataPlaneAddress).string())
                    .col(ColumnDef::new(TransferProcesses::NextHopAddress).json())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferProcesses::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(TransferStateForDb::name()).if_exists().to_owned()).await
    }
}

#[derive(Iden)]
pub enum TransferProcesses {
    Table,
    ProviderPid,
    ConsumerPid,
    AgreementId,
    DataPlaneId,
    SubscriptionId,
    State,
    CreatedAt,
    UpdatedAt,
    DataPlaneAddress,
    NextHopAddress,
}
