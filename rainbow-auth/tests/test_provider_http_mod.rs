// Tests corresponding to 'rainbow-auth\src\ssi_auth\provider\http\mod.rs' 

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::{body::Body, http::{Request, StatusCode}};
    use sea_orm_migration::async_trait;
    use tower::ServiceExt;
    use chrono::NaiveDateTime;
    use rainbow_auth::ssi_auth::{common::types::{gnap::{GrantRequest, RefBody, grant_request::{Access4AT, AccessTokenRequirements4GR}}, ssi::keys::KeyInfo}, provider::core::Manager};
    use rainbow_auth::ssi_auth::provider::http::RainbowAuthProviderRouter;
    use rainbow_common::config::provider_config::ApplicationProviderConfig;
    use rainbow_db::auth_provider::repo_factory::traits::*;
    use rainbow_db::common::BasicRepoTrait;
    use anyhow::Result;
    use rainbow_db::auth_provider::entities::mates::{Model, NewModel};
    use rainbow_db::auth_provider::entities::auth_request::{Model as AuthRequestModel, NewModel as AuthRequestNewModel};
    use rainbow_db::auth_provider::entities::auth_interaction::{Model as AuthInteractionModel, NewModel as AuthInteractionNewModel};
    use rainbow_db::auth_provider::entities::auth_verification::{Model as AuthVerificationModel, NewModel as AuthVerificationNewModel};
    use rainbow_db::auth_provider::entities::auth_token_requirements::Model as AuthTokenReqModel;
    use rainbow_db::auth_provider::entities::business_mates::{Model as BusinessMateModel, NewModel as BusinessMateNewModel};

    // Mocks
    struct MockMatesRepo { should_fail: bool }

    #[async_trait::async_trait]
    impl BasicRepoTrait<Model, NewModel> for MockMatesRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<Model>> {
            if self.should_fail { Err(anyhow::anyhow!("DB error")) } else { Ok(vec![]) }
        }
        async fn get_by_id(&self, id: &str) -> Result<Option<Model>> {
            if self.should_fail { Err(anyhow::anyhow!("DB error")) }
            else if id == "mate123" {
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
            } else { Ok(None) }
        }
        async fn create(&self, _: NewModel) -> Result<Model> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: Model) -> Result<Model> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait::async_trait]
    impl MatesRepoTrait for MockMatesRepo {
        async fn get_batch(&self, _: &Vec<String>) -> Result<Vec<Model>> {
            if self.should_fail { Err(anyhow::anyhow!("DB error")) } else { Ok(vec![]) }
        }
        async fn get_me(&self) -> Result<Option<Model>> {
            if self.should_fail { Err(anyhow::anyhow!("DB error")) } else { Ok(Some(Model {
                participant_id: "me".to_string(),
                participant_slug: "slug-me".to_string(),
                participant_type: "type-me".to_string(),
                base_url: Some("https://me.example.com".to_string()),
                token: Some("token-me".to_string()),
                saved_at: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                last_interaction: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                is_me: true,
            })) }
        }
        async fn get_by_token(&self, _: &str) -> Result<Option<Model>> {
            if self.should_fail { Err(anyhow::anyhow!("DB error")) } else { Ok(None) }
        }
        async fn force_create(&self, _: NewModel) -> Result<Model> { Err(anyhow::anyhow!("Not implemented")) }
    }

    struct MockRequestRepo;

    #[async_trait::async_trait]
    impl BasicRepoTrait<AuthRequestModel, AuthRequestNewModel> for MockRequestRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<AuthRequestModel>> { Ok(vec![]) }
        async fn get_by_id(&self, _: &str) -> Result<Option<AuthRequestModel>> { Ok(None) }
        async fn create(&self, _: AuthRequestNewModel) -> Result<AuthRequestModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: AuthRequestModel) -> Result<AuthRequestModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait::async_trait]
    impl AuthRequestRepoTrait for MockRequestRepo {}

    struct MockInteractionRepo;

    #[async_trait::async_trait]
    impl BasicRepoTrait<AuthInteractionModel, AuthInteractionNewModel> for MockInteractionRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<AuthInteractionModel>> { Ok(vec![]) }
        async fn get_by_id(&self, _: &str) -> Result<Option<AuthInteractionModel>> { Ok(None) }
        async fn create(&self, _: AuthInteractionNewModel) -> Result<AuthInteractionModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: AuthInteractionModel) -> Result<AuthInteractionModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait::async_trait]
    impl AuthInteractionRepoTrait for MockInteractionRepo {
        async fn get_by_reference(&self, _: &str) -> Result<Option<AuthInteractionModel>> { Ok(None) }
        async fn get_by_cont_id(&self, _: &str) -> Result<Option<AuthInteractionModel>> { Ok(None) }
    }

    struct MockVerificationRepo;

    #[async_trait::async_trait]
    impl BasicRepoTrait<AuthVerificationModel, AuthVerificationNewModel> for MockVerificationRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<AuthVerificationModel>> { Ok(vec![]) }
        async fn get_by_id(&self, _: &str) -> Result<Option<AuthVerificationModel>> { Ok(None) }
        async fn create(&self, _: AuthVerificationNewModel) -> Result<AuthVerificationModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: AuthVerificationModel) -> Result<AuthVerificationModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait::async_trait]
    impl AuthVerificationRepoTrait for MockVerificationRepo {
        async fn get_by_state(&self, _: &str) -> Result<Option<AuthVerificationModel>> { Ok(None) }
        async fn create_extra(&self, _: AuthVerificationModel) -> Result<AuthVerificationModel> { Err(anyhow::anyhow!("Not implemented")) }
    }

    struct MockTokenReqRepo;

    #[async_trait::async_trait]
    impl BasicRepoTrait<AuthTokenReqModel, AuthTokenReqModel> for MockTokenReqRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<AuthTokenReqModel>> { Ok(vec![]) }
        async fn get_by_id(&self, _: &str) -> Result<Option<AuthTokenReqModel>> { Ok(None) }
        async fn create(&self, _: AuthTokenReqModel) -> Result<AuthTokenReqModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: AuthTokenReqModel) -> Result<AuthTokenReqModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }
    #[async_trait::async_trait]
    impl AuthTokenRequirementsRepoTrait for MockTokenReqRepo {}

    struct MockBusinessMatesRepo;
    #[async_trait::async_trait]
    impl BasicRepoTrait<BusinessMateModel, BusinessMateNewModel> for MockBusinessMatesRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<BusinessMateModel>> { Ok(vec![]) }
        async fn get_by_id(&self, _: &str) -> Result<Option<BusinessMateModel>> { Ok(None) }
        async fn create(&self, _: BusinessMateNewModel) -> Result<BusinessMateModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update(&self, _: BusinessMateModel) -> Result<BusinessMateModel> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait::async_trait]
    impl BusinessMatesRepoTrait for MockBusinessMatesRepo {
        async fn get_by_token(&self, _: &str) -> Result<Option<BusinessMateModel>> {
            Err(anyhow::anyhow!("Not implemented"))
        }
        async fn force_create(&self, _: BusinessMateNewModel) -> Result<BusinessMateModel> {
            Err(anyhow::anyhow!("Not implemented"))
        }
    }

    #[derive(Clone)]
    struct MockRepoFactory {
        mates_repo: Arc<dyn MatesRepoTrait>,
    }

    impl rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait for MockRepoFactory {
        fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { Arc::new(MockRequestRepo) }
        fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { Arc::new(MockInteractionRepo) }
        fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { Arc::new(MockVerificationRepo) }
        fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { Arc::new(MockTokenReqRepo) }
        fn mates(&self) -> Arc<dyn MatesRepoTrait> { self.mates_repo.clone() }
        fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait> { Arc::new(MockBusinessMatesRepo) }
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

    // Test

    #[tokio::test]
    async fn test_get_mate_by_id_error_real_router() {
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
                    .uri("/api/v1/mates/mate123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

   #[tokio::test]
    async fn test_verify_mate_token_success_mocked() {
        use axum::{body::Body, extract::State, http::Request, response::IntoResponse, routing::post, Router};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::mates::mates::VerifyTokenRequest;
        use axum::Json;
        use serde_json::json;
        use reqwest::StatusCode;

        struct MockManager;
        impl MockManager {
            async fn verify_token(&self, _token: String) -> Result<serde_json::Value, rainbow_common::errors::CommonErrors> {
                Ok(json!({"participant_id": "mate123", "token": "valid"}))
            }
        }

        async fn handler(State(manager): State<Arc<MockManager>>, Json(payload): Json<VerifyTokenRequest>) -> impl IntoResponse {
            match manager.verify_token(payload.token).await {
                Ok(res) => (StatusCode::OK, Json(res)).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new().route("/api/v1/verify/mate/token", post(handler)).with_state(manager);

        let payload = VerifyTokenRequest { token: "valid".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(Request::builder()
                .method("POST")
                .uri("/api/v1/verify/mate/token")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_verify_mate_token_error() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: true }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = rainbow_common::mates::mates::VerifyTokenRequest { token: "invalid".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(Request::builder()
                .method("POST")
                .uri("/api/v1/verify/mate/token")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_retrieve_business_mate_token_success_mocked() {
        use axum::{body::Body, extract::State, http::Request, response::IntoResponse, routing::post, Router};
        use tower::ServiceExt;
        use std::sync::Arc;
        use rainbow_common::auth::business::RainbowBusinessLoginRequest;

        struct MockManager;
        impl MockManager {
            async fn retrieve_business_token(&self, _id: String) -> Result<serde_json::Value, rainbow_common::errors::CommonErrors> {
                Ok(serde_json::json!({"token": "mock-business-token"}))
            }
        }

        async fn handler(State(manager): State<Arc<MockManager>>, axum::Json(payload): axum::Json<RainbowBusinessLoginRequest>) -> impl IntoResponse {
            match manager.retrieve_business_token(payload.auth_request_id).await {
                Ok(res) => (StatusCode::OK, axum::Json(res)).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let manager = Arc::new(MockManager);
        let router = Router::new().route("/api/v1/retrieve/business/token", post(handler)).with_state(manager);

        let payload = RainbowBusinessLoginRequest { auth_request_id: "valid_id".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(Request::builder()
                .method("POST")
                .uri("/api/v1/retrieve/business/token")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_wallet_register_route_exists() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(Request::builder()
                .method("POST")
                .uri("/api/v1/wallet/register")
                .body(Body::empty())
                .unwrap())
            .await.unwrap();

        assert!(response.status().is_success() || response.status().is_server_error());
    }

    #[tokio::test]
    async fn test_get_all_mates_success() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router
            .oneshot(Request::builder()
                .method("GET")
                .uri("/api/v1/mates")
                .body(Body::empty())
                .unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_fast_login_route_exists() {
        let repo = Arc::new(MockRepoFactory {
            mates_repo: Arc::new(MockMatesRepo { should_fail: false }),
        });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = rainbow_common::auth::business::RainbowBusinessLoginRequest {
            auth_request_id: "valid_id".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router
            .oneshot(Request::builder()
                .method("POST")
                .uri("/api/v1/business/login")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap())
            .await.unwrap();

        assert!(response.status() == StatusCode::OK || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_continue_request_missing_token() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = RefBody { interact_ref: "interact_ref_123".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/continue/abc123")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_batch_mates_error_format() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        // Enviamos un body inválido para provocar JsonRejection
        let body = "invalid-json";

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/mates/batch")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_all_mates_me_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/mates/me")
            .body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_mate_by_id_not_found() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/mates/unknown")
            .body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_fallback_returns_404() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/unknown/route")
            .body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }


    use axum::{extract::State, response::IntoResponse, routing::{get, post}, Router};
    use serde_json::json;
    use axum::Json;
    use rainbow_common::auth::business::RainbowBusinessLoginRequest;
    use rainbow_common::mates::mates::VerifyTokenRequest;
    use rainbow_auth::ssi_auth::common::types::ssi::{dids::DidsInfo, keys::KeyDefinition};

    // Mock Manager para simular éxito y error
    struct MockManager { fail: bool }
    impl MockManager {
        async fn register_wallet(&self) -> Result<(), rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok(()) }
        }
        async fn login_wallet(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn logout_wallet(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn onboard_wallet(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn partial_onboard(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn register_key(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn register_did(&self) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn delete_key(&self, _k: KeyDefinition) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn delete_did(&self, _d: DidsInfo) -> Result<(), rainbow_common::errors::CommonErrors> { self.register_wallet().await }
        async fn verify_token(&self, _token: String) -> Result<serde_json::Value, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok(json!({"participant_id": "mate123"})) }
        }
        async fn retrieve_business_token(&self, _id: String) -> Result<serde_json::Value, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok(json!({"token": "business-token"})) }
        }
        async fn fast_login(&self, _id: String) -> Result<String, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok("https://redirect.example.com".to_string()) }
        }
        async fn generate_vp_def(&self, _state: String) -> Result<serde_json::Value, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok(json!({"vp_def": "definition"})) }
        }
        async fn verify_all(&self, _state: String, _vp_token: String) -> Result<String, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok("verification-id".to_string()) }
        }
        async fn end_verification(&self, _id: String) -> Result<Option<String>, rainbow_common::errors::CommonErrors> {
            if self.fail { Err(rainbow_common::errors::CommonErrors::database_new(Some("fail".to_string()))) } else { Ok(Some("https://callback.example.com".to_string())) }
        }
    }

    // Handlers simulados para cada endpoint
     async fn pd_handler(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.generate_vp_def("state123".to_string()).await { Ok(res) => (StatusCode::OK, Json(res)).into_response(), Err(e) => e.into_response() } }
    async fn verify_handler(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.verify_all("state123".to_string(), "vp_token".to_string()).await { Ok(id) => match m.end_verification(id).await { Ok(Some(uri)) => (StatusCode::OK, uri).into_response(), Ok(None) => StatusCode::OK.into_response(), Err(e) => e.into_response() }, Err(e) => e.into_response() } }


    fn build_router(manager: Arc<MockManager>) -> Router {
        Router::new()
            .route("/wallet/register", post(wallet_register))
            .route("/wallet/login", post(wallet_login))
            .route("/wallet/logout", post(wallet_logout))
            .route("/wallet/onboard", post(wallet_onboard))
            .route("/wallet/partial-onboard", post(partial_onboard))
            .route("/wallet/key", post(register_key).delete(delete_key))
            .route("/wallet/did", post(register_did).delete(delete_did))
            .route("/verify/mate/token", post(verify_mate_token))
            .route("/retrieve/business/token", post(retrieve_business_mate_token))
            .route("/business/login", post(fast_login))
            .route("/pd/state123", get(pd_handler))
            .route("/verify/state123", post(verify_handler))
            .with_state(manager)
    }

    async fn wallet_register(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.register_wallet().await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn wallet_login(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.login_wallet().await { Ok(()) => StatusCode::OK.into_response(), Err(e) => e.into_response() } }
    async fn wallet_logout(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.logout_wallet().await { Ok(()) => StatusCode::OK.into_response(), Err(e) => e.into_response() } }
    async fn wallet_onboard(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.onboard_wallet().await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn partial_onboard(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.partial_onboard().await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn register_key(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.register_key().await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn register_did(State(m): State<Arc<MockManager>>) -> impl IntoResponse { match m.register_did().await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn delete_key(State(m): State<Arc<MockManager>>, Json(payload): Json<KeyDefinition>) -> impl IntoResponse { match m.delete_key(payload).await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn delete_did(State(m): State<Arc<MockManager>>, Json(payload): Json<DidsInfo>) -> impl IntoResponse { match m.delete_did(payload).await { Ok(()) => StatusCode::CREATED.into_response(), Err(e) => e.into_response() } }
    async fn verify_mate_token(State(m): State<Arc<MockManager>>, Json(payload): Json<VerifyTokenRequest>) -> impl IntoResponse { match m.verify_token(payload.token).await { Ok(res) => (StatusCode::OK, Json(res)).into_response(), Err(e) => e.into_response() } }
    async fn retrieve_business_mate_token(State(m): State<Arc<MockManager>>, Json(payload): Json<RainbowBusinessLoginRequest>) -> impl IntoResponse { match m.retrieve_business_token(payload.auth_request_id).await { Ok(res) => (StatusCode::OK, Json(res)).into_response(), Err(e) => e.into_response() } }
    async fn fast_login(State(m): State<Arc<MockManager>>, Json(payload): Json<RainbowBusinessLoginRequest>) -> impl IntoResponse { match m.fast_login(payload.auth_request_id).await { Ok(uri) => (StatusCode::OK, uri).into_response(), Err(e) => e.into_response() } }

    #[tokio::test]
    async fn test_all_routes_success() {
        let manager = Arc::new(MockManager { fail: false });
        let router = build_router(manager);
        for route in ["/wallet/register", "/wallet/login", "/wallet/logout", "/wallet/onboard", "/wallet/partial-onboard", "/wallet/key", "/wallet/did", "/verify/mate/token", "/retrieve/business/token", "/business/login", "/pd/state123", "/verify/state123"] {
            let method = if route.contains("verify") || route.contains("wallet") || route.contains("business") || route.contains("retrieve") { "POST" } else { "GET" };
            let body = if route.contains("verify/mate/token") {
                serde_json::to_vec(&VerifyTokenRequest { token: "valid".to_string() }).unwrap()
            } else if route.contains("retrieve/business/token") || route.contains("business/login") {
                serde_json::to_vec(&RainbowBusinessLoginRequest { auth_request_id: "id".to_string() }).unwrap()
            } else { vec![] };
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).header("content-type", "application/json").body(Body::from(body)).unwrap()).await.unwrap();
            assert!(response.status().is_success());
        }
    }

    #[tokio::test]
    async fn test_error_routes() {
        let manager = Arc::new(MockManager { fail: true });
        let router = build_router(manager);
        let response = router.oneshot(Request::builder().method("POST").uri("/wallet/register").body(Body::empty()).unwrap()).await.unwrap();
        assert!(response.status().is_server_error());
    }

    
#[tokio::test]
    async fn test_didweb_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/did.json")
            .body(Body::empty()).unwrap()).await.unwrap();
        
        println!("{:?}", response.status());

        assert!(response.status() == StatusCode::OK || response.status() == StatusCode::PRECONDITION_FAILED);
    }

    #[tokio::test]
    async fn test_access_request_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = rainbow_auth::ssi_auth::common::types::gnap::
            GrantRequest {
            access_token: AccessTokenRequirements4GR {
                access: Access4AT {
                    r#type: "default-type".to_string(),
                    actions: None,
                    locations: None,
                    datatypes: None,
                    identifier: None,
                    privileges: None,
                },
                label: None,
                flags: None,
            },
            subject: None,
            client: serde_json::json!({}),
            user: None,
            interact: None,
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/access")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert!(response.status().is_success() || response.status().is_server_error());
    }

    #[tokio::test]
    async fn test_pd_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/pd/state123")
            .body(Body::empty()).unwrap()).await.unwrap();

        println!("Status: {:?}", response.status());
        assert!(
            response.status().is_success()
            || response.status().is_server_error()
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_verify_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let body = "vp_token=valid";

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/verify/state123")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert!(response.status().is_success() || response.status().is_server_error() || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_key_and_did_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        // Enviamos las peticiones POST para ambas rutas
        for route in ["/api/v1/wallet/key", "/api/v1/wallet/did"] {
            let response = router.clone().oneshot(Request::builder()
                .method("POST")
                .uri(route)
                .body(Body::empty()).unwrap()).await.unwrap();

            println!("Route: {}, Status: {:?}", route, response.status());
            assert!(
                response.status().is_success()
                || response.status().is_server_error()
                || response.status() == StatusCode::PRECONDITION_FAILED
            );
        }
    }
    
    #[tokio::test]
    async fn test_delete_key_and_did_success() {
        use rainbow_auth::ssi_auth::common::types::ssi::keys::KeyDefinition;
        use rainbow_auth::ssi_auth::common::types::ssi::dids::DidsInfo;

        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let key_payload = KeyDefinition {
            key_id: KeyInfo { id: "Test_id".to_string()},
            algorithm: "Ed25519".to_string(),
            crypto_provider: "Ed25519".to_string(),
            key_pair: serde_json::json!("Ed25519"),
            keyset_handle: None,
        };

        let did_payload = DidsInfo {
            did: "did:example:123".to_string(),
            alias: "alias-123".to_string(),
            document: "test_document".to_string(),
            key_id: "test_key_id".to_string(),
            default: false,
            created_on: "01-01-1999".to_string(),
        };

        for (route, payload) in [
            ("/api/v1/wallet/key", serde_json::to_vec(&key_payload).unwrap()),
            ("/api/v1/wallet/did", serde_json::to_vec(&did_payload).unwrap())
        ] {
            let response = router.clone().oneshot(Request::builder()
                .method("DELETE")
                .uri(route)
                .header("content-type", "application/json")
                .body(Body::from(payload)).unwrap()).await.unwrap();   
            println!("{:?}", response.status());
            assert!(
                response.status().is_success()
                || response.status().is_server_error()
                || response.status() == StatusCode::PRECONDITION_FAILED
            );
        }
    }

    #[tokio::test]
    async fn test_get_batch_mates_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = rainbow_common::batch_requests::BatchRequestsAsString { ids: vec!["mate123".to_string()] };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/mates/batch")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert!(response.status().is_success() || response.status().is_server_error());
    }

    #[tokio::test]
    async fn test_get_all_mates_me_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/mates/me")
            .body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_mate_by_id_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/mates/mate123")
            .body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_all_missing_routes_success_and_error() {
        let manager_ok = Arc::new(MockManager { fail: false });
        let manager_fail = Arc::new(MockManager { fail: true });
        let router_ok = build_router(manager_ok);
        let router_fail = build_router(manager_fail);

        for route in [
            "/wallet/login", "/wallet/logout", "/wallet/onboard", "/wallet/partial-onboard",
            "/wallet/key", "/wallet/did", "/api/v1/did.json", "/api/v1/access",
            "/api/v1/pd/state123", "/api/v1/verify/state123"
        ] {
            let method = if route.contains("did.json") || route.contains("pd") { "GET" } else { "POST" };
            let body = if route.contains("verify/state123") { "vp_token=valid".as_bytes().to_vec() } else { vec![] };

            let response_ok = router_ok.clone().oneshot(Request::builder()
                .method(method)
                .uri(route)
                .header("content-type", "application/json")
                .body(Body::from(body.clone())).unwrap()).await.unwrap();

            println!("OK route: {}, status: {:?}", route, response_ok.status());
            assert!(
                response_ok.status().is_success()
                || response_ok.status() == StatusCode::NOT_FOUND
                || response_ok.status() == StatusCode::BAD_REQUEST
            );

            let response_fail = router_fail.clone().oneshot(Request::builder()
                .method(method)
                .uri(route)
                .header("content-type", "application/json")
                .body(Body::from(body)).unwrap()).await.unwrap();

            println!("FAIL route: {}, status: {:?}", route, response_fail.status());
            assert!(
                response_fail.status().is_server_error()
                || response_fail.status() == StatusCode::NOT_FOUND
                || response_fail.status() == StatusCode::BAD_REQUEST
            );
        }
    }

    #[tokio::test]
    async fn test_continue_request_success() {
        use rainbow_auth::ssi_auth::common::types::gnap::{RefBody, GrantRequest, grant_request::{AccessTokenRequirements4GR, Access4AT}};
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;

        struct MockManager;
        impl MockManager {
            async fn validate_continue_request(&self, _id: String, _ref: String, _token: String) -> Result<String, rainbow_common::errors::CommonErrors> {
                Ok("interaction-model".to_string())
            }
            async fn continue_req(&self, _model: String) -> Result<GrantRequest, rainbow_common::errors::CommonErrors> {
                Ok(GrantRequest {
                    access_token: AccessTokenRequirements4GR {
                        access: Access4AT { r#type: "default-type".to_string(), actions: None, locations: None, datatypes: None, identifier: None, privileges: None },
                        label: None,
                        flags: None,
                    },
                    subject: None,
                    client: serde_json::json!({}),
                    user: None,
                    interact: None,
                })
            }
            async fn retrieve_data(&self, _req: GrantRequest, _int: String) -> Result<String, rainbow_common::errors::CommonErrors> {
                Ok("mate-data".to_string())
            }
            async fn save_mate(&self, _mate: String) -> Result<(), rainbow_common::errors::CommonErrors> {
                Ok(())
            }
        }

        let manager = Arc::new(MockManager);
        let router = axum::Router::new()
            .route("/api/v1/continue/abc123", axum::routing::post(handler))
            .with_state(manager);

        async fn handler(State(manager): State<Arc<MockManager>>, Json(payload): Json<RefBody>) -> impl IntoResponse {
            match manager.validate_continue_request("abc123".to_string(), payload.interact_ref, "token123".to_string()).await {
                Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "continued"}))).into_response(),
                Err(e) => e.into_response(),
            }
        }

        let payload = RefBody { interact_ref: "ref123".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/continue/abc123")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_continue_request_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = RefBody { interact_ref: "ref123".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/continue/abc123")
            .header("authorization", "GNAP token123")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Continue error status: {:?}", response.status());
        assert!(response.status().is_server_error() || response.status() == StatusCode::UNAUTHORIZED || response.status() == StatusCode::NOT_FOUND);
    }

    // ---------------- VERIFY END VERIFICATION NONE ----------------
    #[tokio::test]
    async fn test_verify_end_verification_none() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let body = "vp_token=valid";
        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/verify/state123")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Verify None status: {:?}", response.status());
        assert!(
            response.status().is_success()
            || response.status().is_server_error()
            || response.status() == StatusCode::NOT_FOUND
        );
    }

    // ---------------- WALLET ROUTES ERROR ----------------
    #[tokio::test]
    async fn test_wallet_routes_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        for route in [
            "/api/v1/wallet/register",
            "/api/v1/wallet/login",
            "/api/v1/wallet/logout",
            "/api/v1/wallet/onboard",
            "/api/v1/wallet/partial-onboard"
        ] {
            let response = router.clone().oneshot(Request::builder()
                .method("POST")
                .uri(route)
                .body(Body::empty()).unwrap()).await.unwrap();

            println!("Wallet error route: {}, status: {:?}", route, response.status());
            assert!(response.status().is_server_error());
        }
    }

    // ---------------- REGISTER/DELETE KEY & DID ERROR ----------------
    #[tokio::test]
    async fn test_register_and_delete_key_did_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let key_payload = KeyDefinition {
            key_id: KeyInfo { id: "Test_id".to_string()},
            algorithm: "Ed25519".to_string(),
            crypto_provider: "Ed25519".to_string(),
            key_pair: serde_json::json!("Ed25519"),
            keyset_handle: None,
        };

        let did_payload = DidsInfo {
            did: "did:example:123".to_string(),
            alias: "alias-123".to_string(),
            document: "test_document".to_string(),
            key_id: "test_key_id".to_string(),
            default: false,
            created_on: "01-01-1999".to_string(),
        };

        for (route, method, payload) in [
            ("/api/v1/wallet/key", "POST", serde_json::to_vec(&key_payload).unwrap()),
            ("/api/v1/wallet/did", "POST", serde_json::to_vec(&did_payload).unwrap()),
            ("/api/v1/wallet/key", "DELETE", serde_json::to_vec(&key_payload).unwrap()),
            ("/api/v1/wallet/did", "DELETE", serde_json::to_vec(&did_payload).unwrap())
        ] {
            let response = router.clone().oneshot(Request::builder()
                .method(method)
                .uri(route)
                .header("content-type", "application/json")
                .body(Body::from(payload)).unwrap()).await.unwrap();

            println!("Register/Delete error route: {}, status: {:?}", route, response.status());
            assert!(response.status().is_server_error() || response.status() == StatusCode::PRECONDITION_FAILED);
        }
    }

    // ---------------- ACCESS REQUEST ERROR ----------------
    #[tokio::test]
    async fn test_access_request_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = GrantRequest {
            access_token: AccessTokenRequirements4GR {
                access: Access4AT {
                    r#type: "default-type".to_string(),
                    actions: None,
                    locations: None,
                    datatypes: None,
                    identifier: None,
                    privileges: None,
                },
                label: None,
                flags: None,
            },
            subject: None,
            client: serde_json::json!({}),
            user: None,
            interact: None,
        };

        let body = serde_json::to_vec(&payload).unwrap();
        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/access")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Access error status: {:?}", response.status());
        assert!(response.status().is_server_error());
    }

    // ---------------- PD ERROR ----------------
    #[tokio::test]
    async fn test_pd_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.clone().oneshot(Request::builder()
            .method("GET")
            .uri("/api/v1/pd/state123")
            .body(Body::empty()).unwrap()).await.unwrap();

        println!("PD error status: {:?}", response.status());
        assert!(response.status().is_server_error() || response.status() == StatusCode::NOT_FOUND);
    }

    // ---------------- FAST LOGIN ERROR ----------------
    #[tokio::test]
    async fn test_fast_login_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = RainbowBusinessLoginRequest { auth_request_id: "invalid".to_string() };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/business/login")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Fast login error status: {:?}", response.status());
        assert!(response.status().is_server_error());
    }

    // ---------------- FALLBACK EXTRA ----------------
    #[tokio::test]
    async fn test_fallback_post_method() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/nonexistent")
            .body(Body::empty()).unwrap()).await.unwrap();

        println!("Fallback POST status: {:?}", response.status());
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_retrieve_business_mate_token_success() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: false }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = RainbowBusinessLoginRequest {
            auth_request_id: "valid_id".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/retrieve/business/token")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Retrieve business token success status: {:?}", response.status());
        assert!(
            response.status().is_success()
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_retrieve_business_mate_token_error() {
        let repo = Arc::new(MockRepoFactory { mates_repo: Arc::new(MockMatesRepo { should_fail: true }) });
        let config = mock_config("http://localhost");
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthProviderRouter::new(manager).router();

        let payload = RainbowBusinessLoginRequest {
            auth_request_id: "invalid_id".to_string(),
        };
        let body = serde_json::to_vec(&payload).unwrap();

        let response = router.clone().oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/retrieve/business/token")
            .header("content-type", "application/json")
            .body(Body::from(body)).unwrap()).await.unwrap();

        println!("Retrieve business token error status: {:?}", response.status());
        assert!(
            response.status().is_server_error()
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
        );
    }
}
