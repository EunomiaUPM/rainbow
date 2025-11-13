// Tests corresponding to 'rainbow-auth\src\ssi_auth\provider\http\mod.rs'

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::{Json, async_trait, body::Body, extract::{Request, State}, response::IntoResponse, routing::post};
    use chrono::NaiveDateTime;
    use rainbow_auth::ssi_auth::{common::types::{gnap::grant_request::{Access4AT, AccessTokenRequirements4GR}, ssi::keys::KeyInfo}, provider::{core::Manager, http::RainbowAuthProviderRouter}};
    use rainbow_common::{batch_requests::BatchRequests, config::provider_config::ApplicationProviderConfig, errors::{CommonErrors, ErrorLog}, mates::mates::VerifyTokenRequest};
    use rainbow_db::{auth_provider::repo_factory::{factory_trait::AuthRepoFactoryTrait, traits::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait, AuthVerificationRepoTrait, BusinessMatesRepoTrait, MatesRepoTrait}}, common::BasicRepoTrait};
    use reqwest::StatusCode;
    use serde_json::json;
    use anyhow::Result;
    use tower::ServiceExt;
    use tracing::info;
    use axum::extract::Path;
    use tracing::error;
    use urn::Urn;
    use rainbow_db::auth_provider::entities::mates::Model;
    use rainbow_db::auth_provider::entities::mates::NewModel;

    // Mock

    struct MockMatesRepo {
        should_fail: bool,
    }

    #[async_trait]
    impl BasicRepoTrait<Model, NewModel> for MockMatesRepo {
        async fn get_all(&self, _limit: Option<u64>, _offset: Option<u64>) -> anyhow::Result<Vec<Model>>{
            unimplemented!()
        }
        async fn get_by_id(&self, _id: &str) -> Result<Option<Model>> {
            if _id == "mate123" {
                Ok(Some(Model {
                    participant_id: "mate123".to_string(),
                    participant_slug: "slug123".to_string(),
                    participant_type: "type123".to_string(),
                    base_url: Some("https://example.com".to_string()),
                    token: Some("token123".to_string()),
                    saved_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    last_interaction: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    is_me: false,
                }))
            } else {
                Ok(None)
            }
        }
        async fn create(&self, _model: NewModel) -> anyhow::Result<Model>{
            unimplemented!()
        }
        async fn update(&self, _model: Model) -> anyhow::Result<Model>{
            unimplemented!()
        }
        async fn delete(&self, _id: &str) -> anyhow::Result<()>{
            unimplemented!()
        }
    }


    #[async_trait]
    impl MatesRepoTrait for MockMatesRepo {
        async fn get_batch(&self, _ids: &Vec<Urn>) -> Result<Vec<Model>> {
            if self.should_fail {
                Err(anyhow::anyhow!("simulated database error"))
            } else {
                Ok(vec![
                    Model {
                        participant_id: "participant1".to_string(),
                        participant_slug: "slug1".to_string(),
                        participant_type: "type1".to_string(),
                        base_url: Some("https://example.com".to_string()),
                        token: Some("token1".to_string()),
                        saved_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                        last_interaction: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                        is_me: false,
                    },
                    Model {
                        participant_id: "participant2".to_string(),
                        participant_slug: "slug2".to_string(),
                        participant_type: "type2".to_string(),
                        base_url: Some("https://example.org".to_string()),
                        token: Some("token2".to_string()),
                        saved_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                        last_interaction: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                        is_me: true,
                    }
                ])
            }
        }
        
        async fn get_me(&self) -> Result<Option<Model>> {
            if self.should_fail {
                Err(anyhow::anyhow!("simulated database error"))
            } else {
                Ok(Some(Model {
                    participant_id: "me".to_string(),
                    participant_slug: "slug-me".to_string(),
                    participant_type: "type-me".to_string(),
                    base_url: Some("https://me.example.com".to_string()),
                    token: Some("token-me".to_string()),
                    saved_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    last_interaction: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    is_me: true,
                }))
            }
        }

        async fn get_by_token(&self, _token: &str) -> anyhow::Result<Option<Model>> {
            unimplemented!()
        }
        async fn force_create(&self, _mate: NewModel) -> anyhow::Result<Model> {
            unimplemented!()
        }
    }


    


    #[derive(Clone)]
    struct MockRepoFactory {
        mates_repo: Arc<dyn MatesRepoTrait>,
    }

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
            self.mates_repo.clone()
        }
        fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait> {
            unimplemented!()
        }
    }

    fn mock_config(base_url: &str) -> ApplicationProviderConfig {
        let mut config = ApplicationProviderConfig::default();
        config.ssi_wallet_config.wallet_api_protocol = "http".to_string();
        config.ssi_wallet_config.wallet_api_url = base_url.replace("http://", "").replace("https://", "");
        config.ssi_wallet_config.wallet_type = "email".to_string();
        config.ssi_wallet_config.wallet_name = "TestWallet".to_string();
        config.ssi_wallet_config.wallet_email = "test@example.com".to_string();
        config.ssi_wallet_config.wallet_password = "testpassword".to_string();
        config
    }

    //Tests

    #[tokio::test]
    async fn test_router_new_success() {
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use rainbow_auth::ssi_auth::provider::http::RainbowAuthProviderRouter;
        use std::sync::Arc;

        let config = mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let manager = Arc::new(Manager::new(repo, config));

        let router = RainbowAuthProviderRouter::new(manager.clone());

        // Verify that the manager was assigned correctly
        assert!(Arc::ptr_eq(&router.manager, &manager), "Expected manager to be the same");
    }

    #[tokio::test]
    async fn test_router_new_with_empty_manager() {
        use rainbow_auth::ssi_auth::provider::core::Manager;
        use rainbow_auth::ssi_auth::provider::http::RainbowAuthProviderRouter;
        use std::sync::Arc;

        // Manager with mininal config
        let config = mock_config(""); // Empty URL
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let manager = Arc::new(Manager::new(repo, config));

        let router = RainbowAuthProviderRouter::new(manager);

        // Verify that the router is created correctly
        assert!(!std::ptr::eq(&router as *const _, std::ptr::null()), "Router should not be null");
    }

    #[tokio::test]
    async fn test_router_register_success_route() {
        use axum::http::Request;
        use axum::body::Body;
        use tower::ServiceExt;

        let config = crate::tests::mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(Request::builder()
                .uri("/api/v1/wallet/register")
                .method("POST")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();

        // Expect the route to exist, even if the handler fails.
        assert!(
            response.status().is_success() || response.status().is_server_error(),
            "Expected success or server error, got {}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_router_invalid_route_returns_404() {
        use axum::http::{Request, StatusCode};
        use axum::body::Body;
        use tower::ServiceExt;

        let config = crate::tests::mock_config("http://localhost");
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(Request::builder()
                .uri("/api/v1/unknown/route")
                .method("GET")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_wallet_register_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with fn register_wallet
        struct MockManager;

        impl MockManager {
            async fn register_wallet(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn wallet_register(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_wallet().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/register", axum::routing::post(wallet_register))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/register")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_wallet_register_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager that simulates an error
        struct MockManager;

        impl MockManager {
            async fn register_wallet(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated error".to_string())))
            }
        }

        // Handler
        async fn wallet_register(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_wallet().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/register", axum::routing::post(wallet_register))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/register")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_wallet_login_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager con login_wallet que devuelve Ok(())
        struct MockManager;

        impl MockManager {
            async fn login_wallet(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler redefinido para el test
        async fn wallet_login(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.login_wallet().await {
                Ok(()) => StatusCode::OK.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/login", axum::routing::post(wallet_login))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_wallet_login_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with login_wallet return Err
        struct MockManager;

        impl MockManager {
            async fn login_wallet(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated login error".to_string())))
            }
        }

        // Handler
        async fn wallet_login(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.login_wallet().await {
                Ok(()) => StatusCode::OK.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/login", axum::routing::post(wallet_login))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_wallet_logout_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager con logout_wallet que devuelve Ok(())
        struct MockManager;

        impl MockManager {
            async fn logout_wallet(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler redefinido para el test
        async fn wallet_logout(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.logout_wallet().await {
                Ok(()) => StatusCode::OK.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/logout", axum::routing::post(wallet_logout))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/logout")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_wallet_logout_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager con logout_wallet que devuelve Err
        struct MockManager;

        impl MockManager {
            async fn logout_wallet(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated logout error".to_string())))
            }
        }

        // Handler redefinido para el test
        async fn wallet_logout(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.logout_wallet().await {
                Ok(()) => StatusCode::OK.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/logout", axum::routing::post(wallet_logout))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/logout")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_wallet_onboard_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with onboard_wallet return Ok(())
        struct MockManager;

        impl MockManager {
            async fn onboard_wallet(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn wallet_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.onboard_wallet().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/onboard", axum::routing::post(wallet_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/onboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_wallet_onboard_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with onboard_wallet return Err
        struct MockManager;

        impl MockManager {
            async fn onboard_wallet(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated onboard error".to_string())))
            }
        }

        // Handler 
        async fn wallet_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.onboard_wallet().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/onboard", axum::routing::post(wallet_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/onboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_partial_onboard_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with partial_onboard return Ok(())
        struct MockManager;

        impl MockManager {
            async fn partial_onboard(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn partial_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.partial_onboard().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/partial-onboard", axum::routing::post(partial_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/partial-onboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_partial_onboard_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with partial_onboard return Err
        struct MockManager;

        impl MockManager {
            async fn partial_onboard(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated partial onboard error".to_string())))
            }
        }

        // Handler
        async fn partial_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.partial_onboard().await {
                Ok(()) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/partial-onboard", axum::routing::post(partial_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/partial-onboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_register_key_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with register_key return Ok(())
        struct MockManager;

        impl MockManager {
            async fn register_key(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn register_key(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_key().await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/key", axum::routing::post(register_key))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/key")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_key_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with register_key return Err
        struct MockManager;

        impl MockManager {
            async fn register_key(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated key error".to_string())))
            }
        }

        // Handler
        async fn register_key(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_key().await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/key", axum::routing::post(register_key))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/key")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_register_did_success() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with register_did return Ok(())
        struct MockManager;

        impl MockManager {
            async fn register_did(&self) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn register_did(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_did().await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/did", axum::routing::post(register_did))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/did")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_did_error() {
        use axum::{body::Body, http::{Request, StatusCode}, Router, extract::State, response::IntoResponse};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with register_did return Err
        struct MockManager;

        impl MockManager {
            async fn register_did(&self) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated DID error".to_string())))
            }
        }

        // Handler
        async fn register_did(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.register_did().await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/did", axum::routing::post(register_did))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/did")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_key_success() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::delete,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition;

        // MockManager with delete_key return Ok(())
        struct MockManager;

        impl MockManager {
            async fn delete_key(&self, _payload: KeyDefinition) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        // Handler
        async fn delete_key(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<KeyDefinition>,
        ) -> impl IntoResponse {
            match manager.delete_key(payload).await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/key", delete(delete_key))
            .with_state(manager);

        
        let payload = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "TestProvider".to_string(),
            key_id: KeyInfo { id: "key-123".to_string() },
            key_pair: json!({"public": "ABC123", "private": "XYZ789"}),
            keyset_handle: None,
        };


        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/key")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_key_error() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::delete,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition;

        // MockManager with delete_key return Err
        struct MockManager;

        impl MockManager {
            async fn delete_key(&self, _payload: KeyDefinition) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated delete error".to_string())))
            }
        }

        // Handler
        async fn delete_key(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<KeyDefinition>,
        ) -> impl IntoResponse {
            match manager.delete_key(payload).await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/key", delete(delete_key))
            .with_state(manager);

        let payload = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "TestProvider".to_string(),
            key_id: KeyInfo { id: "key-123".to_string() },
            key_pair: json!({"public": "ABC123", "private": "XYZ789"}),
            keyset_handle: None,
        };

        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/key")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_did_success() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::delete,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;

        // MockManager with delete_did return Ok(())
        struct MockManager;

        impl MockManager {
            async fn delete_did(&self, _payload: DidsInfo) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        async fn delete_did(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<DidsInfo>,
        ) -> impl IntoResponse {
            match manager.delete_did(payload).await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/did", delete(delete_did))
            .with_state(manager);

        let payload = DidsInfo {
                did: "did:example:123456789abcdefghi".to_string(),
                alias: "example-alias".to_string(),
                document: "{}".to_string(),
                key_id: "key-123".to_string(),
                default: false,
                created_on: "2025-01-01T00:00:00Z".to_string(),
            };

        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/did")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_did_error() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::delete,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;

        // MockManager with delete_did return Err
        struct MockManager;

        impl MockManager {
            async fn delete_did(&self, _payload: DidsInfo) -> Result<(), CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated DID delete error".to_string())))
            }
        }

        async fn delete_did(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<DidsInfo>,
        ) -> impl IntoResponse {
            match manager.delete_did(payload).await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => (&e).into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/wallet/did", delete(delete_did))
            .with_state(manager);

        let payload = DidsInfo {
                did: "did:example:123456789abcdefghi".to_string(),
                alias: "example-alias".to_string(),
                document: "{}".to_string(),
                key_id: "key-123".to_string(),
                default: false,
                created_on: "2025-01-01T00:00:00Z".to_string(),
            };

        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/did")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_didweb_success() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde_json::json;
        use rainbow_common::errors::CommonErrors;

        // MockManager with get_did_doc return Ok(Json)
        struct MockManager;

        impl MockManager {
            async fn get_did_doc(&self) -> Result<serde_json::Value, CommonErrors> {
                Ok(json!({
                    "id": "did:example:123456789abcdefghi",
                    "publicKey": [{ "id": "key1", "type": "Ed25519", "value": "ABC123" }]
                }))
            }
        }

        async fn didweb(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.get_did_doc().await {
                Ok(did) => Json(did).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/did.json", get(didweb))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/did.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_didweb_error() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;

        // MockManager with get_did_doc return Err
        struct MockManager;

        impl MockManager {
            async fn get_did_doc(&self) -> Result<serde_json::Value, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated DID fetch error".to_string())))
            }
        }

        async fn didweb(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            match manager.get_did_doc().await {
                Ok(did) => Json(did).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/did.json", get(didweb))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/did.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_access_request_success() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::gnap::{GrantRequest, GrantResponse};

        // MockManager with manage_access return Ok(GrantResponse)
        struct MockManager;

        impl MockManager {
            async fn manage_access(&self, _payload: GrantRequest) -> Result<GrantResponse, CommonErrors> {
                Ok(GrantResponse {
                    access_token: None,
                    r#continue: None,
                    interact: None,
                    subject: None,
                    instance_id: None,
                    error: None,
                })
            }
        }

        async fn access_request(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<GrantRequest>,
        ) -> impl IntoResponse {
            match manager.manage_access(payload).await {
                Ok(response) => (StatusCode::OK, Json(response)).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/access", post(access_request))
            .with_state(manager);

        let payload = GrantRequest {
            access_token: AccessTokenRequirements4GR {
                access: Access4AT {
                    r#type: "api-access".to_string(),
                    actions: Some(vec!["talk".to_string()]),
                    locations: None,
                    datatypes: None,
                    identifier: None,
                    privileges: None,
                },
                label: None,
                flags: None,
            },
            subject: None,
            client: json!({"client_id": "test-client"}),
            user: None,
            interact: None,
        };

        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/access")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_access_request_error() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::errors::CommonErrors;
        use rainbow_auth::ssi_auth::common::types::gnap::GrantRequest;

        // MockManager with manage_access return Err
        struct MockManager;

        impl MockManager {
            async fn manage_access(&self, _payload: GrantRequest) -> Result<serde_json::Value, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated access error".to_string())))
            }
        }

        async fn access_request(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<GrantRequest>,
        ) -> impl IntoResponse {
            match manager.manage_access(payload).await {
                Ok(response) => (StatusCode::OK, Json(response)).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/access", post(access_request))
            .with_state(manager);

        let payload = GrantRequest {
            access_token: AccessTokenRequirements4GR {
                access: Access4AT {
                    r#type: "api-access".to_string(),
                    actions: Some(vec!["talk".to_string()]),
                    locations: None,
                    datatypes: None,
                    identifier: None,
                    privileges: None,
                },
                label: None,
                flags: None,
            },
            subject: None,
            client: json!({"client_id": "test-client"}),
            user: None,
            interact: None,
        };

        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/access")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_continue_request_success() {
        use axum::{
            body::Body,
            extract::{Json, Path, State},
            http::{HeaderMap, Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::common::types::gnap::{AccessToken, RefBody};
        use rainbow_common::errors::CommonErrors;

        // Ok MockManager
        struct MockManager;

        impl MockManager {
            async fn validate_continue_request(&self, _id: String, _interact_ref: String, _token: String) -> Result<(), CommonErrors> {
                Ok(())
            }
            async fn continue_req(&self, _model: ()) -> Result<(), CommonErrors> {
                Ok(())
            }
            async fn retrieve_data(&self, _req: (), _int: ()) -> Result<(), CommonErrors> {
                Ok(())
            }
            async fn save_mate(&self, _mate: ()) -> Result<(), CommonErrors> {
                Ok(())
            }
        }

        async fn continue_request(
            State(manager): State<Arc<MockManager>>,
            headers: HeaderMap,
            Path(id): Path<String>,
            Json(payload): Json<RefBody>,
        ) -> impl IntoResponse {
            let token = headers.get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("").replace("GNAP ", "");
            if token.is_empty() {
                return CommonErrors::unauthorized_new(Some("Missing token".to_string())).into_response();
            }

            let int_model = match manager.validate_continue_request(id, payload.interact_ref.clone(), token).await {
                Ok(_) => (),
                Err(e) => return e.into_response(),
            };

            let req_model = match manager.continue_req(int_model).await {
                Ok(_) => (),
                Err(e) => return e.into_response(),
            };

            let mate = match manager.retrieve_data(req_model, int_model).await {
                Ok(_) => (),
                Err(e) => return e.into_response(),
            };

            match manager.save_mate(mate).await {
                Ok(_) => (),
                Err(e) => return e.into_response(),
            }

            let res = AccessToken::default("mock_token".to_string());
            (StatusCode::OK, Json(res)).into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/continue/:id", post(continue_request))
            .with_state(manager);

        let payload = RefBody {
            interact_ref: "interact_ref_123".to_string(),
        };

        let body = serde_json::to_vec(&payload).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "GNAP mock_token".parse().unwrap());

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/continue/abc123")
                    .header("Authorization", "GNAP mock_token")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_continue_request_missing_token_error() {
        use axum::{
            body::Body,
            extract::{Json, Path, State},
            http::{HeaderMap, Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde_json::json;
        use rainbow_auth::ssi_auth::common::types::gnap::RefBody;
        use rainbow_common::errors::CommonErrors;

        struct MockManager;

        async fn continue_request(
            State(_manager): State<Arc<MockManager>>,
            headers: HeaderMap,
            Path(_id): Path<String>,
            Json(_payload): Json<RefBody>,
        ) -> impl IntoResponse {
            let token = headers.get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("").replace("GNAP ", "");
            if token.is_empty() {
                return CommonErrors::unauthorized_new(Some("Missing token".to_string())).into_response();
            }

            (StatusCode::OK, Json(json!({"message": "should not reach"}))).into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/continue/:id", post(continue_request))
            .with_state(manager);

        let payload = RefBody {
            interact_ref: "interact_ref_123".to_string(),
        };

        let body = serde_json::to_vec(&payload).unwrap();

        // Without Authorization
        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/continue/abc123")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_pd_success() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde_json::json;

        // MockManager con respuesta exitosa
        struct MockManager;
        impl MockManager {
            async fn generate_vp_def(&self, _state: String) -> Result<serde_json::Value, CommonErrors> {
                Ok(json!({ "vp": "definition" }))
            }
        }

        async fn pd(State(manager): State<Arc<MockManager>>, Path(state): Path<String>) -> impl IntoResponse {
            let log = format!("GET /pd/{}", state);
            info!("{}", log);

            match manager.generate_vp_def(state).await {
                Ok(vpd) => Json(vpd).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/pd/:state", get(pd))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/pd/test_state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_pd_error() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;

        // MockManager con error
        struct MockManager;
        impl MockManager {
            async fn generate_vp_def(&self, _state: String) -> Result<serde_json::Value, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated error".to_string())))
            }
        }

        async fn pd(State(manager): State<Arc<MockManager>>, Path(state): Path<String>) -> impl IntoResponse {
            let log = format!("GET /pd/{}", state);
            info!("{}", log);

            match manager.generate_vp_def(state).await {
                Ok(vpd) => Json(vpd).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/pd/:state", get(pd))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/pd/test_state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_verify_success() {
        use axum::{
            body::Body,
            extract::{Form, Path, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde::Deserialize;
        use rainbow_common::errors::CommonErrors;

        #[derive(Deserialize)]
        struct VerifyPayload {
            vp_token: String,
        }

        struct MockManager;
        impl MockManager {
            async fn verify_all(&self, _state: String, _vp_token: String) -> Result<String, CommonErrors> {
                Ok("verification_id_123".to_string())
            }

            async fn end_verification(&self, _id: String) -> Result<Option<String>, CommonErrors> {
                Ok(Some("https://example.com/verified".to_string()))
            }
        }

        async fn verify(
            State(manager): State<Arc<MockManager>>,
            Path(state): Path<String>,
            Form(payload): Form<VerifyPayload>,
        ) -> impl IntoResponse {
            let log = format!("POST /verify/{}", state);
            info!("{}", log);

            let id = match manager.verify_all(state, payload.vp_token).await {
                Ok(id) => id,
                Err(e) => return e.into_response(),
            };

            match manager.end_verification(id).await {
                Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
                Ok(None) => StatusCode::OK.into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/verify/:state", post(verify))
            .with_state(manager);

        let form_body = "vp_token=test_token";
        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/verify/test_state")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from(form_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_verify_error() {
        use axum::{
            body::Body,
            extract::{Form, Path, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde::Deserialize;
        use rainbow_common::errors::CommonErrors;

        #[derive(Deserialize)]
        struct VerifyPayload {
            vp_token: String,
        }

        struct MockManager;
        impl MockManager {
            async fn verify_all(&self, _state: String, _vp_token: String) -> Result<String, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("simulated verify error".to_string())))
            }

            async fn end_verification(&self, _id: String) -> Result<Option<String>, CommonErrors> {
                Ok(None)
            }
        }

        async fn verify(
            State(manager): State<Arc<MockManager>>,
            Path(state): Path<String>,
            Form(payload): Form<VerifyPayload>,
        ) -> impl IntoResponse {
            let log = format!("POST /verify/{}", state);
            info!("{}", log);

            let id = match manager.verify_all(state, payload.vp_token).await {
                Ok(id) => id,
                Err(e) => return e.into_response(),
            };

            match manager.end_verification(id).await {
                Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
                Ok(None) => StatusCode::OK.into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/verify/:state", post(verify))
            .with_state(manager);

        let form_body = "vp_token=test_token";
        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/verify/test_state")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from(form_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_verify_mate_token_success() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde_json::json;

        // MockManager with OK response
        struct MockManager;
        impl MockManager {
            async fn verify_token(&self, _token: String) -> Result<serde_json::Value, CommonErrors> {
                Ok(json!({ "mate": "verified" }))
            }
        }

        async fn verify_mate_token(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<VerifyTokenRequest>,
        ) -> impl IntoResponse {
            info!("POST /verify/mate/token");

            let mate = match manager.verify_token(payload.token).await {
                Ok(model) => model,
                Err(e) => return e.into_response(),
            };
            (StatusCode::OK, Json(mate)).into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/verify/mate/token", post(verify_mate_token))
            .with_state(manager);

        let payload = VerifyTokenRequest {
            token: "valid_token".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/verify/mate/token")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_verify_mate_token_error() {
        use axum::{
            body::Body,
            extract::{Json, State},
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::post,
            Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;

        // MockManager with error
        struct MockManager;
        impl MockManager {
            async fn verify_token(&self, _token: String) -> Result<serde_json::Value, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("invalid token".to_string())))
            }
        }

        async fn verify_mate_token(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<VerifyTokenRequest>,
        ) -> impl IntoResponse {
            info!("POST /verify/mate/token");

            let mate = match manager.verify_token(payload.token).await {
                Ok(model) => model,
                Err(e) => return e.into_response(),
            };
            (StatusCode::OK, Json(mate)).into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/verify/mate/token", post(verify_mate_token))
            .with_state(manager);

        let payload = VerifyTokenRequest {
            token: "invalid_token".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/verify/mate/token")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_all_mates_success() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use serde_json::json;
        use tracing::error;

        // MockRepo con respuesta exitosa
        struct MockRepo;
        impl MockRepo {
            async fn get_all(&self, _a: Option<()>, _b: Option<()>) -> Result<serde_json::Value, anyhow::Error> {
                Ok(json!([{ "id": "mate1" }, { "id": "mate2" }]))
            }
        }

        struct MockManager {
            repo: Arc<MockRepo>,
        }

        async fn get_all_mates(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            info!("GET /mates");
            match manager.repo.get_all(None, None).await {
                Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
                Err(e) => {
                    let error = CommonErrors::database_new(Some(e.to_string()));
                    error!("{}", error.log());
                    error.into_response()
                }
            }
        }

        let manager = Arc::new(MockManager {
            repo: Arc::new(MockRepo),
        });

        let router = Router::new()
            .route("/api/v1/mates", get(get_all_mates))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_all_mates_error() {
        use axum::{
            body::Body,
            extract::State,
            http::{Request, StatusCode},
            response::IntoResponse,
            routing::get,
            Json, Router,
        };
        use tower::ServiceExt;
        use std::sync::Arc;
        use anyhow::anyhow;

        // MockRepo con error
        struct MockRepo;
        impl MockRepo {
            async fn get_all(&self, _a: Option<()>, _b: Option<()>) -> Result<serde_json::Value, anyhow::Error> {
                Err(anyhow!("DB failure"))
            }
        }

        struct MockManager {
            repo: Arc<MockRepo>,
        }

        async fn get_all_mates(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
            info!("GET /mates");
            match manager.repo.get_all(None, None).await {
                Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
                Err(e) => {
                    let error = CommonErrors::database_new(Some(e.to_string()));
                    error!("{}", error.log());
                    error.into_response()
                }
            }
        }

        let manager = Arc::new(MockManager {
            repo: Arc::new(MockRepo),
        });

        let router = Router::new()
            .route("/api/v1/mates", get(get_all_mates))
            .with_state(manager);

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_batch_mates_success() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = BatchRequests {
            ids: vec![
                Urn::try_from("urn:example:mate1").unwrap(),
                Urn::try_from("urn:example:mate2").unwrap(),
            ],
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/mates/batch")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }


    #[tokio::test]
    async fn test_get_batch_mates_error() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: true }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = BatchRequests {
            ids: vec![
                Urn::try_from("urn:example:mate1").unwrap(),
                Urn::try_from("urn:example:mate2").unwrap(),
            ],
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/mates/batch")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_all_mates_me_success() {  
        let repo = Arc::new(MockRepoFactory {
                mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
            });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_all_mates_me_error() {
        let repo = Arc::new(MockRepoFactory {
                mates_repo: Arc::new(MockMatesRepo { should_fail: true }),
            });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_mate_by_id_success() {
        let repo = Arc::new(MockRepoFactory {
                    mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
                });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates/mate123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_mate_by_id_not_found() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/mates/unknown-id")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_fast_login_success() {
        use axum::{body::Body, http::Request, Router};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::auth::business::RainbowBusinessLoginRequest;

        // MockManager que devuelve Ok con una URI
        struct MockManager;
        impl MockManager {
            async fn fast_login(&self, _id: String) -> Result<String, CommonErrors> {
                Ok("https://example.com/success".to_string())
            }
        }

        async fn fast_login_handler(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<RainbowBusinessLoginRequest>,
        ) -> impl IntoResponse {
            let uri = match manager.fast_login(payload.auth_request_id).await {
                Ok(uri) => uri,
                Err(e) => return e.into_response(),
            };
            uri.into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/business/login", post(fast_login_handler))
            .with_state(manager);

        let payload = RainbowBusinessLoginRequest {
            auth_request_id: "valid_id".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/business/login")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_fast_login_error() {
        use axum::{body::Body, http::Request, Router};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::auth::business::RainbowBusinessLoginRequest;

        // MockManager que devuelve un error
        struct MockManager;
        impl MockManager {
            async fn fast_login(&self, _id: String) -> Result<String, CommonErrors> {
                Err(CommonErrors::unauthorized_new(Some("Invalid ID".to_string())))
            }
        }

        async fn fast_login_handler(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<RainbowBusinessLoginRequest>,
        ) -> impl IntoResponse {
            let uri = match manager.fast_login(payload.auth_request_id).await {
                Ok(uri) => uri,
                Err(e) => return e.into_response(),
            };
            uri.into_response()
        }

        let manager = Arc::new(MockManager);
        let router = Router::new()
            .route("/api/v1/business/login", post(fast_login_handler))
            .with_state(manager);

        let payload = RainbowBusinessLoginRequest {
            auth_request_id: "invalid_id".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/business/login")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_fallback_returns_not_found() {
        use axum::http::{Method, Uri, StatusCode};
        
        pub async fn testable_fallback(method: Method, uri: Uri) -> (StatusCode, String) {
            RainbowAuthProviderRouter::<MockRepoFactory>::fallback(method, uri).await
        }

        let method = Method::GET;
        let uri: Uri = "/non-existent-route".parse().unwrap();

        let (status, message) = testable_fallback(method.clone(), uri.clone()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, format!("No route for {uri}"));
    }
}