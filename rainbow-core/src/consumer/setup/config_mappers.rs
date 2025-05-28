use crate::consumer::setup::config::CoreConsumerApplicationConfig;
use rainbow_auth::setup::consumer::{AuthConsumerApplicationConfig, DatabaseConfig, HostConfig, SSIConsumerConfig, SSIConsumerWalletConfig};
use rainbow_common::config::config::ConfigRoles;

impl Into<AuthConsumerApplicationConfig> for CoreConsumerApplicationConfig {
    fn into(self) -> AuthConsumerApplicationConfig {
        AuthConsumerApplicationConfig {
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
            ssi_wallet_config: SSIConsumerWalletConfig {
                consumer_wallet_portal_url: self.ssi_wallet_config.consumer_wallet_portal_url,
                consumer_wallet_portal_port: self.ssi_wallet_config.consumer_wallet_portal_port,
                consumer_wallet_type: self.ssi_wallet_config.consumer_wallet_type,
                consumer_wallet_name: self.ssi_wallet_config.consumer_wallet_name,
                consumer_wallet_email: self.ssi_wallet_config.consumer_wallet_email,
                consumer_wallet_password: self.ssi_wallet_config.consumer_wallet_password,
                consumer_wallet_id: self.ssi_wallet_config.consumer_wallet_id,
            },
            ssi_consumer_client: SSIConsumerConfig {
                consumer_client: self.ssi_consumer_client.consumer_client
            },
            role: ConfigRoles::Consumer,
        }
    }
}
