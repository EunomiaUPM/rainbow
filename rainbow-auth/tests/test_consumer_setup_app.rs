// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\setup\app.rs'

#[cfg(test)]
mod tests {
    use futures::FutureExt;
    use rainbow_auth::ssi_auth::consumer::setup::{
        app::{create_ssi_consumer_router, SSIAuthConsumerApplication},
        db_migrations::SSIAuthConsumerMigrations,
    };
    use rainbow_common::config::{
        consumer_config::ApplicationConsumerConfig, database::DbType, global_config::DatabaseConfig,
    };
    use sea_orm::DbErr;
    use sea_orm_migration::sea_query::{Alias, ColumnDef, Table};
    use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait, SchemaManager};
    use std::{env, panic::AssertUnwindSafe};

    // Mock

    struct DummyMigration;

    #[async_trait::async_trait]
    impl MigrationTrait for DummyMigration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Alias::new("dummy"))
                        .if_not_exists()
                        .col(ColumnDef::new(Alias::new("id")).integer().not_null().primary_key())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager.drop_table(Table::drop().table(Alias::new("dummy")).if_exists().to_owned()).await
        }
    }

    impl sea_orm_migration::MigrationName for DummyMigration {
        fn name(&self) -> &str {
            "DummyMigration"
        }
    }

    struct MockedMigrations;

    impl MigratorTrait for MockedMigrations {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![Box::new(DummyMigration)]
        }
    }

    fn valid_config() -> ApplicationConsumerConfig {
        let mut config = ApplicationConsumerConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Sqlite,
            url: "/tmp/test.db".to_string(),
            port: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
        };
        config.is_local = true;
        config
    }

    // Test

    #[tokio::test]
    async fn test_refresh_success() {
        use sea_orm::Database;
        use sea_orm::DbConn;

        let db_url = "sqlite::memory:";
        let db: DbConn = Database::connect(db_url).await.expect("Database can't connect");

        let result = MockedMigrations::refresh(&db).await;
        assert!(result.is_ok(), "Expected refresh() to succeed");
    }

    #[tokio::test]
    async fn test_refresh_failure_invalid_url() {
        use crate::tests::MockedMigrations;
        use sea_orm::Database;

        let db_url = "invalid::url";

        let result = Database::connect(db_url).await;

        assert!(
            result.is_err(),
            "Expected connection to fail with invalid URL"
        );

        if let Ok(db) = result {
            let refresh_result = MockedMigrations::refresh(&db).await;
            assert!(refresh_result.is_err(), "Expected refresh to fail");
        }
    }

    #[test]
    fn test_migrations_success() {
        let migrations = SSIAuthConsumerMigrations::migrations();

        assert!(!migrations.is_empty(), "Expected at least one migration");
    }

    #[tokio::test]
    async fn test_run_environment_scenario_false_with_test_mode() {
        env::set_var("TEST_MODE", "1");
        let mut config = valid_config();
        config.is_local = false;
        let result = SSIAuthConsumerApplication::run(&config).await;
        env::remove_var("TEST_MODE");
        assert!(result.is_ok(), "Expected Ok(()) when TEST_MODE is active");
    }

    #[tokio::test]
    async fn test_run_success_simulated() {
        use sea_orm::Database;
        use sea_orm::DbConn;

        // Use the valid URL for in-memory SQLite.
        let db_url = "sqlite::memory:";

        // Connect to database
        let db: DbConn = Database::connect(db_url).await.expect("Database can't connect");

        // Run the simulated migrations
        let result = MockedMigrations::refresh(&db).await;

        // We verify that they have been successful
        assert!(result.is_ok(), "Expected refresh() to succeed");
    }

    #[tokio::test]
    async fn test_create_ssi_provider_router_should_panic_on_invalid_db() {
        let mut config = ApplicationConsumerConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Postgres,
            url: "invalid_host".to_string(),
            port: "5432".to_string(),
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string(),
        };

        let result = AssertUnwindSafe(create_ssi_consumer_router(config)).catch_unwind().await;

        assert!(
            result.is_err(),
            "Expected panic due to DB connection failure, but function completed successfully"
        );
    }
}
