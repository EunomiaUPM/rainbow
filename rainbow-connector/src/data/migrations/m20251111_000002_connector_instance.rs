use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251111_000002_connector_instance"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConnectorInstances::Table)
                    .col(ColumnDef::new(ConnectorInstances::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ConnectorInstances::TemplateId).string().not_null())
                    .col(ColumnDef::new(ConnectorInstances::TemplateVersion).string().not_null())
                    .col(ColumnDef::new(ConnectorInstances::DistributionId).string().not_null())
                    .col(ColumnDef::new(ConnectorInstances::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ConnectorInstances::ConfigurationValues).json_binary().not_null())
                    .col(ColumnDef::new(ConnectorInstances::RuntimeContext).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_instance_template")
                            .from(ConnectorInstances::Table, ConnectorInstances::TemplateId)
                            .to(ConnectorTemplates::Table, ConnectorTemplates::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ConnectorInstances::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum ConnectorInstances {
    Table,
    Id,
    TemplateId,
    TemplateVersion,
    DistributionId,
    CurrentState,
    CreatedAt,
    UpdatedAt,
    LockVersion,
    ConfigurationValues,
    RuntimeContext,
}

#[derive(Iden)]
pub enum ConnectorTemplates {
    Table,
    Id,
    Name,
    Version,
    Author,
    CreatedAt,
    Spec,
}
