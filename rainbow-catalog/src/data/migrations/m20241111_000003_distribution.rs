use sea_orm_migration::prelude::*;
use crate::data::migrations::m20241111_000002_dataset::Dataset;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000002_distribution"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Distribution::Table)
                    .col(
                        ColumnDef::new(Distribution::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Distribution::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(Distribution::DctModified).date_time())
                    .col(ColumnDef::new(Distribution::DctTitle).string())
                    .col(ColumnDef::new(Distribution::DctDescription).string())
                    .col(ColumnDef::new(Distribution::DcatAccessService).uuid().not_null())
                    .col(ColumnDef::new(Distribution::DatasetId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_distribution_dataset")
                            .from(Distribution::Table, Distribution::DatasetId)
                            .to(Dataset::Table, Dataset::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Distribution::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Distribution {
    Table,
    Id,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    DcatAccessService,
    DatasetId
}