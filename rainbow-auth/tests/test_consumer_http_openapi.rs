// Tests corresponding to 'rainbow-auth\src\ssi_auth\consumer\http\openapi.rs' 

#[cfg(test)]
mod tests {
    use actix_web::{test, App, HttpResponse, http::StatusCode, web};
    use utoipa_swagger_ui::{Config, serve};
    use std::sync::Arc;
    use rainbow_auth::ssi_auth::consumer::http::openapi::{OPENAPI_JSON, get_open_api, route_openapi};
    use axum::{
        response::IntoResponse,
    };
    
    #[tokio::test]
    async fn test_openapi_json_route() {
        
        use axum::{
            body::to_bytes,
            http::{Request, StatusCode},
        };
        use tower::ServiceExt;

        let app = route_openapi();

        let request = Request::builder()
            .uri("/api/v1/auth/openapi.json")
            .method("GET")
            .body(axum::body::Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let content_type = response.headers().get("Content-Type").unwrap();
        assert_eq!(content_type, "application/json");

        let body_bytes = to_bytes(response.into_body(), 100000000).await.unwrap();
        assert_eq!(body_bytes.as_ref(), OPENAPI_JSON.as_bytes());
    }


    #[tokio::test]
    async fn test_openapi_not_found() {
        let config = Arc::new(Config::from("/api/v1/auth/openapi.json"));
        
        let app = test::init_service(
            App::new().route("/api/v1/auth/openapi.json", web::get().to(move || {
                let config = config.clone();
                async move {
                    match serve("/api/v1/auth/openapi.json", config) {
                        Ok(Some(file)) => HttpResponse::Ok()
                            .content_type(file.content_type)
                            .body(file.bytes.to_vec()),
                        _ => HttpResponse::NotFound().finish(),
                    }
                }
            })),
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/auth/openapi.json")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_open_api_status_code() {
        let response = get_open_api().await.into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_open_api_content_and_header() {
        use axum::http::HeaderValue;
        use axum::body::to_bytes;

        let response = get_open_api().await.into_response();

        let content_type = response.headers().get("Content-Type").unwrap();
        assert_eq!(content_type, &HeaderValue::from_static("application/json"));

        let body_bytes = to_bytes(response.into_body(), 100000).await.unwrap();
        assert_eq!(body_bytes.as_ref(), OPENAPI_JSON.as_bytes());
    }    
}
