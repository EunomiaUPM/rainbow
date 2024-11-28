use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000006_keywords_themes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KeyWords::Table)
                    .col(ColumnDef::new(KeyWords::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(KeyWords::KeyWord).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KeyWords::Table)
                    .col(ColumnDef::new(Themes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Themes::Theme).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(KeyWords::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Themes::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum KeyWords {
    Table,
    Id,
    KeyWord,
}

#[derive(Iden)]
pub enum Themes {
    Table,
    Id,
    Theme,
}
