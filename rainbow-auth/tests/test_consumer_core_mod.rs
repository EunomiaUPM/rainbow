// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\core\mod.rs'

#[cfg(test)]
mod tests {
<<<<<<< HEAD
    use std::sync::Arc;

=======
>>>>>>> origin/main
    use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
    use std::sync::Arc;

    #[derive(Clone)]
    struct MockRepo;

    impl AuthRepoFactoryTrait for MockRepo {
<<<<<<< HEAD
        fn request(
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthRequestRepoTrait>
        {
            todo!()
        }

        fn interaction(
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthInteractionRepoTrait>
        {
            todo!()
        }

        fn verification(
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthVerificationRepoTrait>
        {
=======
        fn request(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthRequestRepoTrait> {
            todo!()
        }

        fn interaction(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthInteractionRepoTrait> {
            todo!()
        }

        fn verification(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthVerificationRepoTrait> {
>>>>>>> origin/main
            todo!()
        }

        fn token_requirements(
<<<<<<< HEAD
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthTokenRequirementsRepoTrait>
        {
            todo!()
        }

        fn mates(
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::MatesRepoTrait> {
            todo!()
        }

        fn authority(
            &self
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthorityRequestRepoTrait>
        {
=======
            &self,
        ) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthTokenRequirementsRepoTrait> {
            todo!()
        }

        fn mates(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::MatesRepoTrait> {
            todo!()
        }

        fn authority(&self) -> Arc<dyn rainbow_db::auth_consumer::repo_factory::traits::AuthorityRequestRepoTrait> {
>>>>>>> origin/main
            todo!()
        }
    }

    #[tokio::test]
    async fn test_manager_new_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use std::sync::Arc;

        let config = ApplicationConsumerConfig::default();
        let repo = Arc::new(MockRepo);

        let manager = Manager::new(repo.clone(), config.clone());

        assert!(!manager.wallet_onboard);
        assert_eq!(
            manager.config.ssi_wallet_config.wallet_api_url,
            config.ssi_wallet_config.wallet_api_url
        );
    }

    #[tokio::test]
    async fn test_manager_request_with_invalid_url_should_fail() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use std::sync::Arc;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = "::invalid-url".to_string();

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        // Intentar hacer una solicitud con el cliente
<<<<<<< HEAD
        let result =
            manager.client.get(&manager.config.ssi_wallet_config.wallet_api_url).send().await;
=======
        let result = manager.client.get(&manager.config.ssi_wallet_config.wallet_api_url).send().await;
>>>>>>> origin/main

        assert!(
            result.is_err(),
            "Expected error due to invalid URL, but got Ok"
        );
    }
}
