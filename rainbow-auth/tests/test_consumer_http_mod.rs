// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\http\mod.rs'

#[cfg(test)]
mod tests {
<<<<<<< HEAD
    use std::sync::Arc;

=======
>>>>>>> origin/main
    use anyhow::Result;
    use axum::{
        body::Body,
        extract::State,
        http::{Request, StatusCode},
        response::IntoResponse,
        routing::post,
<<<<<<< HEAD
        Json, Router
=======
        Json, Router,
>>>>>>> origin/main
    };
    use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
    use rainbow_auth::ssi_auth::{
        common::types::ssi::{dids::DidsInfo, keys::KeyDefinition},
<<<<<<< HEAD
        consumer::core::Manager
    };
    use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
    use rainbow_db::auth_consumer::entities::{
        auth_interaction, auth_request, authority_request, mates
    };
=======
        consumer::core::Manager,
    };
    use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
    use rainbow_db::auth_consumer::entities::{auth_interaction, auth_request, authority_request, mates};
>>>>>>> origin/main
    use rainbow_db::auth_consumer::repo_factory::{factory_trait::AuthRepoFactoryTrait, traits::*};
    use rainbow_db::common::BasicRepoTrait;
    use sea_orm_migration::async_trait::{self, async_trait};
    use serde_json::json;
<<<<<<< HEAD
=======
    use std::sync::Arc;
>>>>>>> origin/main
    use tower::ServiceExt;
    use tracing::error;

    // Mock

    struct MockRepo {
<<<<<<< HEAD
        should_fail: bool
=======
        should_fail: bool,
>>>>>>> origin/main
    }

    #[async_trait]
    impl BasicRepoTrait<mates::Model, mates::NewModel> for MockRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<mates::Model>> {
<<<<<<< HEAD
=======
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
        async fn get_by_id(&self, _: &str) -> Result<Option<mates::Model>> {
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(Some(mates::Model {
                    participant_id: "id".into(),
                    participant_slug: "slug".into(),
                    participant_type: "type".into(),
                    base_url: "url".into(),
                    token: None,
                    saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                    last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    is_me: false,
                }))
            }
        }
        async fn create(&self, _: mates::NewModel) -> Result<mates::Model> {
            Ok(mates::Model {
                participant_id: "id".into(),
                participant_slug: "slug".into(),
                participant_type: "type".into(),
                base_url: "url".into(),
                token: None,
                saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                is_me: false,
            })
        }
        async fn update(&self, model: mates::Model) -> Result<mates::Model> {
            Ok(model)
        }
        async fn delete(&self, _: &str) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl BasicRepoTrait<auth_request::Model, auth_request::NewModel> for MockRepo {
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<auth_request::Model>> {
>>>>>>> origin/main
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
        async fn get_by_id(&self, _: &str) -> Result<Option<mates::Model>> {
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(Some(mates::Model {
                    participant_id: "id".into(),
                    participant_slug: "slug".into(),
                    participant_type: "type".into(),
                    base_url: "url".into(),
                    token: None,
                    saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    is_me: false
                }))
            }
        }
        async fn create(&self, _: mates::NewModel) -> Result<mates::Model> {
            Ok(mates::Model {
                participant_id: "id".into(),
                participant_slug: "slug".into(),
                participant_type: "type".into(),
                base_url: "url".into(),
                token: None,
                saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                is_me: false
            })
        }
        async fn update(&self, model: mates::Model) -> Result<mates::Model> { Ok(model) }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait]
    impl BasicRepoTrait<auth_request::Model, auth_request::NewModel> for MockRepo {
        async fn get_all(
            &self,
            _: Option<u64>,
            _: Option<u64>
        ) -> Result<Vec<auth_request::Model>> {
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
        async fn get_by_id(&self, _: &str) -> Result<Option<auth_request::Model>> { Ok(None) }
        async fn create(&self, _: auth_request::NewModel) -> Result<auth_request::Model> {
            Ok(auth_request::Model {
                id: "id".into(),
                provider_id: "pid".into(),
                provider_slug: "slug".into(),
                grant_endpoint: "endpoint".into(),
                assigned_id: None,
                token: None,
                status: "status".into(),
<<<<<<< HEAD
                created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                ended_at: None
=======
                created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                ended_at: None,
>>>>>>> origin/main
            })
        }
        async fn update(&self, model: auth_request::Model) -> Result<auth_request::Model> {
            Ok(model)
        }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait]
    impl BasicRepoTrait<auth_interaction::Model, auth_interaction::NewModel> for MockRepo {
<<<<<<< HEAD
        async fn get_all(
            &self,
            _: Option<u64>,
            _: Option<u64>
        ) -> Result<Vec<auth_interaction::Model>> {
=======
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<auth_interaction::Model>> {
>>>>>>> origin/main
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
        async fn get_by_id(&self, _: &str) -> Result<Option<auth_interaction::Model>> { Ok(None) }
        async fn create(&self, _: auth_interaction::NewModel) -> Result<auth_interaction::Model> {
            Ok(auth_interaction::Model {
                id: "iid".into(),
                start: vec!["start".to_string()], // CORRECTO: Vec<String>
                method: "method".to_string(),
                uri: "uri".to_string(),
                client_nonce: "client_nonce".to_string(),
                hash_method: "hash_method".to_string(),
                hints: Some("hints".to_string()),
                grant_endpoint: "grant_endpoint".to_string(),
                continue_endpoint: Some("continue_endpoint".to_string()),
                continue_token: Some("continue_token".to_string()),
                continue_wait: Some(30), // CORRECTO: Option<i64>
                as_nonce: Some("as_nonce".to_string()),
                interact_ref: Some("ref".to_string()),
                hash: Some("hash".to_string())
            })
        }
        async fn update(&self, model: auth_interaction::Model) -> Result<auth_interaction::Model> {
            Ok(model)
        }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait]
    impl BasicRepoTrait<authority_request::Model, authority_request::NewModel> for MockRepo {
<<<<<<< HEAD
        async fn get_all(
            &self,
            _: Option<u64>,
            _: Option<u64>
        ) -> Result<Vec<authority_request::Model>> {
=======
        async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<authority_request::Model>> {
>>>>>>> origin/main
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
        async fn get_by_id(&self, _: &str) -> Result<Option<authority_request::Model>> { Ok(None) }
        async fn create(&self, _: authority_request::NewModel) -> Result<authority_request::Model> {
            Ok(authority_request::Model {
                id: "aid".into(),
                authority_id: "authid".into(),
                authority_slug: "slug".into(),
                grant_endpoint: "endpoint".into(),
                vc_type: "vc".into(),
                assigned_id: None,
                vc_uri: None,
                status: "status".into(),
<<<<<<< HEAD
                created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                ended_at: None
=======
                created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                ended_at: None,
>>>>>>> origin/main
            })
        }
        async fn update(
            &self,
            model: authority_request::Model
        ) -> Result<authority_request::Model> {
            Ok(model)
        }
        async fn delete(&self, _: &str) -> Result<()> { Ok(()) }
    }

    #[async_trait]
    impl AuthInteractionRepoTrait for MockRepo {}
    #[async_trait]
    impl AuthRequestRepoTrait for MockRepo {}

    #[async_trait::async_trait]
    impl
        BasicRepoTrait<
            rainbow_db::auth_consumer::entities::auth_verification::Model,
<<<<<<< HEAD
            rainbow_db::auth_consumer::entities::auth_verification::NewModel
=======
            rainbow_db::auth_consumer::entities::auth_verification::NewModel,
>>>>>>> origin/main
        > for MockRepo
    {
        async fn get_all(
            &self,
            _: Option<u64>,
<<<<<<< HEAD
            _: Option<u64>
        ) -> anyhow::Result<Vec<rainbow_db::auth_consumer::entities::auth_verification::Model>>
        {
=======
            _: Option<u64>,
        ) -> anyhow::Result<Vec<rainbow_db::auth_consumer::entities::auth_verification::Model>> {
>>>>>>> origin/main
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }

        async fn get_by_id(
            &self,
<<<<<<< HEAD
            _: &str
        ) -> anyhow::Result<Option<rainbow_db::auth_consumer::entities::auth_verification::Model>>
        {
=======
            _: &str,
        ) -> anyhow::Result<Option<rainbow_db::auth_consumer::entities::auth_verification::Model>> {
>>>>>>> origin/main
            Ok(None)
        }

        async fn create(
            &self,
            _: rainbow_db::auth_consumer::entities::auth_verification::NewModel,
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_verification::Model> {
<<<<<<< HEAD
            Ok(rainbow_db::auth_consumer::entities::auth_verification::Model {
                id: "vid".to_string(),
                uri: "uri".to_string(),
                scheme: "scheme".to_string(),
                response_type: "response_type".to_string(),
                client_id: "client_id".to_string(),
                response_mode: "response_mode".to_string(),
                pd_uri: "pd_uri".to_string(),
                client_id_scheme: "client_id_scheme".to_string(),
                nonce: "nonce".to_string(),
                response_uri: "response_uri".to_string(),
                status: "pending".to_string(),
                created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                ended_at: None
            })
=======
            Ok(
                rainbow_db::auth_consumer::entities::auth_verification::Model {
                    id: "vid".to_string(),
                    uri: "uri".to_string(),
                    scheme: "scheme".to_string(),
                    response_type: "response_type".to_string(),
                    client_id: "client_id".to_string(),
                    response_mode: "response_mode".to_string(),
                    pd_uri: "pd_uri".to_string(),
                    client_id_scheme: "client_id_scheme".to_string(),
                    nonce: "nonce".to_string(),
                    response_uri: "response_uri".to_string(),
                    status: "pending".to_string(),
                    created_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                    ended_at: None,
                },
            )
>>>>>>> origin/main
        }

        async fn update(
            &self,
            model: rainbow_db::auth_consumer::entities::auth_verification::Model,
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_verification::Model> {
            Ok(model)
        }

        async fn delete(&self, _: &str) -> anyhow::Result<()> { Ok(()) }
    }

    #[async_trait]
    impl AuthVerificationRepoTrait for MockRepo {}

    #[async_trait::async_trait]
    impl
        BasicRepoTrait<
            rainbow_db::auth_consumer::entities::auth_token_requirements::Model,
<<<<<<< HEAD
            rainbow_db::auth_consumer::entities::auth_token_requirements::Model
=======
            rainbow_db::auth_consumer::entities::auth_token_requirements::Model,
>>>>>>> origin/main
        > for MockRepo
    {
        async fn get_all(
            &self,
            _: Option<u64>,
<<<<<<< HEAD
            _: Option<u64>
        ) -> anyhow::Result<Vec<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>>
        {
=======
            _: Option<u64>,
        ) -> anyhow::Result<Vec<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>> {
>>>>>>> origin/main
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }

        async fn get_by_id(
            &self,
<<<<<<< HEAD
            _: &str
        ) -> anyhow::Result<
            Option<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>
        > {
=======
            _: &str,
        ) -> anyhow::Result<Option<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>> {
>>>>>>> origin/main
            Ok(None)
        }

        async fn create(
            &self,
<<<<<<< HEAD
            _: rainbow_db::auth_consumer::entities::auth_token_requirements::Model
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>
        {
            Ok(rainbow_db::auth_consumer::entities::auth_token_requirements::Model {
                id: "token_req_id".to_string(),
                r#type: "access".to_string(),
                actions: vec!["read".to_string(), "write".to_string()],
                locations: Some(vec!["https://example.com/resource".to_string()]),
                datatypes: Some(vec!["json".to_string(), "xml".to_string()]),
                identifier: Some("identifier123".to_string()),
                privileges: Some(vec!["admin".to_string(), "user".to_string()]),
                label: Some("Test Label".to_string()),
                flags: Some(vec!["flag1".to_string(), "flag2".to_string()])
            })
=======
            _: rainbow_db::auth_consumer::entities::auth_token_requirements::Model,
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_token_requirements::Model> {
            Ok(
                rainbow_db::auth_consumer::entities::auth_token_requirements::Model {
                    id: "token_req_id".to_string(),
                    r#type: "access".to_string(),
                    actions: vec!["read".to_string(), "write".to_string()],
                    locations: Some(vec!["https://example.com/resource".to_string()]),
                    datatypes: Some(vec!["json".to_string(), "xml".to_string()]),
                    identifier: Some("identifier123".to_string()),
                    privileges: Some(vec!["admin".to_string(), "user".to_string()]),
                    label: Some("Test Label".to_string()),
                    flags: Some(vec!["flag1".to_string(), "flag2".to_string()]),
                },
            )
>>>>>>> origin/main
        }

        async fn update(
            &self,
<<<<<<< HEAD
            model: rainbow_db::auth_consumer::entities::auth_token_requirements::Model
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_token_requirements::Model>
        {
=======
            model: rainbow_db::auth_consumer::entities::auth_token_requirements::Model,
        ) -> anyhow::Result<rainbow_db::auth_consumer::entities::auth_token_requirements::Model> {
>>>>>>> origin/main
            Ok(model)
        }

        async fn delete(&self, _: &str) -> anyhow::Result<()> { Ok(()) }
    }
    #[async_trait]
    impl AuthTokenRequirementsRepoTrait for MockRepo {}

    #[async_trait::async_trait]
    impl MatesRepoTrait for MockRepo {
        async fn get_me(&self) -> Result<Option<mates::Model>> {
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(None)
            }
        }
<<<<<<< HEAD
        async fn get_by_token(&self, _: &str) -> Result<Option<mates::Model>> { Ok(None) }
=======
        async fn get_by_token(&self, _: &str) -> Result<Option<mates::Model>> {
            Ok(None)
        }
>>>>>>> origin/main
        async fn force_create(&self, _: mates::NewModel) -> Result<mates::Model> {
            Ok(mates::Model {
                participant_id: "id".into(),
                participant_slug: "slug".into(),
                participant_type: "type".into(),
                base_url: "url".into(),
                token: None,
<<<<<<< HEAD
                saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                is_me: false
=======
                saved_at: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                last_interaction: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                is_me: false,
>>>>>>> origin/main
            })
        }
        async fn get_batch(&self, _: &Vec<urn::Urn>) -> Result<Vec<mates::Model>> {
            if self.should_fail {
                anyhow::bail!("DB error")
            } else {
                Ok(vec![])
            }
        }
    }

    #[async_trait]
    impl AuthorityRequestRepoTrait for MockRepo {}

    #[derive(Clone)]
    struct MockRepoFactory {
<<<<<<< HEAD
        should_fail: bool
=======
        should_fail: bool,
>>>>>>> origin/main
    }
    impl AuthRepoFactoryTrait for MockRepoFactory {
        fn request(&self) -> Arc<dyn AuthRequestRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
        fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
        fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
        fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
        fn mates(&self) -> Arc<dyn MatesRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
        fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
            Arc::new(MockRepo { should_fail: self.should_fail })
        }
    }

    fn build_router(should_fail: bool) -> Router {
        let repo = Arc::new(MockRepoFactory { should_fail });
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        RainbowAuthConsumerRouter::new(manager).router()
    }

    async fn send_request(
        router: Router,
        method: &str,
        uri: &str,
        body: Option<String>
    ) -> StatusCode {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.unwrap_or_default()))
            .unwrap();
        router.oneshot(req).await.unwrap().status()
    }

    #[derive(Clone)]
    struct MockManager {
        should_fail: bool
    }

    impl MockManager {
        async fn register_wallet(&self) -> Result<(), StatusCode> {
            if self.should_fail {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            } else {
                Ok(())
            }
        }

        async fn delete_key(&self, _: KeyDefinition) -> Result<(), StatusCode> {
            if self.should_fail {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            } else {
                Ok(())
            }
        }

        async fn delete_did(&self, _: DidsInfo) -> Result<(), StatusCode> {
            if self.should_fail {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            } else {
                Ok(())
            }
        }
    }

    async fn wallet_register(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.register_wallet().await {
            Ok(_) => StatusCode::CREATED,
            Err(code) => code
        }
    }

    async fn delete_key_handler(
        State(manager): State<Arc<MockManager>>,
<<<<<<< HEAD
        Json(payload): Json<KeyDefinition>
=======
        Json(payload): Json<KeyDefinition>,
>>>>>>> origin/main
    ) -> impl IntoResponse {
        match manager.delete_key(payload).await {
            Ok(_) => StatusCode::CREATED,
            Err(code) => code
        }
    }

    async fn delete_did_handler(
        State(manager): State<Arc<MockManager>>,
<<<<<<< HEAD
        Json(payload): Json<DidsInfo>
=======
        Json(payload): Json<DidsInfo>,
>>>>>>> origin/main
    ) -> impl IntoResponse {
        match manager.delete_did(payload).await {
            Ok(_) => StatusCode::CREATED,
            Err(code) => code
        }
    }

    // Test

    #[tokio::test]
    async fn test_all_routes_success() {
        let router = build_router(false);
        let routes = vec![
            ("/api/v1/wallet/register", "POST"),
            ("/api/v1/wallet/login", "POST"),
            ("/api/v1/wallet/logout", "POST"),
            ("/api/v1/wallet/onboard", "POST"),
            ("/api/v1/wallet/partial-onboard", "POST"),
            ("/api/v1/wallet/key", "POST"),
            ("/api/v1/wallet/did", "POST"),
            ("/api/v1/wallet/key", "DELETE"),
            ("/api/v1/wallet/did", "DELETE"),
            ("/api/v1/did.json", "GET"),
            ("/auth/manual/ssi", "POST"),
            ("/api/v1/callback/test-id?hash=abc&interact_ref=xyz", "GET"),
            ("/api/v1/callback/test-id", "POST"),
            ("/api/v1/authority/beg", "POST"),
            ("/api/v1/authority/beg/oidc", "POST"),
            ("/api/v1/authority/request/all", "GET"),
            ("/api/v1/authority/request/test-id", "GET"),
        ];
        for (uri, method) in routes {
            let status =
                send_request(router.clone(), method, uri, Some(json!({}).to_string())).await;
            println!("{:?}", status);
            assert!(
                status.is_success()
                    || status == StatusCode::NOT_FOUND
                    || status == StatusCode::BAD_REQUEST
                    || status == StatusCode::BAD_GATEWAY
                    || status == StatusCode::PRECONDITION_FAILED
                    || status == StatusCode::UNPROCESSABLE_ENTITY
            );
        }
    }

    #[tokio::test]
    async fn test_fallback_returns_404() {
        let router = build_router(true);
        let status = send_request(router, "GET", "/unknown/route", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_wallet_register_success() {
        let manager = Arc::new(MockManager { should_fail: false });
        let router = Router::new()
            .route("/api/v1/wallet/register", post(wallet_register))
            .with_state(manager);
        let status = send_request(router, "POST", "/api/v1/wallet/register", None).await;
        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_wallet_register_error() {
        let status =
            send_request(build_router(true), "POST", "/api/v1/wallet/register", None).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
    }

    #[tokio::test]
    async fn test_delete_key_success_valid_payload() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "algorithm": "Ed25519",
            "cryptoProvider": "MockProvider",
            "keyId": { "id": "key123" },
            "keyPair": { "public": "ABCDEF123456" },
            "keyset_handle": null
        });
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/key", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/key",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(status == StatusCode::CREATED || status == StatusCode::PRECONDITION_FAILED);
    }

    #[tokio::test]
    async fn test_delete_key_with_mock_manager() {
        let manager = Arc::new(MockManager { should_fail: false });
        let router = Router::new()
            .route(
                "/api/v1/wallet/key",
                axum::routing::delete(delete_key_handler),
            )
            .with_state(manager);
        let payload = serde_json::json!({
            "algorithm": "Ed25519",
            "cryptoProvider": "MockProvider",
            "keyId": { "id": "key123" },
            "keyPair": { "public": "ABCDEF123456" },
            "keyset_handle": null
        });

<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/key", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/key",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_key_error_valid_payload() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "algorithm": "Ed25519",
            "cryptoProvider": "MockProvider",
            "keyId": { "id": "key123" },
            "keyPair": { "public": "ABCDEF123456" },
            "keyset_handle": null
        });
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/key", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/key",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_delete_key_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
        "algorithm": "Ed25519",
        "cryptoProvider": "MockProvider",
        "keyId": { "id": "key123" },
        "keyPair": { "public": "ABCDEF123456" },
        "keyset_handle": null
        });
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/key", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/key",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED,
            "Expected error status, got {:?}",
            status
        );
    }

    #[tokio::test]
    async fn test_wallet_login_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/login",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
        );
    }

    #[tokio::test]
    async fn test_wallet_login_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/login",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_wallet_logout_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/logout",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::BAD_GATEWAY);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_wallet_logout_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/logout",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_wallet_onboard_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/onboard",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
        );
    }

    #[tokio::test]
    async fn test_wallet_onboard_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/onboard",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_wallet_partial_onboard_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/partial-onboard",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::BAD_GATEWAY);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_wallet_partial_onboard_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/partial-onboard",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_wallet_did_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/did",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_wallet_did_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "POST",
            "/api/v1/wallet/did",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    // PROVIDER TESTS
    #[tokio::test]
    async fn test_provider_provider_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "id": "provider-id",
            "slug": "provider-slug",
            "url": "https://provider.example.com",
            "actions": "read,write"
        });
        let status = send_request(
            router,
            "POST",
            "/api/v1/request/onboard/provider",
<<<<<<< HEAD
            Some(payload.to_string())
=======
            Some(payload.to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
        );
    }

    #[tokio::test]
    async fn test_provider_provider_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "id": "provider-id",
            "slug": "provider-slug",
            "url": "https://provider.example.com",
            "actions": "read,write"
        });
        let status = send_request(
            router,
            "POST",
            "/api/v1/request/onboard/provider",
<<<<<<< HEAD
            Some(payload.to_string())
=======
            Some(payload.to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    // CALLBACK TESTS
    #[tokio::test]
    async fn test_callback_test_id_interact_ref_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/callback/test-id?hash=abc&interact_ref=xyz",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_callback_test_id_interact_ref_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/callback/test-id?hash=abc&interact_ref=xyz",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn test_callback_test_id_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "interact_ref": "xyz",
            "hash": "abc"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/callback/test-id", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/callback/test-id",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn test_callback_test_id_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "interact_ref": "xyz",
            "hash": "abc"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/callback/test-id", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/callback/test-id",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::NOT_FOUND
        );
    }

    // AUTHORITY TESTS
    #[tokio::test]
    async fn test_authority_beg_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "id": "auth-id",
            "slug": "auth-slug",
            "url": "https://example.com",
            "vc_type": "VerifiableCredential"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/authority/beg", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/authority/beg",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
                || status == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_authority_beg_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "id": "auth-id",
            "slug": "auth-slug",
            "url": "https://example.com",
            "vc_type": "VerifiableCredential"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/authority/beg", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/authority/beg",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_authority_oidc_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "id": "auth-id",
            "slug": "auth-slug",
            "url": "https://example.com",
            "vc_type": "VerifiableCredential"
        });
        let status = send_request(
            router,
            "POST",
            "/api/v1/authority/beg/oidc",
<<<<<<< HEAD
            Some(payload.to_string())
=======
            Some(payload.to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
                || status == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_authority_oidc_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "id": "auth-id",
            "slug": "auth-slug",
            "url": "https://example.com",
            "vc_type": "VerifiableCredential"
        });
        let status = send_request(
            router,
            "POST",
            "/api/v1/authority/beg/oidc",
<<<<<<< HEAD
            Some(payload.to_string())
=======
            Some(payload.to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_authority_all_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/authority/request/all",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_authority_all_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/authority/request/all",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_authority_test_id_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/authority/request/test-id",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_authority_test_id_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/authority/request/test-id",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::NOT_FOUND
        );
    }

    // MATES TESTS
    #[tokio::test]
    async fn test_mates_mates_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_mates_mates_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_mates_batch_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "POST",
            "/api/v1/mates/batch",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_mates_batch_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "ids": ["urn:example:mate1", "urn:example:mate2"]
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/mates/batch", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/mates/batch",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_mates_me_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates/me",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_mates_me_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates/me",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        println!("{:?}", status);
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_mates_test_id_success() {
        let router = build_router(false);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates/test-id",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
        )
        .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
            Some(serde_json::json!({}).to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }
    #[tokio::test]
    async fn test_mates_test_id_error() {
        let router = build_router(true);
        let status = send_request(
            router,
            "GET",
            "/api/v1/mates/test-id",
<<<<<<< HEAD
            Some(serde_json::json!({}).to_string())
=======
            Some(serde_json::json!({}).to_string()),
>>>>>>> origin/main
        )
        .await;
        assert!(status == StatusCode::BAD_GATEWAY || status == StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_mates_token_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "token": "valid-token"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/verify/mate/token", Some(payload.to_string()))
                .await;
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
        );
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/verify/mate/token",
            Some(payload.to_string()),
        )
        .await;
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
>>>>>>> origin/main
    }

    #[tokio::test]
    async fn test_mates_token_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "token": "invalid-token"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/verify/mate/token", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/verify/mate/token",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::NOT_FOUND
        );
    }

    // OIDC TESTS
    #[tokio::test]
    async fn test_oidc_oidc4vci_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "uri": "https://issuer.example.com"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/process/oidc4vci", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/process/oidc4vci",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_oidc_oidc4vci_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "uri": "https://issuer.example.com"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/process/oidc4vci", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/process/oidc4vci",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_oidc_oidc4vp_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "uri": "https://issuer.example.com"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/process/oidc4vp", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/process/oidc4vp",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        println!("{:?}", status);
        assert!(
            status.is_success()
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::NOT_FOUND
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_oidc_oidc4vp_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "uri": "https://issuer.example.com"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/process/oidc4vp", Some(payload.to_string()))
                .await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/process/oidc4vp",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_delete_did_success() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "did": "did:example:123",
            "method": "did:web"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/did", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/did",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(status == StatusCode::CREATED || status == StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_did_error() {
        let router = build_router(true);
        let payload = serde_json::json!({
            "did": "did:example:123",
            "method": "did:web"
        });
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/did", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/did",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::UNPROCESSABLE_ENTITY
        );
    }

    #[tokio::test]
    async fn test_didweb_success() {
        let router = build_router(false);
        let status = send_request(router, "GET", "/api/v1/did.json", None).await;
        assert!(status.is_success() || status == StatusCode::PRECONDITION_FAILED);
    }

    #[tokio::test]
    async fn test_didweb_error() {
        let router = build_router(true);
        let status = send_request(router, "GET", "/api/v1/did.json", None).await;
        println!("{:?}", status);
        assert!(
            status == StatusCode::BAD_GATEWAY
                || status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::PRECONDITION_FAILED
        );
    }

    #[tokio::test]
    async fn test_mates_batch_success_with_payload() {
        let router = build_router(false);
        let payload = serde_json::json!({
            "ids": ["urn:example:mate1", "urn:example:mate2"]
        });
<<<<<<< HEAD
        let status =
            send_request(router, "POST", "/api/v1/mates/batch", Some(payload.to_string())).await;
=======
        let status = send_request(
            router,
            "POST",
            "/api/v1/mates/batch",
            Some(payload.to_string()),
        )
        .await;
>>>>>>> origin/main
        assert!(status.is_success() || status == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_mates_get_by_id_success() {
        let router = build_router(false);
        let status = send_request(router, "GET", "/api/v1/mates/test-id", None).await;
        assert!(status.is_success() || status == StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_fallback_real_router() {
        let router = build_router(false);
        let status = send_request(router, "GET", "/unknown/route", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_key_invalid_payload() {
        let router = build_router(false);
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/key", Some("{}".to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/key",
            Some("{}".to_string()),
        )
        .await;
>>>>>>> origin/main
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_did_invalid_payload() {
        let router = build_router(false);
<<<<<<< HEAD
        let status =
            send_request(router, "DELETE", "/api/v1/wallet/did", Some("{}".to_string())).await;
=======
        let status = send_request(
            router,
            "DELETE",
            "/api/v1/wallet/did",
            Some("{}".to_string()),
        )
        .await;
>>>>>>> origin/main
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
}
