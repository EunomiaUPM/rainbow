use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251111_000003_connector_distribution_relation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConnectorDistroRelations::Table)
                    .col(
                        ColumnDef::new(ConnectorDistroRelations::DistributionId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConnectorDistroRelations::ConnectorInstanceId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_instance_template")
                            .from(
                                ConnectorDistroRelations::Table,
                                ConnectorDistroRelations::ConnectorInstanceId,
                            )
                            .to(ConnectorInstances::Table, ConnectorInstances::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ConnectorDistroRelations::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum ConnectorDistroRelations {
    Table,
    DistributionId,
    ConnectorInstanceId,
}

#[derive(Iden)]
pub enum ConnectorInstances {
    Table,
    Id,
}
