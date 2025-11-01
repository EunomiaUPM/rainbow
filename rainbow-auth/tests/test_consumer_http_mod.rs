// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\http\mod.rs' 

#[cfg(test)]
mod tests {
    use axum::{Json, body::Body, extract::State, http::{Request, StatusCode}, response::IntoResponse};
    use chrono::NaiveDateTime;
    use chrono::NaiveDate;
    use chrono::NaiveTime;
    use sea_orm_migration::async_trait;
    use serde_json::json;
    use tracing::info;
    use std::{collections::HashMap, sync::Arc, usize::MAX};
    use tower::ServiceExt; // para `.oneshot()`
    use rainbow_auth::ssi_auth::{common::types::{entities::{ReachAuthority, ReachMethod, ReachProvider}, gnap::CallbackBody, ssi::{dids::DidsInfo, keys::{KeyDefinition, KeyInfo}}}, consumer::{core::Manager, http::RainbowAuthConsumerRouter}};
    use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
    use rainbow_common::{config::consumer_config::ApplicationConsumerConfig, errors::{CommonErrors, ErrorInfo, helpers::BadFormat}};
    use serde_json::Value;
    use axum::body::to_bytes;
    use axum::{extract::{Path, Query}};

    // Mocks

    #[derive(Clone)]
    struct MockRepoFactory;
    impl AuthRepoFactoryTrait for MockRepoFactory {
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


    #[derive(Clone)]
    struct MockManager {
        should_fail: bool,
    }

    impl MockManager {
        pub fn new(should_fail: bool) -> Arc<Self> {
            Arc::new(Self { should_fail })
        }
        

        pub async fn register_wallet(&self) -> Result<(), CommonErrors> {
            if self.should_fail {
                Err(CommonErrors::ConsumerError {
                    info: ErrorInfo {
                        message: "Registro fallido".to_string(),
                        error_code: 1001,
                        status_code: StatusCode::BAD_REQUEST,
                        details: Some("Simulación de error en el registro".to_string()),
                    },
                    http_code: Some(400),
                    url: Some("/api/v1/wallet/register".to_string()),
                    method: Some("POST".to_string()),
                    cause: Some("Simulación de error".to_string()),
                })
            } else {
                Ok(())
            }
        }
 
        pub async fn login_wallet(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Login fallido".to_string(),
                            error_code: 1002,
                            status_code: StatusCode::UNAUTHORIZED,
                            details: Some("Credenciales inválidas".to_string()),
                        },
                        http_code: Some(401),
                        url: Some("/api/v1/wallet/login".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }

        pub async fn logout_wallet(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Logout fallido".to_string(),
                            error_code: 1003,
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            details: Some("Error inesperado al cerrar sesión".to_string()),
                        },
                        http_code: Some(500),
                        url: Some("/api/v1/wallet/logout".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }

        pub async fn onboard_wallet(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Onboarding fallido".to_string(),
                            error_code: 1004,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Error al iniciar el onboarding".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/onboard".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }
        
        pub async fn partial_onboard(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Partial onboarding fallido".to_string(),
                            error_code: 1005,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Error al realizar el onboarding parcial".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/partial-onboard".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }
                
        pub async fn register_key(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Registro de clave fallido".to_string(),
                            error_code: 1006,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Error al registrar la clave".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/key".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }
        
        pub async fn register_did(&self) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Registro de DID fallido".to_string(),
                            error_code: 1007,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Error al registrar el DID".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/did".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }

        pub async fn delete_key(&self, _payload: KeyDefinition) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error al eliminar la clave".to_string(),
                            error_code: 1009,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Clave no encontrada".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/key".to_string()),
                        method: Some("DELETE".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }
        
        pub async fn delete_did(&self, _payload: DidsInfo) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error al eliminar el DID".to_string(),
                            error_code: 1010,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("DID no válido o no encontrado".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/wallet/did".to_string()),
                        method: Some("DELETE".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }
        
        pub async fn get_did_doc(&self) -> Result<Value, CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error al obtener el documento DID".to_string(),
                            error_code: 1012,
                            status_code: StatusCode::NOT_FOUND,
                            details: Some("DID no encontrado".to_string()),
                        },
                        http_code: Some(404),
                        url: Some("/api/v1/did.json".to_string()),
                        method: Some("GET".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(json!({
                        "@context": "https://www.w3.org/ns/did/v1",
                        "id": "did:example:123456789abcdefghi",
                        "verificationMethod": [{
                            "id": "did:example:123456789abcdefghi#keys-1",
                            "type": "Ed25519VerificationKey2018",
                            "controller": "did:example:123456789abcdefghi",
                            "publicKeyBase58": "H3C2AVvLMfQ9c..."
                        }]
                    }))
                }
            }

        pub async fn request_onboard_provider(
                &self,
                url: String,
                _id: String,
                slug: String,
            ) -> Result<String, CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error al solicitar el onboarding del proveedor".to_string(),
                            error_code: 1013,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Proveedor no válido".to_string()),
                        },
                        http_code: Some(400),
                        url: Some(url),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(format!("{}/onboard/{}", url, slug))
                }
            }

        pub async fn check_callback(&self, _id: String, _interact_ref: String, _hash: String) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error en check_callback".to_string(),
                            error_code: 1014,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Hash inválido".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/callback".to_string()),
                        method: Some("GET".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }

        pub async fn continue_request(&self, _id: String, _interact_ref: String) -> Result<Value, CommonErrors> {
            if self.should_fail {
                Err(CommonErrors::ConsumerError {
                    info: ErrorInfo {
                        message: "Error en continue_request".to_string(),
                        error_code: 1015,
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        details: Some("No se pudo continuar".to_string()),
                    },
                    http_code: Some(500),
                    url: Some("/api/v1/callback".to_string()),
                    method: Some("GET".to_string()),
                    cause: Some("Simulación de error".to_string()),
                })
            } else {
                Ok(json!({ "status": "ok", "id": _id }))
            }
        }
        
        pub async fn beg_credential(
                &self,
                _payload: ReachAuthority,
                _method: ReachMethod,
            ) -> Result<(), CommonErrors> {
                if self.should_fail {
                    Err(CommonErrors::ConsumerError {
                        info: ErrorInfo {
                            message: "Error al solicitar credencial".to_string(),
                            error_code: 1018,
                            status_code: StatusCode::BAD_REQUEST,
                            details: Some("Autoridad no válida".to_string()),
                        },
                        http_code: Some(400),
                        url: Some("/api/v1/authority/beg".to_string()),
                        method: Some("POST".to_string()),
                        cause: Some("Simulación de error".to_string()),
                    })
                } else {
                    Ok(())
                }
            }

            

    }    

    // Handler 

    async fn wallet_register(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn wallet_login(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => (&e).into_response(),
        }
    }
    
    async fn wallet_logout(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn wallet_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.onboard_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn partial_onboard(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.partial_onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }
    
    async fn register_key(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn register_did(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn delete_key(State(manager): State<Arc<MockManager>>, Json(payload): Json<KeyDefinition>) -> impl IntoResponse {
        match manager.delete_key(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }
 
    async fn delete_did(State(manager): State<Arc<MockManager>>, Json(payload): Json<DidsInfo>) -> impl IntoResponse {
        match manager.delete_did(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => (&e).into_response(),
        }
    }
    
    async fn didweb(State(manager): State<Arc<MockManager>>) -> impl IntoResponse {
        match manager.get_did_doc().await {
            Ok(did) => Json(did).into_response(),
            Err(e) => (&e).into_response(),
        }
    }

    async fn request_provider_onboard(
        State(manager): State<Arc<MockManager>>,
        Json(payload): Json<ReachProvider>,
    ) -> impl IntoResponse {
        let uri = match manager
            .request_onboard_provider(payload.url.clone(), payload.id.clone(), payload.slug.clone())
            .await
        {
            Ok(uri) => uri,
            Err(e) => return (&e).into_response(),
        };
        uri.into_response()
    }

    async fn get_callback(
        State(manager): State<Arc<MockManager>>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let hash = match params.get("hash") {
            Some(h) => h,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Received,
                    Some("Unable to retrieve hash from callback".to_string()),
                );
                return error.into_response();
            }
        };

        let interact_ref = match params.get("interact_ref") {
            Some(i) => i,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Received,
                    Some("Unable to retrieve interact reference".to_string()),
                );
                return error.into_response();
            }
        };

        if let Err(e) = manager.check_callback(id.clone(), interact_ref.to_string(), hash.to_string()).await {
            return e.into_response();
        }

        match manager.continue_request(id, interact_ref.to_string()).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.into_response(),
        }
    }

    async fn post_callback(
        State(manager): State<Arc<MockManager>>,
        Path(id): Path<String>,
        Json(payload): Json<CallbackBody>,
    ) -> impl IntoResponse {
        if let Err(e) = manager
            .check_callback(id.clone(), payload.interact_ref.clone(), payload.hash.clone())
            .await
        {
            return (&e).into_response();
        }

        match manager.continue_request(id, payload.interact_ref.clone()).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => (&e).into_response(),
        }
    }
 
    async fn beg4credential(
        State(manager): State<Arc<MockManager>>,
        Json(payload): Json<ReachAuthority>,
    ) -> impl IntoResponse {
        match manager.beg_credential(payload, ReachMethod::CrossUser).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => (&e).into_response(),
        }
    }
    
    async fn beg4credential_oidc(
            State(manager): State<Arc<MockManager>>,
            Json(payload): Json<ReachAuthority>,
        ) -> impl IntoResponse {
            info!("POST /beg/credential");
            match manager.beg_credential(payload, ReachMethod::Oidc).await {
                Ok(data) => data.into_response(),
                Err(e) => e.into_response(),
            }
        }


    //Tests 
    
    #[tokio::test]
    async fn test_wallet_login_route_success() {
        let repo = Arc::new(MockRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

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

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invalid_route_returns_404() {
        let repo = Arc::new(MockRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/wallet/invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_wallet_register_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/register", axum::routing::post(wallet_register))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/register")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_wallet_register_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/register", axum::routing::post(wallet_register))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/register")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_wallet_login_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/login", axum::routing::post(wallet_login))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/login")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_wallet_login_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/login", axum::routing::post(wallet_login))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/login")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[tokio::test]
    async fn test_wallet_logout_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/logout", axum::routing::post(wallet_logout))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/logout")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_wallet_logout_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/logout", axum::routing::post(wallet_logout))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/logout")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_wallet_onboard_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/onboard", axum::routing::post(wallet_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/onboard")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_wallet_onboard_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/onboard", axum::routing::post(wallet_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/onboard")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_partial_onboard_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/partial-onboard", axum::routing::post(partial_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/partial-onboard")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_partial_onboard_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/partial-onboard", axum::routing::post(partial_onboard))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/partial-onboard")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_key_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/key", axum::routing::post(register_key))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/key")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_key_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/key", axum::routing::post(register_key))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/key")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_did_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/did", axum::routing::post(register_did))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/did")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_did_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/did", axum::routing::post(register_did))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/wallet/did")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_key_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/key", axum::routing::delete(delete_key))
            .with_state(manager);

        let payload = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "MockProvider".to_string(),
            key_id: KeyInfo {
                id: "key123".to_string(),
            },
            key_pair: json!({"public": "ABCDEF123456"}),
            keyset_handle: None,
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/key")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_key_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/key", axum::routing::delete(delete_key))
            .with_state(manager);

        let payload = KeyDefinition {
            algorithm: "Ed25519".to_string(),
            crypto_provider: "MockProvider".to_string(),
            key_id: KeyInfo {
                id: "key123".to_string(),
            },
            key_pair: json!({"public": "ABCDEF123456"}),
            keyset_handle: None,
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/key")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_did_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/wallet/did", axum::routing::delete(delete_did))
            .with_state(manager);

        let payload = DidsInfo {
            did: "did:example:123456789abcdefghi".to_string(),
            alias: "test-alias".to_string(),
            document: "{}".to_string(),
            key_id: "key123".to_string(),
            default: false,
            created_on: "2023-01-01T00:00:00Z".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/did")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_did_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/wallet/did", axum::routing::delete(delete_did))
            .with_state(manager);

        let payload = DidsInfo {
            did: "did:example:invalid".to_string(),
            alias: "invalid-alias".to_string(),
            document: "{}".to_string(),
            key_id: "invalid-key".to_string(),
            default: false,
            created_on: "2023-01-01T00:00:00Z".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/wallet/did")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_didweb_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/did.json", axum::routing::get(didweb))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/v1/did.json")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_didweb_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/did.json", axum::routing::get(didweb))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/v1/did.json")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_request_provider_onboard_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/auth/manual/ssi", axum::routing::post(request_provider_onboard))
            .with_state(manager);

        let payload = ReachProvider {
            id: "provider123".to_string(),
            slug: "test-provider".to_string(),
            url: "http://provider.com".to_string(),
            actions: "onboard".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/manual/ssi")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), MAX).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, "http://provider.com/onboard/test-provider");
    }

    #[tokio::test]
    async fn test_request_provider_onboard_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/auth/manual/ssi", axum::routing::post(request_provider_onboard))
            .with_state(manager);

        let payload = ReachProvider {
                id: "provider123".to_string(),
                slug: "test-provider".to_string(),
                url: "http://provider.com".to_string(),
                actions: "onboard".to_string(),
            };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/manual/ssi")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_callback_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/callback/:id", axum::routing::get(get_callback))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/v1/callback/test-id?hash=abc123&interact_ref=xyz789")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_callback_missing_hash() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/callback/:id", axum::routing::get(get_callback))
            .with_state(manager);

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/v1/callback/test-id?interact_ref=xyz789")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_post_callback_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/callback/:id", axum::routing::post(post_callback))
            .with_state(manager);

        let payload = CallbackBody {
            interact_ref: "xyz789".to_string(),
            hash: "abc123".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/callback/test-id")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post_callback_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/callback/:id", axum::routing::post(post_callback))
            .with_state(manager);

        let payload = CallbackBody {
            interact_ref: "xyz789".to_string(),
            hash: "abc123".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/callback/test-id")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_beg4credential_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/authority/beg", axum::routing::post(beg4credential))
            .with_state(manager);

        let payload = ReachAuthority {
                id: "auth123".to_string(),
                slug: "test-slug".to_string(),
                url: "http://authority.com".to_string(),
                vc_type: "VerifiableCredential".to_string(),
            };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/authority/beg")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_beg4credential_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/authority/beg", axum::routing::post(beg4credential))
            .with_state(manager);

        let payload = ReachAuthority {
            id: "auth123".to_string(),
            slug: "test-slug".to_string(),
            url: "http://authority.com".to_string(),
            vc_type: "VerifiableCredential".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/authority/beg")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_beg4credential_oidc_success() {
        let manager = MockManager::new(false);
        let router = axum::Router::new()
            .route("/api/v1/authority/beg/oidc", axum::routing::post(beg4credential_oidc))
            .with_state(manager);

        let payload = ReachAuthority {
            id: "auth123".to_string(),
            slug: "test-slug".to_string(),
            url: "http://authority.com".to_string(),
            vc_type: "VerifiableCredential".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/authority/beg/oidc")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_beg4credential_oidc_error() {
        let manager = MockManager::new(true);
        let router = axum::Router::new()
            .route("/api/v1/authority/beg/oidc", axum::routing::post(beg4credential_oidc))
            .with_state(manager);

        let payload = ReachAuthority {
            id: "auth123".to_string(),
            slug: "test-slug".to_string(),
            url: "http://authority.com".to_string(),
            vc_type: "VerifiableCredential".to_string(),
        };

        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/authority/beg/oidc")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_all_authority_success() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;
        use axum::http::StatusCode;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use rainbow_db::auth_consumer::repo_factory::traits::*;
        use rainbow_db::auth_consumer::entities::authority_request::{Model, NewModel};
        use rainbow_db::common::BasicRepoTrait;
        use anyhow::Result;

        struct MockAuthorityRepo;

        #[async_trait::async_trait]
        impl BasicRepoTrait<Model, NewModel> for MockAuthorityRepo {
            async fn get_all(&self, _limit: Option<u64>, _offset: Option<u64>) -> Result<Vec<Model>> {
                Ok(vec![Model {
                    id: "auth1".into(),
                    authority_id: "auth-id".into(),
                    authority_slug: "slug".into(),
                    grant_endpoint: "http://authority.com/grant".into(),
                    vc_type: "VerifiableCredential".into(),
                    assigned_id: Some("assigned123".into()),
                    vc_uri: Some("http://vc.uri".into()),
                    status: "pending".into(),      
                    created_at: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 11, 14).unwrap(),
                        NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                    ),
                    ended_at: None,
                }])
            }

            async fn get_by_id(&self, _id: &str) -> Result<Option<Model>> {
                todo!()
            }

            async fn create(&self, _model: NewModel) -> Result<Model> {
                todo!()
            }

            async fn update(&self, _model: Model) -> Result<Model> {
                todo!()
            }

            async fn delete(&self, _id: &str) -> Result<()> {
                todo!()
            }
        }

        #[async_trait::async_trait]
        impl AuthorityRequestRepoTrait for MockAuthorityRepo {}

        #[derive(Clone)]
        struct MockRepoFactory;

        impl AuthRepoFactoryTrait for MockRepoFactory {
            fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
                Arc::new(MockAuthorityRepo)
            }
            fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { todo!() }
            fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { todo!() }
            fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { todo!() }
            fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { todo!() }
            fn mates(&self) -> Arc<dyn MatesRepoTrait> { todo!() }
        }

        let repo = Arc::new(MockRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/authority/request/all")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_all_authority_error() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;
        use axum::http::StatusCode;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use rainbow_db::auth_consumer::repo_factory::traits::*;
        use rainbow_db::auth_consumer::entities::authority_request::{Model, NewModel};
        use rainbow_db::common::BasicRepoTrait;
        use anyhow::{Result, anyhow};

        struct FailingAuthorityRepo;

        #[async_trait::async_trait]
        impl BasicRepoTrait<Model, NewModel> for FailingAuthorityRepo {
            async fn get_all(&self, _limit: Option<u64>, _offset: Option<u64>) -> Result<Vec<Model>> {
                Err(anyhow!("Simulated DB error"))
            }

            async fn get_by_id(&self, _id: &str) -> Result<Option<Model>> {
                todo!()
            }

            async fn create(&self, _model: NewModel) -> Result<Model> {
                todo!()
            }

            async fn update(&self, _model: Model) -> Result<Model> {
                todo!()
            }

            async fn delete(&self, _id: &str) -> Result<()> {
                todo!()
            }
        }

        #[async_trait::async_trait]
        impl AuthorityRequestRepoTrait for FailingAuthorityRepo {}

        #[derive(Clone)]
        struct FailingRepoFactory;

        impl AuthRepoFactoryTrait for FailingRepoFactory {
            fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
                Arc::new(FailingAuthorityRepo)
            }
            fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { todo!() }
            fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { todo!() }
            fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { todo!() }
            fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { todo!() }
            fn mates(&self) -> Arc<dyn MatesRepoTrait> { todo!() }
        }

        let repo = Arc::new(FailingRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/authority/request/all")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_one_authority_success() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;
        use axum::http::StatusCode;
        use std::sync::Arc;
        use chrono::NaiveDateTime;
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use rainbow_db::auth_consumer::repo_factory::traits::*;
        use rainbow_db::auth_consumer::entities::authority_request::{Model, NewModel};
        use rainbow_db::common::BasicRepoTrait;
        use anyhow::Result;

        struct MockAuthorityRepo;

        #[async_trait::async_trait]
        impl BasicRepoTrait<Model, NewModel> for MockAuthorityRepo {
            async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<Model>> {
                Ok(vec![])
            }

            async fn get_by_id(&self, id: &str) -> Result<Option<Model>> {
                Ok(Some(Model {
                    id: id.to_string(),
                    authority_id: "auth-id".into(),
                    authority_slug: "slug".into(),
                    grant_endpoint: "http://authority.com/grant".into(),
                    vc_type: "VerifiableCredential".into(),
                    assigned_id: Some("assigned123".into()),
                    vc_uri: Some("http://vc.uri".into()),
                    status: "pending".into(),      
                    created_at: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 11, 14).unwrap(),
                        NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                    ),
                    ended_at: None,
                }))
            }

            async fn create(&self, _: NewModel) -> Result<Model> { todo!() }
            async fn update(&self, _: Model) -> Result<Model> { todo!() }
            async fn delete(&self, _: &str) -> Result<()> { todo!() }
        }

        #[async_trait::async_trait]
        impl AuthorityRequestRepoTrait for MockAuthorityRepo {}

        #[derive(Clone)]
        struct MockRepoFactory;

        impl AuthRepoFactoryTrait for MockRepoFactory {
            fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
                Arc::new(MockAuthorityRepo)
            }
            fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { todo!() }
            fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { todo!() }
            fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { todo!() }
            fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { todo!() }
            fn mates(&self) -> Arc<dyn MatesRepoTrait> { todo!() }
        }

        let repo = Arc::new(MockRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/authority/request/auth1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_one_authority_not_found() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;
        use axum::http::StatusCode;
        use std::sync::Arc;
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
        use rainbow_auth::ssi_auth::consumer::core::Manager;
        use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
        use rainbow_db::auth_consumer::repo_factory::traits::*;
        use rainbow_db::auth_consumer::entities::authority_request::{Model, NewModel};
        use rainbow_db::common::BasicRepoTrait;
        use anyhow::Result;

        struct NotFoundAuthorityRepo;

        #[async_trait::async_trait]
        impl BasicRepoTrait<Model, NewModel> for NotFoundAuthorityRepo {
            async fn get_all(&self, _: Option<u64>, _: Option<u64>) -> Result<Vec<Model>> {
                Ok(vec![])
            }

            async fn get_by_id(&self, _: &str) -> Result<Option<Model>> {
                Ok(None)
            }

            async fn create(&self, _: NewModel) -> Result<Model> { todo!() }
            async fn update(&self, _: Model) -> Result<Model> { todo!() }
            async fn delete(&self, _: &str) -> Result<()> { todo!() }
        }

        #[async_trait::async_trait]
        impl AuthorityRequestRepoTrait for NotFoundAuthorityRepo {}

        #[derive(Clone)]
        struct NotFoundRepoFactory;

        impl AuthRepoFactoryTrait for NotFoundRepoFactory {
            fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
                Arc::new(NotFoundAuthorityRepo)
            }
            fn request(&self) -> Arc<dyn AuthRequestRepoTrait> { todo!() }
            fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> { todo!() }
            fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> { todo!() }
            fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> { todo!() }
            fn mates(&self) -> Arc<dyn MatesRepoTrait> { todo!() }
        }

        let repo = Arc::new(NotFoundRepoFactory);
        let config = ApplicationConsumerConfig::default();
        let manager = Arc::new(Manager::new(repo, config));
        let router = RainbowAuthConsumerRouter::new(manager).router();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/authority/request/unknown-id")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_fallback_returns_404_with_message() {
        use axum::http::{Method, Uri, StatusCode};
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;

        let method = Method::GET;
        let uri: Uri = "/nonexistent/route".parse().unwrap();

        let (status, message) = RainbowAuthConsumerRouter::<MockRepoFactory>::fallback(method, uri.clone()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, format!("No route for {uri}"));
    }

    #[tokio::test]
    async fn test_fallback_with_post_method() {
        use axum::http::{Method, Uri, StatusCode};
        use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;

        let method = Method::POST;
        let uri: Uri = "/api/v1/unknown".parse().unwrap();

        let (status, message) = RainbowAuthConsumerRouter::<MockRepoFactory>::fallback(method, uri.clone()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, format!("No route for {uri}"));
    }
}