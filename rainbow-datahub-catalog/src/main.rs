use axum::{
    Router,
};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use tokio;
use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;
use crate::http::rainbow_entities::policy_relations_router::PolicyTemplatesRouter; 
use rainbow_db::datahub::repo::sql::DatahubConnectorRepoForSql;
use rainbow_db::datahub::repo::{NewDataHubDatasetModel, DatahubDatasetsRepo};
use sea_orm::{DatabaseConnection, Database, EntityTrait};

mod core;
mod http;

#[tokio::main]
async fn main() {
    let config = ApplicationProviderConfig::default();
    let datahub_service = Arc::new(DatahubProxyService::new(config));
    let db_connection = Database::connect("postgres://ds_transfer_provider:ds_transfer_provider@127.0.0.1:1300/ds_transfer_provider").await.unwrap();
    
    let policy_templates_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));

    // Obtener datasets del dominio medicamentos
    let domain_id = "urn:li:domain:medicamentos";
    match DatahubProxyTrait::get_datahub_datasets_by_domain_id(&*datahub_service, domain_id.to_string()).await {
        Ok(datasets) => {
            println!("Datasets encontrados: {}", datasets.len());
            
            // Guardar cada dataset en la base de datos
            let repo = DatahubConnectorRepoForSql::new(db_connection.clone());
            for dataset in datasets {
                let new_dataset = NewDataHubDatasetModel {
                    urn: dataset.urn,
                    name: dataset.name,
                };

                // Verificar si el dataset ya existe
                let existing_dataset = rainbow_db::datahub::entities::datahub_datasets::Entity::find_by_id(&new_dataset.urn)
                    .one(&db_connection)
                    .await
                    .unwrap();

                match existing_dataset {
                    Some(_) => println!("El dataset {} ya existe en la base de datos", new_dataset.urn),
                    None => {
                        match repo.create_datahub_dataset(new_dataset).await {
                            Ok(dataset) => println!("Dataset creado: {:?}", dataset),
                            Err(e) => println!("Error al crear el dataset: {:?}", e),
                        }
                    }
                }
            }
        },
        Err(e) => println!("Error al obtener los datasets: {:?}", e),
    }

    // Creamos el router de datahub_proxy
    let datahub_router = DataHubProxyRouter::new(datahub_service.clone());
    let policy_templates_router = PolicyTemplatesRouter::new(policy_templates_service);

    // Montamos el router en la aplicación principal
    let app = Router::new()
        .merge(datahub_router.router())
        .merge(policy_templates_router.router());

    println!("🚀 Servidor corriendo en http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}