use crate::provider::setup::config::CoreProviderApplicationConfig;
use rainbow_auth::setup::provider::{AuthProviderApplicationConfig, DatabaseConfig, HostConfig, SSIProviderConfig};
use rainbow_common::config::config::ConfigRoles;

impl Into<AuthProviderApplicationConfig> for CoreProviderApplicationConfig {
    fn into(self) -> AuthProviderApplicationConfig {
        AuthProviderApplicationConfig {
            core_host: HostConfig {
                protocol: self.core_host.protocol,
                url: self.core_host.url,
                port: self.core_host.port,
            },
            database_config: DatabaseConfig {
                db_type: self.database_config.db_type,
                url: self.database_config.url,
                port: self.database_config.port,
                user: self.database_config.user,
                password: self.database_config.password,
                name: self.database_config.name,
            },
            ssi_provider_config: SSIProviderConfig {
                provider_verification_portal_url: self.ssi_provider_config.provider_verification_portal_url,
            },
            role: ConfigRoles::Catalog,
        }
    }
}
