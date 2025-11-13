// Tests corresponding to 'rainbow-auth\src\ssi_auth\provider\core\mod.rs'

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rainbow_common::config::provider_config::ApplicationProviderConfig;
    use rainbow_db::auth_provider::repo_factory::{factory_trait::AuthRepoFactoryTrait, traits::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait, AuthVerificationRepoTrait, BusinessMatesRepoTrait, MatesRepoTrait}};

    // Mock

    #[derive(Clone)]
    pub struct MockRepoFactory;

    impl AuthRepoFactoryTrait for MockRepoFactory {
        fn request(&self) -> Arc<dyn AuthRequestRepoTrait> {
            unimplemented!()
        }
        fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> {
            unimplemented!()
        }
        fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> {
            unimplemented!()
        }
        fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> {
            unimplemented!()
        }
        fn mates(&self) -> Arc<dyn MatesRepoTrait> {
            unimplemented!()
        }
        fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait> {
            unimplemented!()
        }
    }

    pub fn mock_config(base_url: &str) -> ApplicationProviderConfig {
        let mut config = ApplicationProviderConfig::default();

        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_api_url = base_url.replace("http://", "").replace("https://", "");
        config.ssi_wallet_config.wallet_api_port = None; // O Some("7001".to_string()) si quieres incluir puerto
        config.ssi_wallet_config.wallet_type = "email".to_string();
        config.ssi_wallet_config.wallet_name = "TestWallet".to_string();
        config.ssi_wallet_config.wallet_email = "test@example.com".to_string();
        config.ssi_wallet_config.wallet_password = "testpassword".to_string();

        config
    }

    //Tests
 
    #[tokio::test]
    async fn test_manager_new_initializes_correctly() {
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use std::sync::Arc;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo.clone(), config.clone());

        // Verifica que los campos se inicializan correctamente
        let session = manager.wallet_session.lock().await;
        assert!(session.account_id.is_none());
        assert!(session.token.is_none());
        assert!(session.token_exp.is_none());
        assert!(session.wallets.is_empty());

        let key_data = manager.key_data.lock().await;
        assert!(key_data.is_empty());

        assert_eq!(manager.wallet_onboard, false);
    }
}