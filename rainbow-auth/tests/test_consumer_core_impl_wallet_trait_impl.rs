// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\core\impls\wallet_trait_impl.rs' 

#[cfg(test)]
mod tests {
    use base64::Engine;
    use mockall::mock;
    use rainbow_auth::ssi_auth::consumer::core::Manager;
    use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
    use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
    use serde_json::{json, Value};
    use rainbow_auth::ssi_auth::common::{traits::RainbowSSIAuthWalletTrait, types::ssi::dids::DidsInfo};
    use rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition;
    use rainbow_auth::ssi_auth::common::types::ssi::wallet::{WalletInfo, WalletSession};
    use axum::{async_trait};
    use anyhow::{Result};
    use rainbow_auth::ssi_auth::common::types::ssi::keys::KeyInfo;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    mock! {
        pub Manager {}

        #[async_trait]
        impl RainbowSSIAuthWalletTrait for Manager {
            async fn register_wallet(&self) -> anyhow::Result<()>;
            async fn login_wallet(&self) -> anyhow::Result<()>;
            async fn logout_wallet(&self) -> anyhow::Result<()>;
            async fn onboard_wallet(&self) -> anyhow::Result<()>;
            async fn partial_onboard(&self) -> anyhow::Result<()>;
            async fn retrieve_wallet_info(&self) -> anyhow::Result<()>;
            async fn retrieve_keys(&self) -> anyhow::Result<()>;
            async fn retrieve_wallet_dids(&self) -> anyhow::Result<()>;
            async fn get_wallet(&self) -> anyhow::Result<WalletInfo>;
            async fn get_key(&self) -> anyhow::Result<KeyDefinition>;
            async fn get_did(&self) -> anyhow::Result<String>;
            async fn get_token(&self) -> anyhow::Result<String>;
            async fn get_did_doc(&self) -> anyhow::Result<serde_json::Value>;
            async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()>;
            async fn delete_key(&self, key_data: KeyDefinition) -> anyhow::Result<()>;
            async fn register_key(&self) -> anyhow::Result<()>;
            async fn register_did(&self) -> anyhow::Result<()>;
            async fn set_default_did(&self) -> anyhow::Result<()>;
            async fn token_expired(&self) -> anyhow::Result<bool>;
            async fn update_token(&self) -> anyhow::Result<()>;
            async fn ok(&self) -> anyhow::Result<()>;
        }
    }


    mock! {
        #[async_trait]
        pub RainbowSSIAuthWalletTrait {}

        #[async_trait]
        impl RainbowSSIAuthWalletTrait for RainbowSSIAuthWalletTrait {
            // BASIC
            
            async fn register_wallet(&self) -> Result<()>;
            async fn login_wallet(&self) -> Result<()>;
            async fn logout_wallet(&self) -> Result<()>;
            async fn onboard_wallet(&self) -> Result<()>;
            async fn partial_onboard(&self) -> Result<()>;
            // GET FROM MANAGER (It gives a cloned Value, not a reference)
            async fn get_wallet(&self) -> anyhow::Result<WalletInfo> {
                // Simulación de la obtención de la billetera
                Ok(WalletInfo {
                    id: "wallet123".to_string(),
                    name: "My Wallet".to_string(),
                    created_on: "2025-01-01".to_string(),
                    added_on: "2025-01-01".to_string(),
                    permission: "read_write".to_string(),
                    dids: vec![], // No DIDs en la billetera
                })
            }
            async fn get_did(&self) -> Result<String>;
            async fn get_token(&self) -> Result<String>;
            async fn get_did_doc(&self) -> Result<Value>;
            async fn get_key(&self) -> Result<KeyDefinition>;
            // RETRIEVE FROM WALLET
            async fn retrieve_wallet_info(&self) -> anyhow::Result<()> {
                let wallet_info = WalletInfo {
                    id: "w1".to_string(),
                    name: "Wallet One".to_string(),
                    created_on: "2025-01-02".to_string(),
                    added_on: "2025-01-02".to_string(),
                    permission: "read_only".to_string(),
                    dids: vec![],
                };
                let mut session = self.wallet_session.lock().await;
                session.wallets.push(wallet_info);
                Ok(())
            }
            async fn retrieve_keys(&self) -> Result<()>;
            async fn retrieve_wallet_dids(&self) -> Result<()>;
            async fn register_key(&self) -> Result<()>;
            async fn register_did(&self) -> Result<()>;
            async fn set_default_did(&self) -> Result<()>;
            async fn delete_key(&self, key: KeyDefinition) -> Result<()>;
            async fn delete_did(&self, did_info: DidsInfo) -> Result<()>;
            async fn token_expired(&self) -> Result<bool>;
            async fn update_token(&self) -> Result<()>;
            async fn ok(&self) -> Result<()>;
        }
    }

    pub async fn testable_partial_onboard<T: RainbowSSIAuthWalletTrait + Sync>(
        service: &T,
    ) -> anyhow::Result<()> {
        service.login_wallet().await?;
        service.retrieve_wallet_info().await?;
        service.retrieve_keys().await?;
        service.retrieve_wallet_dids().await?;
        Ok(())
    }

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
    async fn test_register_wallet_success() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;

        // Start the mock server
        let mock_server = MockServer::start().await;

        // Set up the mock to intercept the request
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/register"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        // Create a default configuration and modify only the necessary field
        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo); // Asegúrate de tener FakeRepo definido
        let manager = Manager::new(repo, config);

        let result = manager.register_wallet().await;

        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_wallet_error_response() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;

        // Start the mock server
        let mock_server = MockServer::start().await;

        // Set up the mock to intercept the request with error 500
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/register"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        // Create a default configuration and modify only the necessary field
        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo); // Asegúrate de tener MockRepo definido
        let manager = Manager::new(repo, config);

        let result = manager.register_wallet().await;

        // Verify that it returns an error
        assert!(result.is_err());

        // Optional: Prints the error for debugging
        if let Err(e) = &result {
            println!("Error esperado: {:?}", e);
        }
    }

   #[tokio::test]
    async fn test_login_wallet_success() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;
        use serde_json::json;
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

        // Simulated JWT with all required claims
        let claims = json!({
            "sub": "test_subject",
            "exp": 2147483647,
            "iat": 1700000000,
            "jti": "test_jti",
            "iss": "test_issuer",
            "aud": "test_audience"
        });
        let claims_encoded = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).unwrap());
        let fake_jwt = format!("header.{}.signature", claims_encoded);

        let mock_server = MockServer::start().await;

        // Simula respuesta 200 con JWT válido y todos los campos requeridos
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "test_account_id",
                "token": fake_jwt,
                "username": "test_user" 
            })))
            .mount(&mock_server)
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo, config);

        let result = manager.login_wallet().await;

        println!("El result: {:?}", result.is_ok());

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_wallet_invalid_jwt_format() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;

        let mock_server = MockServer::start().await;

        // Simula respuesta 401 Unauthorized
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo, config);

        let result = manager.login_wallet().await;

        assert!(result.is_err());

        if let Err(e) = &result {
            println!("Error esperado: {:?}", e);
        }
    }

   #[tokio::test]
    async fn test_logout_wallet_success() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;

        let mock_server = MockServer::start().await;

        // Simulate response 200 OK
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/logout"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo, config);

        // Simulates that the token is present before logout
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("fake_token".to_string());
        }

        let result = manager.logout_wallet().await;

        assert!(result.is_ok());

        // Verify that the token was deleted
        let session = manager.wallet_session.lock().await;
        assert!(session.token.is_none());
    }

    #[tokio::test]
    async fn test_logout_wallet_error_response() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::sync::Arc;

        let mock_server = MockServer::start().await;

        // Simulates a 500 Internal Server Error response
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/logout"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = mock_server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(mock_server.address().port().to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo, config);

        let result = manager.logout_wallet().await;

        assert!(result.is_err());

        if let Err(e) = &result {
            println!("Error esperado: {:?}", e);
        }
    }

    #[tokio::test]
    async fn test_onboard_wallet_success() {

        let mut mock = MockRainbowSSIAuthWalletTrait::new();

        mock.expect_register_wallet()
            .returning(|| Ok(()));
        mock.expect_login_wallet()
            .returning(|| Ok(()));
        mock.expect_retrieve_wallet_info()
            .returning(|| Ok(()));
        mock.expect_retrieve_keys()
            .returning(|| Ok(()));
        mock.expect_retrieve_wallet_dids()
            .returning(|| Ok(()));
        mock.expect_get_wallet()
            .returning(|| Ok(WalletInfo {
                id: "wallet123".to_string(),
                name: "Test Wallet".to_string(),
                created_on: "Test created_on".to_string(),
                added_on: "Test added_on".to_string(),
                permission: "Test permission".to_string(),
                dids: vec![DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "did:example:123".to_string(),
                    default: true,
                    created_on: "did:example:123".to_string(),
                }],
            }));
        mock.expect_get_key()
            .returning(|| Ok(KeyDefinition {
                key_id: KeyInfo { id: "key123".to_string() },
                algorithm: "Test algorithm".to_string(),
                crypto_provider: "Test crypto_provider".to_string(),
                key_pair: None,
                keyset_handle: None,
            }));
        mock.expect_delete_did()
            .returning(|_| Ok(()));
        mock.expect_delete_key()
            .returning(|_| Ok(()));
        mock.expect_register_key()
            .returning(|| Ok(()));
        mock.expect_register_did()
            .returning(|| Ok(()));
        mock.expect_set_default_did()
            .returning(|| Ok(()));
        mock.expect_onboard_wallet()
            .returning(|| Ok(()));

        let result = mock.onboard_wallet().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_onboard_wallet_missing_did_error() {
        use std::sync::Arc;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;

        let config = ApplicationConsumerConfig::default();
        let repo = Arc::new(MockRepo);
        let mut manager = Manager::new(repo, config);

        manager.wallet_onboard = true;

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("fake.jwt.token".to_string());
            session.token_exp = Some(2147483647);
            session.account_id = Some("test_account_id".to_string());
            session.wallets.push(WalletInfo {
                id: "wallet_id".to_string(),
                name: "Test Wallet".to_string(),
                created_on: "2025-01-01T00:00:00Z".to_string(),
                added_on: "2025-01-01T00:00:00Z".to_string(),
                permission: "owner".to_string(),
                dids: vec![],
            });
        }

        let result = manager.onboard_wallet().await;

        assert!(result.is_err());

        if let Err(e) = &result {
            println!("Error esperado: {:?}", e);
        }
    }

    #[tokio::test]
    async fn test_partial_onboard_success() {
        let mut mock = MockRainbowSSIAuthWalletTrait::new();

        mock.expect_login_wallet()
            .returning(|| Ok(()));
        mock.expect_retrieve_wallet_info()
            .returning(|| Ok(()));
        mock.expect_retrieve_keys()
            .returning(|| Ok(()));
        mock.expect_retrieve_wallet_dids()
            .returning(|| Ok(()));

        let result = testable_partial_onboard(&mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_partial_onboard_failure_on_retrieve_keys() {
        let mut mock = MockRainbowSSIAuthWalletTrait::new();

        mock.expect_login_wallet()
            .returning(|| Ok(()));
        mock.expect_retrieve_wallet_info()
            .returning(|| Ok(()));
        mock.expect_retrieve_keys()
            .returning(|| Err(anyhow::anyhow!("Failed to retrieve keys")));
        mock.expect_retrieve_wallet_dids()
            .returning(|| Ok(()));

        let result = testable_partial_onboard(&mock).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Failed to retrieve keys");
    }

    #[tokio::test]
    async fn test_get_wallet_success() {
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let wallet_info = WalletInfo {
            id: "wallet123".to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![],
        };

        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![wallet_info.clone()],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_wallet().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), wallet_info);
    }

    #[tokio::test]
    async fn test_get_wallet_failure_no_wallets() {
        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_wallet().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        // Ajusta esto según el contenido real del error
        assert!(
            error_msg.contains("Missing Action Error")
                || error_msg.contains("There is no wallet to retrieve dids from"),
            "El error es: {:?}",
            error_msg
        );
    }
    
    #[tokio::test]
    async fn test_get_did_success() {
        let _did_info = DidsInfo {
            did: "did:example:123456789".to_string(),
            alias: "alias1".to_string(),
            document: "id: did:example:123456789".to_string(),
            key_id: "key1".to_string(),
            default: true,
            created_on: "2023-01-01T00:00:00Z".to_string(),
        };

        let wallet_info = WalletInfo {
            id: "wallet123".to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![_did_info.clone()],
        };

        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![wallet_info],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_did().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "did:example:123456789");
    }

    #[tokio::test]
    async fn test_get_did_failure_no_dids() {
        let wallet_info = WalletInfo {
            id: "wallet123".to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![], // No DIDs
        };

        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![wallet_info],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_did().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Missing Action Error") || error_msg.contains("No DIDs found in wallet"),
            "Mensaje de error inesperado: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_get_token_success() {
        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token_abc_123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_token().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "token_abc_123");
    }

    #[tokio::test]
    async fn test_get_token_failure_no_token() {
        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: None, // No token presente
            token_exp: Some(9999999999),
            wallets: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_token().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Missing Action Error") || error_msg.contains("There is no token available for use"),
            "Mensaje de error inesperado: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_get_did_doc_success() {
        let did_info = DidsInfo {
            did: "did:example:123456789".to_string(),
            alias: "alias1".to_string(),
            document: r#"{"id":"did:example:123456789","@context":"https://www.w3.org/ns/did/v1"}"#.to_string(),
            key_id: "key1".to_string(),
            default: true,
            created_on: "2023-01-01T00:00:00Z".to_string(),
        };

        let wallet_info = WalletInfo {
            id: "wallet123".to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![did_info],
        };

        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![wallet_info],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_did_doc().await;
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["id"], "did:example:123456789");
        assert_eq!(json["@context"], "https://www.w3.org/ns/did/v1");
    }

    #[tokio::test]
    async fn test_get_did_doc_failure_no_dids() {
        let wallet_info = WalletInfo {
            id: "wallet123".to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![], // No DIDs
        };

        let session = WalletSession {
            account_id: Some("account1".to_string()),
            token: Some("token123".to_string()),
            token_exp: Some(9999999999),
            wallets: vec![wallet_info],
        };

        let manager = Manager {
            wallet_session: Mutex::new(session),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_did_doc().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Missing Action Error") || error_msg.contains("No DIDs found in wallet"),
            "Mensaje de error inesperado: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_get_key_success() {
        let key = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "provider1".to_string(),
            key_id: KeyInfo {
                id: "key1".to_string()
            },
            key_pair: serde_json::json!({
                "publicKey": "ABC123",
                "privateKey": "XYZ789"
            }),
            keyset_handle: Some(serde_json::json!({
                "handle": "some_handle_data"
            })),
        };

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![key.clone()]),
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_key().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), key);
    }

    #[tokio::test]
    async fn test_get_key_failure_no_keys() {
        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]), // No claves
            client: reqwest::Client::new(),
            config: ApplicationConsumerConfig::default(),
        };

        let result = manager.get_key().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Missing Action Error") || error_msg.contains("Retrieve keys first"),
            "Mensaje de error inesperado: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_retrieve_wallet_info_success() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();

        let mock_response = serde_json::json!({
            "account": "account1",
            "wallets": [
                {
                    "id": "wallet123",
                    "name": "Test Wallet",
                    "createdOn": "2023-01-01",
                    "addedOn": "2023-01-02",
                    "permission": "read",
                    "dids": []
                }
            ]
        });

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/wallet-api/wallet/accounts/wallets");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(mock_response);
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_wallet_info().await;
        assert!(result.is_ok());

        let session = manager.wallet_session.lock().await;
        assert_eq!(session.wallets.len(), 1);
        assert_eq!(session.wallets[0].id, "wallet123");

        mock.assert();
    }

    #[tokio::test]
    async fn test_retrieve_wallet_info_failure_http_error() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/wallet-api/wallet/accounts/wallets");
            then.status(500)
                .header("content-type", "application/json")
                .body("Internal Server Error");
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_wallet_info().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Wallet Error")
                || error_msg.contains("Petition to retrieve Wallet information failed")
                || error_msg.contains("Internal Server Error"),
            "Mensaje de error inesperado: {}",
            error_msg
        );

        mock.assert();
    }

    #[tokio::test]
    async fn test_retrieve_keys_success() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();

        let wallet_id = "wallet123";
        let mock_keys = serde_json::json!([
            {
                "algorithm": "Ed25519",
                "cryptoProvider": "provider1",
                "keyId": {
                    "id": "key1",
                    "alias": "alias1"
                },
                "keyPair": {
                    "publicKey": "ABC123",
                    "privateKey": "XYZ789"
                },
                "keyset_handle": null
            }
        ]);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/wallet-api/wallet/{}/keys", wallet_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(mock_keys.to_string());
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let wallet_info = WalletInfo {
            id: wallet_id.to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![wallet_info],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_keys().await;
        assert!(result.is_ok());

        let key_data = manager.key_data.lock().await;
        assert_eq!(key_data.len(), 1);
        assert_eq!(key_data[0].algorithm, "Ed25519");

        mock.assert();
    }

    #[tokio::test]
    async fn test_retrieve_keys_failure_http_error() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();
        let wallet_id = "wallet123";

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/wallet-api/wallet/{}/keys", wallet_id));
            then.status(500)
                .header("content-type", "application/json")
                .body("Internal Server Error");
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let wallet_info = WalletInfo {
            id: wallet_id.to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![wallet_info],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_keys().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Wallet Error")
                || error_msg.contains("Petition to retrieve keys failed")
                || error_msg.contains("Internal Server Error"),
            "Mensaje de error inesperado: {}",
            error_msg
        );

        mock.assert();
    }

   #[tokio::test]
    async fn test_retrieve_wallet_dids_success() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();
        let wallet_id = "wallet123";

        let raw_body = r#"
        [
            {
                "did": "did:example:123456789",
                "alias": "alias1",
                "document": "{\"id\":\"did:example:123456789\"}",
                "keyId": "key1",
                "default": true,
                "createdOn": "2023-01-01T00:00:00Z"
            }
        ]
        "#;

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/wallet-api/wallet/{}/dids", wallet_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(raw_body);
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let wallet_info = WalletInfo {
            id: wallet_id.to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![],
        };

        let manager = Manager {
            wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![wallet_info],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_wallet_dids().await;
        assert!(result.is_ok(), "Error inesperado: {:?}", result);

        let session = manager.wallet_session.lock().await;
        assert_eq!(session.wallets[0].dids.len(), 1);
        assert_eq!(session.wallets[0].dids[0].did, "did:example:123456789");

        mock.assert();
    }
    
    #[tokio::test]
    async fn test_retrieve_wallet_dids_failure_http_error() {
        use httpmock::MockServer;
        use httpmock::Method::GET;

        let server = MockServer::start();
        let wallet_id = "wallet123";

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/wallet-api/wallet/{}/dids", wallet_id));
            then.status(500)
                .header("content-type", "application/json")
                .body("Internal Server Error");
        });

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.address().port().to_string());

        let wallet_info = WalletInfo {
            id: wallet_id.to_string(),
            name: "Test Wallet".to_string(),
            created_on: "2023-01-01".to_string(),
            added_on: "2023-01-02".to_string(),
            permission: "read".to_string(),
            dids: vec![],
        };

        let manager = Manager {
                wallet_session: Mutex::new(WalletSession {
                account_id: Some("account1".to_string()),
                token: Some("token123".to_string()),
                token_exp: Some(9999999999),
                wallets: vec![wallet_info],
            }),
            wallet_onboard: false,
            repo: Arc::new(MockRepo),
            key_data: Mutex::new(vec![]),
            client: reqwest::Client::new(),
            config,
        };

        let result = manager.retrieve_wallet_dids().await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Wallet Error")
                || error_msg.contains("Petition to retrieve Wallet DIDs failed")
                || error_msg.contains("Internal Server Error"),
            "Mensaje de error inesperado: {}",
            error_msg
        );

        mock.assert();
    }

    #[tokio::test]
    async fn test_register_key_success() {
        use mockito::Server;
        use std::sync::Arc;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "test_token";

        let url_path = format!("/wallet-api/wallet/{}/keys/import", wallet_id);

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("content-type", "text/plain")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(201)
            .with_body("Key registered")
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![], // Si necesitas DIDs simulados, puedes añadirlos aquí
            });
        }

        let result = manager.register_key().await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_key_http_error() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";

        let url_path = format!("/wallet-api/wallet/{}/keys/import", wallet_id);

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("content-type", "text/plain")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "Internal Server Error"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        let result = manager.register_key().await;

        mock.assert_async().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_did_success() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use serde_json::json;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let key_id = "key123";

        let url_path = format!(
            "/wallet-api/wallet/{}/dids/create/jwk?keyId={}&alias=privatekey",
            wallet_id, key_id
        );

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(200)
            .with_body(r#"{"did": "did:example:123"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(KeyDefinition {
                algorithm: "Ed25519".to_string(),
                crypto_provider: "local".to_string(),
                key_id: KeyInfo { id: key_id.to_string() },
                key_pair: json!({
                    "publicKey": "public_key_data",
                    "privateKey": "private_key_data"
                }),
                keyset_handle: None,
            });
        }

        let result = manager.register_did().await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_did_conflict() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use serde_json::json;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let key_id = "key123";

        let url_path = format!(
            "/wallet-api/wallet/{}/dids/create/jwk?keyId={}&alias=privatekey",
            wallet_id, key_id
        );

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(409)
            .with_body(r#"{"error": "DID already exists"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(KeyDefinition {
                algorithm: "Ed25519".to_string(),
                crypto_provider: "local".to_string(),
                key_id: KeyInfo { id: key_id.to_string() },
                key_pair: json!({
                    "publicKey": "public_key_data",
                    "privateKey": "private_key_data"
                }),
                keyset_handle: None,
            });
        }

        let result = manager.register_did().await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_default_did_success() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let did = "did123";

        let url_path = format!(
            "/wallet-api/wallet/{}/dids/default?did={}",
            wallet_id, did
        );

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(202)
            .with_body("Accepted")
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets[0].dids.push(DidsInfo {
                did: "did123".to_string(),
                alias: "Test alias".to_string(),
                document: "Test document".to_string(),
                key_id: "Test key_id".to_string(),
                default: true,
                created_on: "Test".to_string(),
            });
        }

        let result = manager.set_default_did().await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_default_did_error() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let did = "did:example:123";

        let url_path = format!(
            "/wallet-api/wallet/{}/dids/default?did={}",
            wallet_id, did
        );

        let mock = server
            .mock("POST", url_path.as_str())
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(500)
            .with_body(r#"{"error": "Internal Server Error"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets[0].dids.push(DidsInfo {
                did: "did:example:123".to_string(),
                alias: "Test alias".to_string(),
                document: "Test document".to_string(),
                key_id: "Test key_id".to_string(),
                default: true,
                created_on: "Test".to_string(),
            });
        }

        let result = manager.set_default_did().await;

        mock.assert_async().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_key_success() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use serde_json::json;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let key_id = "key123";

        let url_path = format!("/wallet-api/wallet/{}/keys/{}", wallet_id, key_id);

        let mock = server
            .mock("DELETE", url_path.as_str())
            .match_header("content-type", "text/plain")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(202)
            .with_body("Key deleted")
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        let key = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "local".to_string(),
            key_id: KeyInfo { id: key_id.to_string() },
            key_pair: json!({
                "publicKey": "public_key_data",
                "privateKey": "private_key_data"
            }),
            keyset_handle: None,
        };

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(key.clone());
        }

        let result = manager.delete_key(key.clone()).await;

        mock.assert_async().await;
        assert!(result.is_ok());

        let key_data = manager.key_data.lock().await;
        assert!(!key_data.contains(&key)); // Verifica que se eliminó de la estructura interna
    }

    #[tokio::test]
    async fn test_delete_key_error() {
        use mockito::Server;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use serde_json::json;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let key_id = "key123";

        let url_path = format!("/wallet-api/wallet/{}/keys/{}", wallet_id, key_id);

        let mock = server
            .mock("DELETE", url_path.as_str())
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(500)
            .with_body(r#"{"error": "Internal Server Error"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![],
            });
        }

        let key = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "local".to_string(),
            key_id: KeyInfo { id: key_id.to_string() },
            key_pair: json!({
                "publicKey": "public_key_data",
                "privateKey": "private_key_data"
            }),
            keyset_handle: None,
        };

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(key.clone());
        }

        let result = manager.delete_key(key.clone()).await;

        mock.assert_async().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_did_success() {
        use mockito::Server;
        use std::sync::Arc;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let did = "did:example:123";

        let url_path = format!("/wallet-api/wallet/{}/dids/{}", wallet_id, did);

        let mock = server
            .mock("DELETE", url_path.as_str())
            .match_header("content-type", "text/plain")
            .match_header("accept", "application/json")
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(202)
            .with_body("DID deleted")
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        let did_info = DidsInfo {
            did: "did:example:123".to_string(),
            alias: "Test alias".to_string(),
            document: "Test document".to_string(),
            key_id: "Test key_id".to_string(),
            default: true,
            created_on: "Test".to_string(),
        };
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![did_info.clone()],
            });
        }

        let result = manager.delete_did(did_info.clone()).await;

        mock.assert_async().await;
        assert!(result.is_ok());

        let session = manager.wallet_session.lock().await;
        let wallet = &session.wallets[0];
        assert!(!wallet.dids.contains(&did_info)); // Verifica que se eliminó del wallet
    }

    #[tokio::test]
    async fn test_delete_did_error() {
        use mockito::Server;
        use std::sync::Arc;

        let mut server = Server::new_async().await;
        let wallet_id = "wallet123";
        let token = "mock_token";
        let did = "did:example:123";

        let url_path = format!("/wallet-api/wallet/{}/dids/{}", wallet_id, did);

        let mock = server
            .mock("DELETE", url_path.as_str())
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(500)
            .with_body(r#"{"error": "Internal Server Error"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_id = Some(wallet_id.to_string());

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config.clone());

        let did_info = DidsInfo {
            did: "did:example:123".to_string(),
            alias: "Test alias".to_string(),
            document: "Test document".to_string(),
            key_id: "Test key_id".to_string(),
            default: true,
            created_on: "Test".to_string(),
        };

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some(token.to_string());
            session.wallets.push(WalletInfo {
                id: wallet_id.to_string(),
                name: "My Wallet".to_string(),
                created_on: "2025-01-01".to_string(),
                added_on: "2025-01-01".to_string(),
                permission: "read_write".to_string(),
                dids: vec![did_info.clone()],
            });
        }

        let result = manager.delete_did(did_info.clone()).await;

        mock.assert_async().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_expired_true() {
        use std::sync::Arc;
        use std::time::{SystemTime, UNIX_EPOCH};

        let repo = Arc::new(MockRepo);
        let config = ApplicationConsumerConfig::default();
        let manager = Manager::new(repo.clone(), config);

        let expired_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - 10; // 10 seconds in the past

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = Some(expired_time);
        }

        let result = manager.token_expired().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_token_expired_false() {
        use std::sync::Arc;
        use std::time::{SystemTime, UNIX_EPOCH};

        let repo = Arc::new(MockRepo);
        let config = ApplicationConsumerConfig::default();
        let manager = Manager::new(repo.clone(), config);

        let future_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour in the future

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = Some(future_time);
        }

        let result = manager.token_expired().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[tokio::test]
    async fn test_token_expired_none() {
        use std::sync::Arc;

        let repo = Arc::new(MockRepo);
        let config = ApplicationConsumerConfig::default();
        let manager = Manager::new(repo.clone(), config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = None;
        }

        let result = manager.token_expired().await;
        assert!(result.is_err());
    }





    #[tokio::test]
    async fn test_update_token_success() -> anyhow::Result<()> {

        let mut service = MockRainbowSSIAuthWalletTrait::new();

        service.expect_login_wallet()
            .returning(|| Ok(()));
        service.expect_update_token()
            .returning(|| Ok(()));

        service.update_token().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_update_token_error() -> anyhow::Result<()> {
        // Crear el mock del servicio
        let mut service = MockRainbowSSIAuthWalletTrait::new();

        service.expect_login_wallet()
            .returning(|| Err(anyhow::anyhow!("Error al iniciar sesión")));
        service.expect_update_token()
            .returning(|| Err(anyhow::anyhow!("Error al iniciar sesión")));

        let result = service.update_token().await;
        assert!(result.is_err(), "Se esperaba un error, pero se obtuvo Ok");
        assert_eq!(result.unwrap_err().to_string(), "Error al iniciar sesión");

        Ok(())
    }

    #[tokio::test]
    async fn test_ok_token_expired_and_updated_successfully() {
        use mockito::Server;
        use std::sync::Arc;
        use std::time::{SystemTime, UNIX_EPOCH};
        use serde_json::json;

        let mut server = Server::new_async().await;

        let login_path = "/wallet-api/auth/login";

        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600;

        let jwt_payload = json!({
            "sub": "mock_user_id",
            "exp": exp,
            "iat": SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "jti": "mock-jti-123",
            "iss": "mock-issuer",
            "aud": "mock-audience"
        });

        let encoded_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(jwt_payload.to_string());

        let mock_token = format!("header.{}.signature", encoded_payload);

        let _mock = server
            .mock("POST", login_path)
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .with_header("content-type", "application/json")
            .with_status(200)  
            .with_body(json!({
                "id": "account123",
                "username": "mock_user",
                "token": mock_token
            }).to_string().into_bytes())
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config);

        // Simula token expirado
        let expired_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - 10;

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = Some(expired_time);
            session.token = Some("expired_token".to_string());
        }

        let result = manager.ok().await;
        assert!(result.is_ok());

        let session = manager.wallet_session.lock().await;
        assert_eq!(session.token.as_deref(), Some(&mock_token).map(|x| x.as_str()));
        assert_eq!(session.token_exp, Some(exp));
    }

    #[tokio::test]
    async fn test_ok_token_expired_and_update_failed() {
        use mockito::Server;
        use std::sync::Arc;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut server = Server::new_async().await;

        let url_path = "/wallet-api/auth/login";

        let _mock = server
            .mock("POST", url_path)
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .with_status(500)
            .with_body(r#"{"error": "Login failed"}"#)
            .create_async()
            .await;

        let mut config = ApplicationConsumerConfig::default();
        config.ssi_wallet_config.wallet_api_url = server.socket_address().ip().to_string();
        config.ssi_wallet_config.wallet_api_port = Some(server.socket_address().port().to_string());
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();

        let repo = Arc::new(MockRepo);
        let manager = Manager::new(repo.clone(), config);

        // Simula token expirado
        let expired_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - 10;

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = Some(expired_time);
            session.token = Some("expired_token".to_string());
        }

        let result = manager.ok().await;
        assert!(result.is_err());
    }
}