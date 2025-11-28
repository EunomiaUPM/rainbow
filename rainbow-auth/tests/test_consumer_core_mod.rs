// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\core\mod.rs'

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;

    #[derive(Clone)]
    struct MockRepo;

    impl AuthRepoFactoryTrait for MockRepo {
        
        fn request(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthRequestRepoTrait> {
            todo!()
        }
        
        fn interaction(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthInteractionRepoTrait> {
            todo!()
        }
        
        fn verification(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthVerificationRepoTrait> {
            todo!()
        }
        
        fn token_requirements(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthTokenRequirementsRepoTrait> {
            todo!()
        }
        
        fn mates(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::MatesRepoTrait> {
            todo!()
        }
        
        fn authority(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthorityRequestRepoTrait> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_manager_new_success() {
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;

        let config = ApplicationConsumerConfig::default();
        let repo = Arc::new(MockRepo);

        let manager = Manager::new(repo.clone(), config.clone());

        assert!(!manager.wallet_onboard);
        assert_eq!(manager.config.ssi_wallet_config.wallet_api_url, config.ssi_wallet_config.wallet_api_url);
    }

    #[tokio::test]
    async fn test_manager_request_with_invalid_url_should_fail() {
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = "::invalid-url".to_string();

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        // Intentar hacer una solicitud con el cliente
        let result = manager
            .client
            .get(&manager.config.ssi_wallet_config.wallet_api_url)
            .send()
            .await;

        assert!(result.is_err(), "Expected error due to invalid URL, but got Ok");
    }
}