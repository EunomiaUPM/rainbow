// use rainbow_common::config::provider_config::ApplicationProviderConfig;
// use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
// use crate::core::datahub_proxy::DatahubProxyTrait;
// use tokio;

// mod core;

// async fn print_domains(datahub_service: &DatahubProxyService) -> anyhow::Result<()> {
//     println!("Obteniendo dominios de DataHub...");
//     let domains = datahub_service.get_datahub_domains().await?;
    
//     println!("\nDominios encontrados:");
//     println!("====================");
//     for domain in domains {
//         println!("URN: {}", domain.urn);
//         println!("Nombre: {}", domain.properties.name);
//         println!("Descripción: {}", domain.properties.description.unwrap_or_default());
//         println!("--------------------");
//     }
//     Ok(())
// }

// async fn print_datasets(datahub_service: &DatahubProxyService, domain_urn: &str, domain_name: &str) -> anyhow::Result<()> {
//     println!("\nObteniendo datasets para el dominio {}...", domain_name);
//     match datahub_service.get_datahub_datasets_by_domain_id(domain_urn.to_string()).await {
//         Ok(datasets) => {
//             println!("\nDatasets encontrados:");
//             println!("====================");
//             for dataset in datasets {
//                 println!("URN: {}", dataset.urn);
//                 println!("Nombre: {}", dataset.name);
//                 // println!("Plataforma: {}", dataset.platform.name);
//                 println!("--------------------");
//             }
//         },
//         Err(e) => println!("Error al obtener los datasets: {}", e),
//     }
//     Ok(())
// }

// async fn print_dataset_metadata(datahub_service: &DatahubProxyService, dataset_urn: &str) -> anyhow::Result<()> {
//     println!("\nObteniendo metadatos para el dataset {}...", dataset_urn);
//     match datahub_service.get_datahub_dataset_by_id(dataset_urn.to_string()).await {
//         Ok(dataset) => {
//             println!("URN: {}", dataset.urn);
//             println!("Nombre: {}", dataset.name);
//             // println!("Plataforma: {}", dataset.platform.name);
//             println!("Descripción: {}", dataset.description.unwrap_or_default());
//             println!("Tags: {}", dataset.tag_names.join(", "));
//             println!("Metadatos:");
//             for (key, value) in &dataset.custom_properties {
//                 println!("  {}: {}", key, value);
//             }
//             println!("--------------------");
//             // println!("Política (campo 'policy'):");
//             // // Buscar el campo 'policy' en custom_properties
//             // if let Some(policy_value) = dataset.custom_properties.iter()
//             //     .find_map(|(key, value)| if key == "policy" { Some(value) } else { None }) {
//             //     println!("{}", policy_value);
//             // } else {
//             //     println!("No se encontró el campo 'policy'.");
//             // }
//             // println!("--------------------");
//         },
//         Err(e) => println!("Error al obtener metadatos del dataset: {}", e),
//     }
//     Ok(())
// }


// fn main() {
//     let config = ApplicationProviderConfig::default();
//     let datahub_service = DatahubProxyService::new(config);
    
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.block_on(async {
//         // Print domains
//         if let Err(e) = print_domains(&datahub_service).await {
//             println!("Error printing domains: {}", e);
//         }
        
//         // Print datasets for each domain
//         if let Ok(domains) = datahub_service.get_datahub_domains().await {
//             for domain in domains {
//                 if let Err(e) = print_datasets(&datahub_service, &domain.urn, &domain.properties.name).await {
//                     println!("Error printing datasets for domain {}: {}", domain.properties.name, e);
//                 }
//             }
//         }

//         // Llamar a la función para imprimir metadatos del dataset específico
//         if let Err(e) = print_dataset_metadata(&datahub_service, "urn:li:dataset:(urn:li:dataPlatform:airflow,ASPIRIN_events,PROD)").await {
//             println!("Error al imprimir metadatos del dataset ASPIRIN_events: {}", e);
//         }
//     });
// }

use axum::{
    Router,
};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use tokio;
use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;  // Importamos el router

mod core;
mod http;

#[tokio::main]
async fn main() {
    let config = ApplicationProviderConfig::default();
    let datahub_service = Arc::new(DatahubProxyService::new(config));

    // Creamos el router de datahub_proxy
    let datahub_router = DataHubProxyRouter::new(datahub_service.clone());

    // Montamos el router en la aplicación principal
    let app = Router::new()
        .merge(datahub_router.router());

    println!("🚀 Servidor corriendo en http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}