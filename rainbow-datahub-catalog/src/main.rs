use rainbow_common::config::provider_config::ApplicationProviderConfig;
// Adjust the import path below to match your actual module structure.
// For example, if DatahubProxyService is defined in core/datahub_proxy.rs:
// use crate::core::datahub_proxy::DatahubProxyService;
// Or, if it's in core/datahub_proxy/datahub_proxy_service.rs:
// use crate::core::datahub_proxy::datahub_proxy_service::DatahubProxyService;
use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::datahub_proxy::DatahubProxyTrait;
use tokio;
mod core;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Crear la configuración
    let config = ApplicationProviderConfig::default();
    
    // Crear el servicio
    let datahub_service = DatahubProxyService::new(config);
    
    // Obtener y mostrar los dominios
    println!("Obteniendo dominios de DataHub...");
    match datahub_service.get_datahub_domains().await {
        Ok(domains) => {
            println!("\nDominios encontrados:");
            println!("====================");
            for domain in domains {
                println!("URN: {}", domain.urn);
                println!("Nombre: {}", domain.properties.name);
                println!("Descripción: {}", domain.properties.description.unwrap_or_default());
                println!("--------------------");
            }
        },
        Err(e) => println!("Error al obtener los dominios: {}", e),
    }

    Ok(())
}