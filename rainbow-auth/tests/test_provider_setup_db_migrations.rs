// Tests corresponding to
// 'rainbow-auth\src\ssi_auth\provider\setup\db_migrations.rs'

#[cfg(test)]
mod tests {
<<<<<<< HEAD
    use std::env;
    use std::panic::AssertUnwindSafe;

    use futures::FutureExt;
    use rainbow_auth::ssi_auth::provider::setup::db_migrations::SSIAuthProviderMigrations;
    use rainbow_common::config::{
        database::DbType, global_config::DatabaseConfig, provider_config::ApplicationProviderConfig
=======
    use futures::FutureExt;
    use rainbow_auth::ssi_auth::provider::setup::db_migrations::SSIAuthProviderMigrations;
    use rainbow_common::config::{
        database::DbType, global_config::DatabaseConfig, provider_config::ApplicationProviderConfig,
>>>>>>> origin/main
    };
    use sea_orm::Database;
    use sea_orm::{sea_query, DbErr};
    use sea_orm_migration::MigrationName;
    use sea_orm_migration::{async_trait, SchemaManager};
    use sea_orm_migration::{MigrationTrait, MigratorTrait};
<<<<<<< HEAD
=======
    use std::env;
    use std::panic::AssertUnwindSafe;
>>>>>>> origin/main

    struct TestMigration;

    #[async_trait::async_trait]
    impl MigrationTrait for TestMigration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    sea_query::Table::create()
                        .table(sea_query::Alias::new("dummy"))
                        .if_not_exists()
<<<<<<< HEAD
                        .col(
                            sea_query::ColumnDef::new(sea_query::Alias::new("id"))
                                .integer()
                                .not_null()
                                .primary_key()
                        )
                        .to_owned()
=======
                        .col(sea_query::ColumnDef::new(sea_query::Alias::new("id")).integer().not_null().primary_key())
                        .to_owned(),
>>>>>>> origin/main
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
<<<<<<< HEAD
                .drop_table(
                    sea_query::Table::drop()
                        .table(sea_query::Alias::new("dummy"))
                        .if_exists()
                        .to_owned()
                )
=======
                .drop_table(sea_query::Table::drop().table(sea_query::Alias::new("dummy")).if_exists().to_owned())
>>>>>>> origin/main
                .await
        }
    }

    impl MigrationName for TestMigration {
        fn name(&self) -> &str { "DummyMigration" }
    }

    struct MockedMigrations;

    impl MigratorTrait for MockedMigrations {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> { vec![Box::new(TestMigration)] }
    }

    fn sqlite_config() -> ApplicationProviderConfig {
        let mut config = ApplicationProviderConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Sqlite,
            url: "/tmp/test_migrations.db".to_string(),
            port: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            name: "".to_string()
        };
        config
    }

    // Test

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
        let db: DbConn = Database::connect(db_url).await.expect("Database can't connect");

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
            name: "db".to_string()
        };

        let result = AssertUnwindSafe(SSIAuthProviderMigrations::run(&config)).catch_unwind().await;

        assert!(
            result.is_err(),
            "Expected panic due to DB connection failure, but function completed successfully"
        );
    }

    #[tokio::test]
    async fn test_run_with_test_mode_should_skip_db() {
        env::set_var("TEST_MODE", "1");
        let config = sqlite_config();
        let result = SSIAuthProviderMigrations::run(&config).await;
        env::remove_var("TEST_MODE");
        assert!(result.is_ok(), "Expected Ok(()) when TEST_MODE is active");
    }

    #[tokio::test]
    async fn test_run_with_mysql_should_fail() {
        let mut config = ApplicationProviderConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Mysql,
            url: "invalid_mysql".to_string(),
            port: "3306".to_string(),
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string()
        };
        let result = std::panic::AssertUnwindSafe(SSIAuthProviderMigrations::run(&config)).catch_unwind().await;
        assert!(result.is_err(), "Expected panic for invalid MySQL config");
    }

    #[tokio::test]
    async fn test_run_with_postgres_should_fail() {
        let mut config = ApplicationProviderConfig::default();
        config.database_config = DatabaseConfig {
            db_type: DbType::Postgres,
            url: "invalid_pg".to_string(),
            port: "5432".to_string(),
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string()
        };
        let result = std::panic::AssertUnwindSafe(SSIAuthProviderMigrations::run(&config)).catch_unwind().await;
        assert!(
            result.is_err(),
            "Expected panic for invalid Postgres config"
        );
    }

    #[tokio::test]
    async fn test_up_and_down_should_succeed() {
        let db = Database::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
        let manager = SchemaManager::new(&db);
        let migration = TestMigration;
        assert!(
            migration.up(&manager).await.is_ok(),
            "Expected up() to succeed"
        );
        assert!(
            migration.down(&manager).await.is_ok(),
            "Expected down() to succeed"
        );
    }

    #[test]
    fn test_empty_migrations_vector() {
        struct EmptyMigrator;
        impl MigratorTrait for EmptyMigrator {
            fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> { vec![] }
        }
        assert!(
            EmptyMigrator::migrations().is_empty(),
            "Expected empty migrations"
        );
    }
}
