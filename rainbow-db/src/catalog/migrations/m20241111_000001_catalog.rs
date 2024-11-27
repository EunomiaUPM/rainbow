use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000001_catalog"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Catalog::Table)
                    .col(ColumnDef::new(Catalog::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Catalog::FoafHomePage).string())
                    .col(ColumnDef::new(Catalog::DctConformsTo).string())
                    .col(ColumnDef::new(Catalog::DctCreator).string())
                    .col(ColumnDef::new(Catalog::DctIdentifier).string())
                    .col(ColumnDef::new(Catalog::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(Catalog::DctModified).date_time())
                    .col(ColumnDef::new(Catalog::DctTitle).string())
                    .col(ColumnDef::new(Catalog::DspaceParticipantId).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Catalog::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Catalog {
    Table,
    Id,
    FoafHomePage,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DspaceParticipantId,
}
