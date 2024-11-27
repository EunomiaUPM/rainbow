use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000007_descriptions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DcatDescriptions::Table)
                    .col(ColumnDef::new(DcatDescriptions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(DcatDescriptions::Description).string().not_null())
                    .col(ColumnDef::new(DcatDescriptions::DescriptionLanguage).string().not_null())
                    .col(ColumnDef::new(DcatDescriptions::Entity).uuid().not_null())
                    .col(
                        ColumnDef::new(DcatDescriptions::EntityType)
                            .enumeration(
                                DcatDescriptions::EntityType,
                                [
                                    EntityTypes::Catalog,
                                    EntityTypes::Dataset,
                                    EntityTypes::Distribution,
                                    EntityTypes::DataService,
                                ],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DcatDescriptions::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum DcatDescriptions {
    Table,
    Id,
    Description,
    DescriptionLanguage,
    Entity,
    EntityType,
}

#[derive(Iden)]
enum EntityTypes {
    #[iden = "catalog"]
    Catalog,
    #[iden = "dataset"]
    Dataset,
    #[iden = "distribution"]
    Distribution,
    #[iden = "dataService"]
    DataService,
}
