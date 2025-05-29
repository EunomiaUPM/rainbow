use rainbow_common::config::provider_config::ApplicationProviderConfig;
use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::datahub_proxy::DatahubProxyTrait;
use tokio;

mod core;

async fn print_domains(datahub_service: &DatahubProxyService) -> anyhow::Result<()> {
    println!("Obteniendo dominios de DataHub...");
    let domains = datahub_service.get_datahub_domains().await?;
    
    println!("\nDominios encontrados:");
    println!("====================");
    for domain in domains {
        println!("URN: {}", domain.urn);
        println!("Nombre: {}", domain.properties.name);
        println!("Descripción: {}", domain.properties.description.unwrap_or_default());
        println!("--------------------");
    }
    Ok(())
}

async fn print_datasets(datahub_service: &DatahubProxyService, domain_urn: &str, domain_name: &str) -> anyhow::Result<()> {
    println!("\nObteniendo datasets para el dominio {}...", domain_name);
    match datahub_service.get_datahub_datasets_by_domain_id(domain_urn.to_string()).await {
        Ok(datasets) => {
            println!("\nDatasets encontrados:");
            println!("====================");
            for dataset in datasets {
                println!("URN: {}", dataset.urn);
                println!("Nombre: {}", dataset.name);
                // println!("Plataforma: {}", dataset.platform.name);
                println!("--------------------");
            }
        },
        Err(e) => println!("Error al obtener los datasets: {}", e),
    }
    Ok(())
}

fn main() {
    let config = ApplicationProviderConfig::default();
    let datahub_service = DatahubProxyService::new(config);
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Print domains
        if let Err(e) = print_domains(&datahub_service).await {
            println!("Error printing domains: {}", e);
        }
        
        // Print datasets for each domain
        if let Ok(domains) = datahub_service.get_datahub_domains().await {
            for domain in domains {
                if let Err(e) = print_datasets(&datahub_service, &domain.urn, &domain.properties.name).await {
                    println!("Error printing datasets for domain {}: {}", domain.properties.name, e);
                }
            }
        }
    });
}