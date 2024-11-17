use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241116_000003_contract_agreements"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactAgreements::Table)
                    .col(ColumnDef::new(ContactAgreements::AgreementId).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ContactAgreements::DataServiceId).uuid().not_null())
                    .col(ColumnDef::new(ContactAgreements::Identity).string())
                    .col(ColumnDef::new(ContactAgreements::IdentityToken).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ContactAgreements::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum ContactAgreements {
    Table,
    AgreementId,
    DataServiceId,
    Identity,
    IdentityToken,
}
