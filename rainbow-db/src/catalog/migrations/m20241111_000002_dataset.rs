use super::m20241111_000001_catalog::Catalog;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000002_dataset"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dataset::Table)
                    .col(
                        ColumnDef::new(Dataset::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dataset::DctConformsTo).string())
                    .col(ColumnDef::new(Dataset::DctCreator).string())
                    .col(ColumnDef::new(Dataset::DctIdentifier).string())
                    .col(ColumnDef::new(Dataset::DctIssued).date_time().not_null())
                    .col(ColumnDef::new(Dataset::DctModified).date_time())
                    .col(ColumnDef::new(Dataset::DctTitle).string())
                    .col(ColumnDef::new(Dataset::DctDescription).string())
                    .col(ColumnDef::new(Dataset::CatalogId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dataset_catalog")
                            .from(Dataset::Table, Dataset::CatalogId)
                            .to(Catalog::Table, Catalog::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Dataset::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Dataset {
    Table,
    Id,
    DctConformsTo,
    DctCreator,
    DctIdentifier,
    DctIssued,
    DctModified,
    DctTitle,
    DctDescription,
    CatalogId,
}