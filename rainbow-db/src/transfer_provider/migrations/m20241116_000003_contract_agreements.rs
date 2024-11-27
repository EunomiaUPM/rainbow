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
                    .table(ContractAgreements::Table)
                    .col(ColumnDef::new(ContractAgreements::AgreementId).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ContractAgreements::DataServiceId).uuid().not_null())
                    .col(ColumnDef::new(ContractAgreements::Identity).string())
                    .col(ColumnDef::new(ContractAgreements::IdentityToken).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ContractAgreements::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum ContractAgreements {
    Table,
    AgreementId,
    DataServiceId,
    Identity,
    IdentityToken,
}
