use sea_orm::{DatabaseConnection, Database};
use crate::datahub::repo::{DatahubConnectorRepoForSql, NewDataHubDatasetModel};

#[tokio::test]
async fn test_create_dataset() {
    // Configurar la conexión a la base de datos
    let db_url = "postgres://ds_transfer_provider:ds_transfer_provider@localhost:1300/ds_transfer_provider";
    let db_connection = Database::connect(db_url).await.unwrap();

    // Crear un nuevo dataset
    let new_dataset = NewDataHubDatasetModel {
        urn: "urn:li:dataset:(urn:li:dataPlatform:airflow,ACETAMINOPHEN_events,PROD)".to_string(),
        name: "ACETAMINOPHEN_events".to_string(),
    };

    // Insertar en la base de datos
    let repo = DatahubConnectorRepoForSql::new(db_connection);
    let dataset = repo.create_datahub_dataset(new_dataset).await.unwrap();

    // Verificar que el dataset se creó correctamente
    assert_eq!(dataset.urn, "urn:li:dataset:(urn:li:dataPlatform:airflow,ACETAMINOPHEN_events,PROD)");
    assert_eq!(dataset.name, "ACETAMINOPHEN_events");
} 