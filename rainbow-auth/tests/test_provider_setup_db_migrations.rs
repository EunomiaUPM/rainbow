// Tests corresponding to 'rainbow-auth\src\ssi_auth\provider\setup\db_migrations.rs'

#[cfg(test)]
mod tests {
    use rainbow_auth::ssi_auth::provider::setup::db_migrations::SSIAuthProviderMigrations;
    use rainbow_common::config::{database::DbType, global_config::DatabaseConfig, provider_config::ApplicationProviderConfig};
    use sea_orm_migration::{MigrationTrait, MigratorTrait};
    use std::panic::AssertUnwindSafe;
    use futures::FutureExt;
    use sea_orm::{DbErr, sea_query};
    use sea_orm_migration::{SchemaManager, async_trait};
    use sea_orm_migration::{MigrationName};

    struct TestMigration;

    #[async_trait::async_trait]
    impl MigrationTrait for TestMigration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    sea_query::Table::create()
                        .table(sea_query::Alias::new("dummy"))
                        .if_not_exists()
                        .col(
                            sea_query::ColumnDef::new(sea_query::Alias::new("id"))
                                .integer()
                                .not_null()
                                .primary_key(),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(
                    sea_query::Table::drop()
                        .table(sea_query::Alias::new("dummy"))
                        .if_exists()
                        .to_owned(),
                )
                .await
        }
    }

    impl MigrationName for TestMigration {
        fn name(&self) -> &str {
            "DummyMigration"
        }
    }

    struct MockedMigrations;

    impl MigratorTrait for MockedMigrations {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![Box::new(TestMigration)]
        }
    }

    #[test]
    fn test_migrations_success() {
        let migrations = SSIAuthProviderMigrations::migrations();

        assert!(!migrations.is_empty(), "Expected at least one migration");
    }

    #[tokio::test]
    async fn test_run_success_simulated() {
        use sea_orm::Database;
        use sea_orm::DbConn;

        // Use the valid URL for in-memory SQLite.
        let db_url = "sqlite::memory:";

        // Connect to database
        let db: DbConn = Database::connect(db_url)
            .await
            .expect("Database can't connect");

        // Run the simulated migrations
        let result = MockedMigrations::refresh(&db).await;

        // We verify that they have been successful
        assert!(result.is_ok(), "Expected refresh() to succeed");
    }

    #[tokio::test]
    async fn test_migration_run_fails_with_invalid_db() {
        let mut config = ApplicationProviderConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Postgres,
            url: "invalid_host".to_string(),
            port: "5432".to_string(),
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string(),
        };

        let result = AssertUnwindSafe(SSIAuthProviderMigrations::run(&config))
            .catch_unwind()
            .await;

        assert!(
            result.is_err(),
            "Expected panic due to DB connection failure, but function completed successfully"
        );
    }
}