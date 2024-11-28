use sea_orm::{DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use serde::{Deserialize, Serialize};

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000005_odrl_offers"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("entity_type"))
                    .values([
                        Alias::new("catalog"),
                        Alias::new("dataset"),
                        Alias::new("distribution"),
                        Alias::new("service"),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ODRLOffers::Table)
                    .col(ColumnDef::new(ODRLOffers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ODRLOffers::ODRLOffers).json())
                    .col(ColumnDef::new(ODRLOffers::Entity).uuid().not_null())
                    .col(
                        ColumnDef::new(ODRLOffers::EntityType)
                            .custom(ODRLOffers::EntityType)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ODRLOffers::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(ODRLOffers::EntityType).if_exists().to_owned()).await
    }
}

#[derive(Iden)]
pub enum ODRLOffers {
    Table,
    Id,
    ODRLOffers,
    Entity,
    EntityType,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "entity_type")]
pub enum EntityTypes {
    #[sea_orm(string_value = "catalog")]
    Catalog,
    #[sea_orm(string_value = "dataset")]
    Dataset,
    #[sea_orm(string_value = "distribution")]
    Distribution,
    #[sea_orm(string_value = "service")]
    DataService,
}
