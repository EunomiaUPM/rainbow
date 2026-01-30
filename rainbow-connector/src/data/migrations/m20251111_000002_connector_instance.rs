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
                    .col(ColumnDef::new(ConnectorInstances::TemplateName).string().not_null())
                    .col(ColumnDef::new(ConnectorInstances::TemplateVersion).string().not_null())
                    .col(ColumnDef::new(ConnectorInstances::DistributionId).string().not_null())
                    .col(
                        ColumnDef::new(ConnectorInstances::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ConnectorInstances::Metadata).json_binary().not_null())
                    .col(
                        ColumnDef::new(ConnectorInstances::ConfigurationParameters)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConnectorInstances::Authentication).json_binary().not_null(),
                    )
                    .col(ColumnDef::new(ConnectorInstances::Interaction).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_instance_template")
                            .from(
                                ConnectorInstances::Table,
                                (
                                    ConnectorInstances::TemplateName,
                                    ConnectorInstances::TemplateVersion,
                                ),
                            )
                            .to(
                                ConnectorTemplates::Table,
                                (ConnectorTemplates::Name, ConnectorTemplates::Version),
                            )
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
    TemplateName,
    TemplateVersion,
    DistributionId,
    CreatedAt,
    Metadata,
    ConfigurationParameters,
    Authentication,
    Interaction,
}

#[derive(Iden)]
pub enum ConnectorTemplates {
    Table,
    Name,
    Version,
}
