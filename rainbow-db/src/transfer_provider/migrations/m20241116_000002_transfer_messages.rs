use rainbow_common::protocol::transfer::{TransferMessageTypesForDb, TransferRoles};
use sea_orm::sea_query::extension::postgres::Type;
use sea_orm::ActiveEnum;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241116_000002_transfer_messages"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("transfer_message_type"))
                    .values([
                        Alias::new("dspace:TransferRequestMessage"),
                        Alias::new("dspace:TransferStartMessage"),
                        Alias::new("dspace:TransferSuspensionMessage"),
                        Alias::new("dspace:TransferCompletionMessage"),
                        Alias::new("dspace:TransferTerminationMessage"),
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("transfer_roles"))
                    .values([Alias::new("provider"), Alias::new("consumer")])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransferMessages::Table)
                    .col(ColumnDef::new(TransferMessages::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(TransferMessages::TransferProcessId).uuid().not_null())
                    .col(ColumnDef::new(TransferMessages::CreatedAt).date_time().not_null())
                    .col(
                        ColumnDef::new(TransferMessages::MessageType)
                            .custom(TransferMessageTypesForDb::name())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TransferMessages::From)
                            .custom(TransferRoles::name())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TransferMessages::To)
                            .custom(TransferRoles::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(TransferMessages::Content).json().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferMessages::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(TransferRoles::name()).if_exists().to_owned()).await?;
        manager
            .drop_type(Type::drop().name(TransferMessageTypesForDb::name()).if_exists().to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TransferMessages {
    Table,
    Id,
    TransferProcessId,
    CreatedAt,
    MessageType,
    From,
    To,
    Content,
}
