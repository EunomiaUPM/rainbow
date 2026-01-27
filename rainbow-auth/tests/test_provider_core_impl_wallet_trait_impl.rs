// Tests corresponding to
// 'rainbow-auth\src\ssi_auth\provider\core\impls\wallet_trait_impl.rs'

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use rainbow_auth::ssi_auth::common::traits::RainbowSSIAuthWalletTrait;
    use rainbow_auth::ssi_auth::common::types::oidc::{InputDescriptor, Vpd};
    use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;
    use rainbow_auth::ssi_auth::provider::core::Manager;
    use rainbow_common::config::provider_config::ApplicationProviderConfig;
    use rainbow_db::auth_provider::entities::business_mates::{Model, NewModel};
<<<<<<< HEAD
    use rainbow_db::auth_provider::entities::mates::{
        Model as MatesModel, NewModel as MatesNewModel
    };
    use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
    use rainbow_db::auth_provider::repo_factory::traits::{
        AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait,
        AuthVerificationRepoTrait, BusinessMatesRepoTrait, MatesRepoTrait
    };
    use rainbow_db::common::BasicRepoTrait;
=======
    use rainbow_db::auth_provider::entities::mates::{Model as MatesModel, NewModel as MatesNewModel};
    use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
    use rainbow_db::auth_provider::repo_factory::traits::{
        AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait, AuthVerificationRepoTrait,
        BusinessMatesRepoTrait, MatesRepoTrait,
    };
    use rainbow_db::common::BasicRepoTrait;
    use std::sync::Arc;
>>>>>>> origin/main
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Mocks
    pub struct MockMatesRepo;

    #[async_trait]
    impl BasicRepoTrait<MatesModel, MatesNewModel> for MockMatesRepo {
        async fn create(&self, item: MatesNewModel) -> anyhow::Result<MatesModel> {
            Ok(MatesModel {
                participant_id: item.participant_id,
                participant_slug: item.participant_slug,
                participant_type: item.participant_type,
                base_url: item.base_url,
                token: item.token,
                saved_at: chrono::Utc::now().naive_utc(),
                last_interaction: chrono::Utc::now().naive_utc(),
                is_me: item.is_me
            })
        }

        async fn get_all(
            &self,
            _limit: Option<u64>,
            _offset: Option<u64>
        ) -> anyhow::Result<Vec<MatesModel>> {
            Ok(vec![])
        }

        async fn get_by_id(&self, _id: &str) -> anyhow::Result<Option<MatesModel>> { Ok(None) }

        async fn update(&self, model: MatesModel) -> anyhow::Result<MatesModel> { Ok(model) }

        async fn delete(&self, _id: &str) -> anyhow::Result<()> { Ok(()) }
    }

    #[async_trait]
    impl MatesRepoTrait for MockMatesRepo {
        async fn get_me(&self) -> anyhow::Result<Option<MatesModel>> { Ok(None) }

        async fn get_by_token(&self, _token: &str) -> anyhow::Result<Option<MatesModel>> {
            Ok(None)
        }

        async fn force_create(&self, mate: MatesNewModel) -> anyhow::Result<MatesModel> {
            self.create(mate).await
        }

        async fn get_batch(&self, _ids: &Vec<String>) -> anyhow::Result<Vec<MatesModel>> {
            Ok(vec![])
        }
    }

    #[derive(Clone)]
    pub struct MockBusinessMatesRepo;

    #[async_trait]
    impl BasicRepoTrait<Model, NewModel> for MockBusinessMatesRepo {
        async fn create(&self, item: NewModel) -> anyhow::Result<Model> {
            Ok(Model {
                id: "mock-id".to_string(),
                participant_id: item.participant_id,
                token: item.token,
                saved_at: chrono::Utc::now().naive_utc(),
                last_interaction: chrono::Utc::now().naive_utc()
            })
        }

        async fn get_all(
            &self,
            _limit: Option<u64>,
            _offset: Option<u64>
        ) -> anyhow::Result<Vec<Model>> {
            Ok(vec![]) // Devuelve una lista vacía
        }

        async fn get_by_id(&self, _id: &str) -> anyhow::Result<Option<Model>> {
            Ok(None) // No encuentra nada
        }

        async fn update(&self, model: Model) -> anyhow::Result<Model> {
            Ok(model) // Devuelve el mismo modelo
        }

        async fn delete(&self, _id: &str) -> anyhow::Result<()> {
            Ok(()) // No hace nada
        }
    }

    #[async_trait]
    impl BusinessMatesRepoTrait for MockBusinessMatesRepo {
        async fn get_by_token(&self, _token: &str) -> anyhow::Result<Option<Model>> { Ok(None) }

        async fn force_create(&self, mate: NewModel) -> anyhow::Result<Model> {
            self.create(mate).await
        }
    }

    #[derive(Clone)]
    pub struct MockRepoFactory;

    impl AuthRepoFactoryTrait for MockRepoFactory {
        fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { unimplemented!() }
        fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { unimplemented!() }
        fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { unimplemented!() }
        fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { unimplemented!() }
        fn mates(&self) -> Arc<dyn MatesRepoTrait> { Arc::new(MockMatesRepo) }
        fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait> {
            Arc::new(MockBusinessMatesRepo)
        }
    }

    pub fn mock_config(base_url: &str) -> ApplicationProviderConfig {
        let mut config = ApplicationProviderConfig::default();

        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_api_url =
            base_url.replace("http://", "").replace("https://", "");
        config.ssi_wallet_config.wallet_api_port = None; // O Some("7001".to_string()) si quieres incluir puerto
        config.ssi_wallet_config.wallet_type = "email".to_string();
        config.ssi_wallet_config.wallet_name = "TestWallet".to_string();
        config.ssi_wallet_config.wallet_email = "test@example.com".to_string();
        config.ssi_wallet_config.wallet_password = "testpassword".to_string();

        config
    }

<<<<<<< HEAD
    // Tests
=======
    //Tests
>>>>>>> origin/main

    #[tokio::test]
    async fn test_register_wallet_success() {
        // Start a mock server
        let mock_server = MockServer::start().await;

        // Simulate a response 201
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/register"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        // Mock configuration
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.register_wallet().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_wallet_error() {
        let mock_server = MockServer::start().await;

        // Simulate a response 500
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/register"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.register_wallet().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_wallet_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        // Crear un JWT válido con campo exp y sub
        let payload = json!({
            "exp": 4723600000u64, // fecha futura
            "sub": "user-123",
            "iat": 1700000000u64,
            "jti": "jwt-id-456",
            "iss": "https://issuer.example.com",
            "aud": "https://audience.example.com"
        });
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap());
        let jwt = format!("header.{}.signature", encoded_payload);

        let response_body = json!({
            "id": "wallet-id-123",
            "username": "testuser",
            "token": jwt
        });

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.login_wallet().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_login_wallet_error() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(401)) // Unauthorized
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.login_wallet().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_wallet_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/logout"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simulate that there is an active token
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("fake-token".to_string());
        }

        let result = manager.logout_wallet().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the token was deleted
        let session = manager.wallet_session.lock().await;
        assert!(
            session.token.is_none(),
            "Expected token to be None after logout"
        );
    }

    #[tokio::test]
    async fn test_logout_wallet_error() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/logout"))
            .respond_with(ResponseTemplate::new(500)) // Error del servidor
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.logout_wallet().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_onboard_wallet_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
<<<<<<< HEAD
=======
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // JWT
        let jwt_payload = json!({
            "exp": 4723600000u64,
            "iat": 1700000000u64,
            "sub": "user-123",
            "jti": "jwt-id-456",
            "iss": "https://issuer.example.com",
            "aud": "https://audience.example.com"
        });
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(&jwt_payload).unwrap());
        let jwt = format!("header.{}.signature", encoded_payload);

        // Mock login
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "wallet-id-123",
                "username": "testuser",
                "token": jwt
            })))
            .mount(&mock_server)
            .await;

        // Mock register_wallet
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/register"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        // Mock retrieve_wallet_info
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/accounts/wallets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "account": "wallet-id-123",
                "wallets": [{
                    "id": "wallet-id-123",
                    "name": "TestWallet",
                    "createdOn": "2023-01-01",
                    "addedOn": "2023-01-01",
                    "permission": "owner",
                    "dids": [{
                        "did": "did:example:123",
                        "document": "{}",
                        "alias": "Test alias",
                        "keyId": "did:example:123",
                        "default": true,
                        "createdOn": "2023-01-01"
                    }]
                }]
            })))
            .mount(&mock_server)
            .await;

        // Mock retrieve_keys with correct KeyDefinition structure
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"
                [
                    {
                        "algorithm": "RSA",
                        "cryptoProvider": "OpenSSL",
                        "keyId": { "id": "key-123" },
                        "keyPair": {},
                        "keyset_handle": null
                    }
                ]
<<<<<<< HEAD
            "#
=======
            "#,
>>>>>>> origin/main
            ))
            .mount(&mock_server)
            .await;

        // Mock retrieve_wallet_dids
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
                "did": "did:example:123",
                "document": "{}",
                "alias": "Test alias",
                "keyId": "key-123",
                "default": true,
                "createdOn": "2023-01-01"
            }])))
            .mount(&mock_server)
            .await;

        // Mock delete_did
        Mock::given(method("DELETE"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/dids/did:example:123",
            ))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        // Mock delete_key
        Mock::given(method("DELETE"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/key-123"))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        // Mock register_key
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/import"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        // Mock register_did
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/create/jwk"))
            .and(query_param("keyId", "key-123"))
            .and(query_param("alias", "privatekey"))
            .respond_with(ResponseTemplate::new(200).set_body_string("did:example:123"))
            .mount(&mock_server)
            .await;

        // Mock set_default_did
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/default"))
            .and(query_param("did", "did:example:123"))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.onboard_wallet().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_onboard_wallet_fails_on_login() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate login failure
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.onboard_wallet().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_partial_onboard_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
<<<<<<< HEAD
=======
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // JWT
        let jwt_payload = json!({
            "exp": 4723600000u64,
            "iat": 1700000000u64,
            "sub": "user-123",
            "jti": "jwt-id-456",
            "iss": "https://issuer.example.com",
            "aud": "https://audience.example.com"
        });
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(&jwt_payload).unwrap());
        let jwt = format!("header.{}.signature", encoded_payload);

        // Mock login
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "wallet-id-123",
                "username": "testuser",
                "token": jwt
            })))
            .mount(&mock_server)
            .await;

        // Mock retrieve_wallet_info
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/accounts/wallets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "account": "wallet-id-123",
                "wallets": [{
                    "id": "wallet-id-123",
                    "name": "TestWallet",
                    "createdOn": "2023-01-01",
                    "addedOn": "2023-01-01",
                    "permission": "owner",
                    "dids": []
                }]
            })))
            .mount(&mock_server)
            .await;

        // Mock retrieve_keys
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"[{
                "algorithm": "RSA",
                "cryptoProvider": "OpenSSL",
                "keyId": { "id": "key-123" },
                "keyPair": {},
                "keyset_handle": null
<<<<<<< HEAD
            }]"#
=======
            }]"#,
>>>>>>> origin/main
            ))
            .mount(&mock_server)
            .await;

        // Mock retrieve_wallet_dids
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.partial_onboard().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_partial_onboard_fails_on_login() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock login with error
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.partial_onboard().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_wallet_success() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // It simulates that there is a wallet in session.
        {
            let mut session = manager.wallet_session.lock().await;
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.get_wallet().await;
        assert!(result.is_ok(), "Expected Ok(WalletInfo), got: {:?}", result);
        let wallet = result.unwrap();
        assert_eq!(wallet.id, "wallet-id-123");
    }

    #[tokio::test]
    async fn test_get_wallet_error_when_empty() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // No wallet is added to the session

        let result = manager.get_wallet().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_did_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simula un wallet con un DID en sesión
        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets.push(WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
        }

        let result = manager.get_did().await;
        assert!(result.is_ok(), "Expected Ok(did), got: {:?}", result);
        assert_eq!(result.unwrap(), "did:example:123");
    }

    #[tokio::test]
    async fn test_get_did_error_when_no_dids() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simula un wallet sin DIDs
        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets.push(WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![] // sin DIDs
            });
        }

        let result = manager.get_did().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_token_success() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // It simulates that there is a token in session.
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token-123".to_string());
        }

        let result = manager.get_token().await;
        assert!(result.is_ok(), "Expected Ok(token), got: {:?}", result);
        assert_eq!(result.unwrap(), "mock-token-123");
    }

    #[tokio::test]
    async fn test_get_token_error_when_none() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // No token is added to the session

        let result = manager.get_token().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_did_doc_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simulate a wallet with a valid DID in session
        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets.push(WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![DidsInfo {
                    did: "did:example:123".to_string(),
                    document: json!({ "id": "did:example:123" }).to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
        }

        let result = manager.get_did_doc().await;
        assert!(result.is_ok(), "Expected Ok(Value), got: {:?}", result);
        let doc = result.unwrap();
        assert_eq!(doc["id"], "did:example:123");
    }

    #[tokio::test]
    async fn test_get_did_doc_error_when_no_dids() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simulate a wallet without DIDs
        {
            let mut session = manager.wallet_session.lock().await;
            session.wallets.push(WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![] // without DIDs
            });
        }

        let result = manager.get_did_doc().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_key_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simula que hay una clave en sesión
        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(KeyDefinition {
                algorithm: "RSA".to_string(),
                crypto_provider: "OpenSSL".to_string(),
                key_id: KeyInfo { id: "key-123".to_string() },
                key_pair: json!({}),
                keyset_handle: None
            });
        }

        let result = manager.get_key().await;
        assert!(
            result.is_ok(),
            "Expected Ok(KeyDefinition), got: {:?}",
            result
        );
        let key = result.unwrap();
        assert_eq!(key.key_id.id, "key-123");
    }

    #[tokio::test]
    async fn test_get_key_error_when_none() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // No se añade ninguna clave

        let result = manager.get_key().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_retrieve_wallet_info_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // It simulates that there is a token in session.
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
        }

        // Mock del endpoint /wallet-api/wallet/accounts/wallets
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/accounts/wallets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "account": "wallet-id-123",
                "wallets": [{
                    "id": "wallet-id-123",
                    "name": "TestWallet",
                    "createdOn": "2023-01-01",
                    "addedOn": "2023-01-01",
                    "permission": "owner",
                    "dids": []
                }]
            })))
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_wallet_info().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the wallet was saved in session
        let session = manager.wallet_session.lock().await;
        assert_eq!(session.wallets.len(), 1);
        assert_eq!(session.wallets[0].id, "wallet-id-123");
    }

    #[tokio::test]
    async fn test_retrieve_wallet_info_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // It simulates that there is a token in session.
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
        }

        // Mock of the endpoint with error
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/accounts/wallets"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_wallet_info().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_retrieve_keys_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock of endpoint /wallet-api/wallet/wallet-id-123/keys
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"
                [
                    {
                        "algorithm": "RSA",
                        "cryptoProvider": "OpenSSL",
                        "keyId": { "id": "key-123" },
                        "keyPair": {},
                        "keyset_handle": null
                    }
                ]
<<<<<<< HEAD
            "#
=======
            "#,
>>>>>>> origin/main
            ))
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_keys().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the key was saved
        let key_data = manager.key_data.lock().await;
        assert_eq!(key_data.len(), 1);
        assert_eq!(key_data[0].key_id.id, "key-123");
    }

    #[tokio::test]
    async fn test_retrieve_keys_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock with error
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_keys().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_retrieve_wallet_dids_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock of endpoint /wallet-api/wallet/wallet-id-123/dids
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "did": "did:example:123",
                    "document": "{}",
                    "alias": "Test alias",
                    "keyId": "key-123",
                    "default": true,
                    "createdOn": "2023-01-01"
                }
            ])))
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_wallet_dids().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the DID was saved
        let session = manager.wallet_session.lock().await;
        assert_eq!(session.wallets[0].dids.len(), 1);
        assert_eq!(session.wallets[0].dids[0].did, "did:example:123");
    }

    #[tokio::test]
    async fn test_retrieve_wallet_dids_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock with error
        Mock::given(method("GET"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let result = manager.retrieve_wallet_dids().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_register_key_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock of endpoint /wallet-api/wallet/wallet-id-123/keys/import
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/import"))
            .respond_with(ResponseTemplate::new(201).set_body_string("Key registered"))
            .mount(&mock_server)
            .await;

        let result = manager.register_key().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_register_key_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token and wallet in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        // Mock with error
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/import"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let result = manager.register_key().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_register_did_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token, wallet, and key in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
        }
        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition {
                algorithm: "RSA".to_string(),
                crypto_provider: "OpenSSL".to_string(),
                key_id: rainbow_auth::ssi_auth::common::types::ssi::keys::KeyInfo {
                    id: "key-123".to_string()
                },
                key_pair: json!({}),
                keyset_handle: None
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
        }
        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(
                rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition {
                    algorithm: "RSA".to_string(),
                    crypto_provider: "OpenSSL".to_string(),
                    key_id: rainbow_auth::ssi_auth::common::types::ssi::keys::KeyInfo { id: "key-123".to_string() },
                    key_pair: json!({}),
                    keyset_handle: None,
                },
            );
>>>>>>> origin/main
        }

        // Mock of endpoint /dids/create/jwk
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/create/jwk"))
            .and(query_param("keyId", "key-123"))
            .and(query_param("alias", "privatekey"))
            .respond_with(ResponseTemplate::new(200).set_body_string("did:example:123"))
            .mount(&mock_server)
            .await;

        let result = manager.register_did().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_register_did_error_response() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate token, wallet, and key in session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
        }
        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition {
                algorithm: "RSA".to_string(),
                crypto_provider: "OpenSSL".to_string(),
                key_id: rainbow_auth::ssi_auth::common::types::ssi::keys::KeyInfo {
                    id: "key-123".to_string()
                },
                key_pair: json!({}),
                keyset_handle: None
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
        }
        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(
                rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition {
                    algorithm: "RSA".to_string(),
                    crypto_provider: "OpenSSL".to_string(),
                    key_id: rainbow_auth::ssi_auth::common::types::ssi::keys::KeyInfo { id: "key-123".to_string() },
                    key_pair: json!({}),
                    keyset_handle: None,
                },
            );
>>>>>>> origin/main
        }

        // Mock with error
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/create/jwk"))
            .and(query_param("keyId", "key-123"))
            .and(query_param("alias", "privatekey"))
            .respond_with(ResponseTemplate::new(500)) // Internal Server Error
            .mount(&mock_server)
            .await;

        let result = manager.register_did().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_set_default_did_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        // Start mock server
        let mock_server = MockServer::start().await;

        // Mock del endpoint /dids/default con respuesta 202
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/default"))
            .and(query_param("did", "did:example:123"))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        // Setup and simulated session
        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.set_default_did().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_set_default_did_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("POST"))
            .and(path("/wallet-api/wallet/wallet-id-123/dids/default"))
            .and(query_param("did", "did:example:123"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.set_default_did().await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_delete_key_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock of the DELETE endpoint with response 202
        Mock::given(method("DELETE"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/key-123"))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simulate session with token, wallet and key
        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let key = KeyDefinition {
            algorithm: "RSA".to_string(),
            crypto_provider: "OpenSSL".to_string(),
            key_id: KeyInfo { id: "key-123".to_string() },
            key_pair: json!({}),
            keyset_handle: None
        };

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(key.clone());
        }

        let result = manager.delete_key(key.clone()).await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the key was removed from the internal state
        let key_data = manager.key_data.lock().await;
        assert!(!key_data.contains(&key), "Expected key to be removed");
    }

    #[tokio::test]
    async fn test_delete_key_error_response() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::common::types::ssi::keys::{KeyDefinition, KeyInfo};
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // DELETE endpoint mock with error 500
        Mock::given(method("DELETE"))
            .and(path("/wallet-api/wallet/wallet-id-123/keys/key-123"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let key = KeyDefinition {
            algorithm: "RSA".to_string(),
            crypto_provider: "OpenSSL".to_string(),
            key_id: KeyInfo { id: "key-123".to_string() },
            key_pair: json!({}),
            keyset_handle: None
        };

        {
            let mut key_data = manager.key_data.lock().await;
            key_data.push(key.clone());
        }

        let result = manager.delete_key(key.clone()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);

        // Verify that the key is still present in the internal state
        let key_data = manager.key_data.lock().await;
        assert!(
            key_data.contains(&key),
            "Expected key to remain after failed deletion"
        );
    }

    #[tokio::test]
    async fn test_delete_did_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock of the DELETE endpoint with response 202
        Mock::given(method("DELETE"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/dids/did:example:123",
            ))
            .respond_with(ResponseTemplate::new(202))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Simulate session with token, wallet and DID
        let did_info = DidsInfo {
            did: "did:example:123".to_string(),
            document: "{}".to_string(),
            alias: "Test alias".to_string(),
            key_id: "key-123".to_string(),
            default: true,
            created_on: "2023-01-01".to_string()
        };

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![did_info.clone()]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![did_info.clone()],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.delete_did(did_info.clone()).await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);

        // Verify that the DID was removed from the internal state
        let session = manager.wallet_session.lock().await;
        assert!(
            !session.wallets[0].dids.contains(&did_info),
            "Expected DID to be removed"
        );
    }

    #[tokio::test]
    async fn test_delete_did_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // DELETE endpoint mock with error 500
        Mock::given(method("DELETE"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/dids/did:example:123",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let did_info = DidsInfo {
            did: "did:example:123".to_string(),
            document: "{}".to_string(),
            alias: "Test alias".to_string(),
            key_id: "key-123".to_string(),
            default: true,
            created_on: "2023-01-01".to_string()
        };

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![did_info.clone()]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![did_info.clone()],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.delete_did(did_info.clone()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);

        // Verify that the DID is still present in the internal state
        let session = manager.wallet_session.lock().await;
        assert!(
            session.wallets[0].dids.contains(&did_info),
            "Expected DID to remain after failed deletion"
        );
    }

    #[tokio::test]
    async fn test_resolve_credential_offer_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response and valid body
        let response_body = json!({
            "credential_issuer": "https://issuer.example.com",
            "grants": {
                "authorization_code": {
                    "issuer_state": "state-123"
                },
                "urn:ietf:params:oauth:grant-type:pre-authorized_code": {
                    "pre-authorized_code": "code-456",
                    "user_pin_required": false
                }
            },
            "credentials": [],
            "credential_configuration_ids": ["config-1", "config-2"]
        });

        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolveCredentialOffer"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolveCredentialOffer",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body.clone()))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.resolve_credential_offer("mock-uri".to_string()).await;
        assert!(
            result.is_ok(),
            "Expected Ok(CredentialOfferResponse), got: {:?}",
            result
        );

        let offer = result.unwrap();
        assert_eq!(offer.credential_issuer, "https://issuer.example.com");
    }

    #[tokio::test]
    async fn test_resolve_credential_offer_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolveCredentialOffer"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolveCredentialOffer",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.resolve_credential_offer("mock-uri".to_string()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_resolve_credential_issuer_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response and valid JSON body
        Mock::given(method("GET"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolveIssuerOpenIDMetadata"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolveIssuerOpenIDMetadata",
>>>>>>> origin/main
            ))
            .and(query_param("issuer", "https://issuer.example.com"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "issuer": "https://issuer.example.com",
                "metadata": {
                    "authorization_endpoint": "https://issuer.example.com/auth",
                    "token_endpoint": "https://issuer.example.com/token"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result =
            manager.resolve_credential_issuer("https://issuer.example.com".to_string()).await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_resolve_credential_issuer_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("GET"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolveIssuerOpenIDMetadata"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolveIssuerOpenIDMetadata",
>>>>>>> origin/main
            ))
            .and(query_param("issuer", "https://issuer.example.com"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result =
            manager.resolve_credential_issuer("https://issuer.example.com".to_string()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_use_offer_req_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response and valid JSON body
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/useOfferRequest",
            ))
            .and(query_param("did", "did:example:123"))
            .and(query_param("requireUserInput", "false"))
            .and(query_param("pinOrTxCode", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "status": "accepted"
            })))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.use_offer_req("mock-uri".to_string(), "123456".to_string()).await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_use_offer_req_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/useOfferRequest",
            ))
            .and(query_param("did", "did:example:123"))
            .and(query_param("requireUserInput", "false"))
            .and(query_param("pinOrTxCode", "123456"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.use_offer_req("mock-uri".to_string(), "123456".to_string()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_join_exchange_success() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response and text body
        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolvePresentationRequest"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolvePresentationRequest",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string("exchange-success"))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.join_exchange("mock-exchange-url".to_string()).await;
        assert!(result.is_ok(), "Expected Ok(String), got: {:?}", result);
        assert_eq!(result.unwrap(), "exchange-success");
    }

    #[tokio::test]
    async fn test_join_exchange_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/resolvePresentationRequest"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/resolvePresentationRequest",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let result = manager.join_exchange("mock-exchange-url".to_string()).await;
        assert!(result.is_err(), "Expected Err, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_vpd_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
        use urlencoding::encode;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Create a valid VPD as a parameter in the URL
        let vpd_json = json!({
            "id": "vpd-id-123",
            "input_descriptors": [
                {
                    "id": "descriptor-1",
                    "schema": "https://schema.org/Person"
                }
            ]
        });

        let vpd_json_string = vpd_json.to_string();
        let vpd_encoded = encode(&vpd_json_string);
        let full_url = format!(
            "https://example.com?presentation_definition={}",
            vpd_encoded
        );

        let result = manager.parse_vpd(full_url).await;
        assert!(result.is_ok(), "Expected Ok(Vpd), got: {:?}", result);

        let parsed = result.unwrap();
        assert_eq!(parsed.id, "vpd-id-123");
        assert_eq!(parsed.input_descriptors.len(), 1);
        assert_eq!(parsed.input_descriptors[0].id, "descriptor-1");
    }

    #[tokio::test]
    async fn test_parse_vpd_invalid_json() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use urlencoding::encode;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        // Malformed JSON in the parameter
        let malformed_json = "not-a-valid-json";
        let encoded_json = encode(malformed_json);
        let full_url = format!(
            "https://example.com?presentation_definition={}",
            encoded_json
        );

        let result = manager.parse_vpd(full_url).await;
        assert!(
            result.is_err(),
            "Expected Err due to invalid JSON, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_get_matching_vcs_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response and a valid VC
<<<<<<< HEAD
        Mock::given(method("POST",),)
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ),)
            .respond_with(ResponseTemplate::new(200,).set_body_json(json!([
=======
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
>>>>>>> origin/main
                {
                    "id": "vc-id-001",
                    "addedOn": "2023-01-01T00:00:00Z",
                    "disclosures": "[]",
                    "document": r#"{
                        "@context": ["https://www.w3.org/2018/credentials/v1"],
                        "type": ["VerifiableCredential"],
                        "issuer": "https://issuer.example.com",
                        "credentialSubject": {
                            "id": "did:example:123",
                            "name": "Alice"
                        }
                    }"#,
                    "format": r#"{
                        "jwt": {
                            "alg": ["ES256"]
                        }
                    }"#,
                    "parsedDocument": r#"{
                        "id": "vc-id-001",
                        "type": ["VerifiableCredential"],
                        "issuer": "https://issuer.example.com"
                    }"#,
                    "pending": false,
                    "wallet": r#"{
                        "id": "wallet-id-123",
                        "name": "TestWallet"
                    }"#
                }
            ]),),)
            .mount(&mock_server,)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let vpd = Vpd {
            id: "vpd-id".to_string(),
            input_descriptors: vec![InputDescriptor {
                id: "descriptor-1".to_string(),
                constraints: None,
                format: None
            }]
        };

        let result = manager.get_matching_vcs(vpd).await;
        assert!(
            result.is_ok(),
            "Expected Ok(Vec<String>), got: {:?}",
            result
        );
        let vcs = result.unwrap();
        assert_eq!(vcs.len(), 1);
        assert_eq!(vcs[0], "vc-id-001");
    }

    #[tokio::test]
    async fn test_get_matching_vcs_no_match() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock endpoint with 200 response but without VC
<<<<<<< HEAD
        Mock::given(method("POST",),)
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ),)
            .respond_with(ResponseTemplate::new(200,).set_body_json(json!([]),),)
            .mount(&mock_server,)
=======
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&mock_server)
>>>>>>> origin/main
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let vpd = Vpd {
            id: "vpd-id".to_string(),
            input_descriptors: vec![InputDescriptor {
                id: "descriptor-1".to_string(),
                constraints: None,
                format: None
            }]
        };

        let result = manager.get_matching_vcs(vpd).await;
        assert!(
            result.is_err(),
            "Expected Err due to no matching VCs, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_match_vc4vp_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock of the endpoint with valid VC
<<<<<<< HEAD
        Mock::given(method("POST",),)
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ),)
            .respond_with(ResponseTemplate::new(200,).set_body_json(json!([
=======
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
>>>>>>> origin/main
                {
                    "id": "vc-id-001",
                    "addedOn": "2023-01-01T00:00:00Z",
                    "disclosures": "[]",
                    "document": "{}",
                    "format": "{}",
                    "parsedDocument": "{}",
                    "pending": false,
                    "wallet": "{}"
                }
            ]),),)
            .mount(&mock_server,)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let vp_def = json!({
            "id": "vp-id-001",
            "input_descriptors": [
                {
                    "id": "descriptor-1",
                    "schema": "https://schema.org/Person"
                }
            ]
        });

        let result = manager.match_vc4vp(vp_def).await;
        assert!(
            result.is_ok(),
            "Expected Ok(Vec<MatchingVCs>), got: {:?}",
            result
        );
        let vcs = result.unwrap();
        assert_eq!(vcs.len(), 1);
        assert_eq!(vcs[0].id, "vc-id-001");
    }

    #[tokio::test]
    async fn test_match_vc4vp_error_response() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
<<<<<<< HEAD
        Mock::given(method("POST",),)
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ),)
            .respond_with(ResponseTemplate::new(500,),)
            .mount(&mock_server,)
=======
        Mock::given(method("POST"))
            .and(path(
                "/wallet-api/wallet/wallet-id-123/exchange/matchCredentialsForPresentationDefinition",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
>>>>>>> origin/main
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![]
            });
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![],
                },
            );
>>>>>>> origin/main
        }

        let vp_def = json!({
            "id": "vp-id-001",
            "input_descriptors": [
                {
                    "id": "descriptor-1",
                    "schema": "https://schema.org/Person"
                }
            ]
        });

        let result = manager.match_vc4vp(vp_def).await;
        assert!(
            result.is_err(),
            "Expected Err due to server error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_present_vp_success() {
<<<<<<< HEAD
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock of the endpoint with a 200 response and a correct `redirectUri` field
        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/usePresentationRequest"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/usePresentationRequest",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "redirectUri": "https://example.com/redirect"
            })))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
        }

        let result =
            manager.present_vp("mock-request".to_string(), vec!["vc-id-001".to_string()]).await;
        assert!(result.is_ok(), "Expected Ok(RedirectResponse), got: {:?}", result);
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
        }

        let result = manager.present_vp("mock-request".to_string(), vec!["vc-id-001".to_string()]).await;
        assert!(
            result.is_ok(),
            "Expected Ok(RedirectResponse), got: {:?}",
            result
        );
>>>>>>> origin/main

        let redirect = result.unwrap();
        assert_eq!(redirect.redirect_uri, "https://example.com/redirect");
    }

    #[tokio::test]
    async fn test_present_vp_error_response() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Endpoint mockup with 500 error
        Mock::given(method("POST"))
            .and(path(
<<<<<<< HEAD
                "/wallet-api/wallet/wallet-id-123/exchange/usePresentationRequest"
=======
                "/wallet-api/wallet/wallet-id-123/exchange/usePresentationRequest",
>>>>>>> origin/main
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token = Some("mock-token".to_string());
<<<<<<< HEAD
            session.wallets.push(rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                id: "wallet-id-123".to_string(),
                name: "TestWallet".to_string(),
                created_on: "2023-01-01".to_string(),
                added_on: "2023-01-01".to_string(),
                permission: "owner".to_string(),
                dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                    did: "did:example:123".to_string(),
                    document: "{}".to_string(),
                    alias: "Test alias".to_string(),
                    key_id: "key-123".to_string(),
                    default: true,
                    created_on: "2023-01-01".to_string()
                }]
            });
        }

        let result =
            manager.present_vp("mock-request".to_string(), vec!["vc-id-001".to_string()]).await;
        assert!(result.is_err(), "Expected Err due to server error, got: {:?}", result);
=======
            session.wallets.push(
                rainbow_auth::ssi_auth::common::types::ssi::wallet::WalletInfo {
                    id: "wallet-id-123".to_string(),
                    name: "TestWallet".to_string(),
                    created_on: "2023-01-01".to_string(),
                    added_on: "2023-01-01".to_string(),
                    permission: "owner".to_string(),
                    dids: vec![rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo {
                        did: "did:example:123".to_string(),
                        document: "{}".to_string(),
                        alias: "Test alias".to_string(),
                        key_id: "key-123".to_string(),
                        default: true,
                        created_on: "2023-01-01".to_string(),
                    }],
                },
            );
        }

        let result = manager.present_vp("mock-request".to_string(), vec!["vc-id-001".to_string()]).await;
        assert!(
            result.is_err(),
            "Expected Err due to server error, got: {:?}",
            result
        );
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_token_expired_true() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            // Simulate an expired token (past date)
            session.token_exp = Some(1000);
        }

        let result = manager.token_expired().await;
        assert!(result.is_ok(), "Expected Ok(bool), got: {:?}", result);
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_token_expired_none() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = None; // No token
        }

        let result = manager.token_expired().await;
        assert!(
            result.is_err(),
            "Expected Err due to missing token, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_update_token_success() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
<<<<<<< HEAD
=======
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Simulate valid JWT
        let payload = json!({
            "exp": 4723600000u64,
            "sub": "user-123",
            "iat": 1700000000u64,
            "jti": "jwt-id-456",
            "iss": "https://issuer.example.com",
            "aud": "https://audience.example.com"
        });
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap());
        let jwt = format!("header.{}.signature", encoded_payload);

        // Mock login with 200 response
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "wallet-id-123",
                "username": "testuser",
                "token": jwt
            })))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.update_token().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_update_token_error() {
<<<<<<< HEAD
=======
        use rainbow_auth::ssi_auth::provider::core::Manager;
>>>>>>> origin/main
        use std::sync::Arc;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        use rainbow_auth::ssi_auth::provider::core::Manager;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Mock login with error 500
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        let result = manager.update_token().await;
        assert!(
            result.is_err(),
            "Expected Err due to login failure, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_ok_token_expired_and_updated() {
<<<<<<< HEAD
        use std::sync::Arc;

=======
>>>>>>> origin/main
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use serde_json::json;
<<<<<<< HEAD
=======
        use std::sync::Arc;
>>>>>>> origin/main
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // JWT
        let payload = json!({
            "exp": 4723600000u64,
            "sub": "user-123",
            "iat": 1700000000u64,
            "jti": "jwt-id-456",
            "iss": "https://issuer.example.com",
            "aud": "https://audience.example.com"
        });
        let encoded_payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap());
        let jwt = format!("header.{}.signature", encoded_payload);

        // Mock login with 200 response
        Mock::given(method("POST"))
            .and(path("/wallet-api/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "wallet-id-123",
                "username": "testuser",
                "token": jwt
            })))
            .mount(&mock_server)
            .await;

        let config = mock_config(&mock_server.uri());
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = Some(1000); // Expired token
        }

        let result = manager.ok().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_ok_token_not_expired() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
<<<<<<< HEAD
            let future_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600; // Add 1 hour
=======
            let future_timestamp =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() + 3600; // Add 1 hour
>>>>>>> origin/main
            session.token_exp = Some(future_timestamp);
        }

        let result = manager.ok().await;
        assert!(result.is_ok(), "Expected Ok(()), got: {:?}", result);
    }

    #[tokio::test]
    async fn test_ok_token_missing() {
        use std::sync::Arc;

        use rainbow_auth::ssi_auth::provider::core::Manager;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory);
        let manager = Manager::new(repo, config);

        {
            let mut session = manager.wallet_session.lock().await;
            session.token_exp = None; // No token
        }

        let result = manager.ok().await;
        assert!(
            result.is_err(),
            "Expected Err due to missing token, got: {:?}",
            result
        );
    }
}
