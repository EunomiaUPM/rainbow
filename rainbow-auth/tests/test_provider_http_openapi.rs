// Tests corresponding to 'rainbow-auth\src\ssi_auth\provider\http\openapi.rs'

#[cfg(test)]
mod tests {
    use std::usize::MAX;

    use axum::{body::to_bytes, response::IntoResponse};
    use axum::http::StatusCode;
    use rainbow_auth::ssi_auth::consumer::http::openapi::{get_open_api, route_openapi};
    use tower::ServiceExt;

    //Mock

    async fn get_open_api_invalid() -> impl IntoResponse {
        (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            b"not a json", // contenido inválido
        ).into_response()
    }

    //Test

    #[tokio::test]
    async fn test_route_openapi_success() {
        use axum::{body::Body, http::Request};
        let app = route_openapi();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/auth/openapi.json")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_route_openapi_not_found() {
        use axum::{body::Body, http::Request};
        let app = route_openapi();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/auth/invalid")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_get_open_api_success() {
        let response = get_open_api().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "application/json"
        );

        let body = to_bytes(response.into_body(), MAX).await.unwrap();
        assert!(body.starts_with(b"{")); // Asumiendo que el JSON empieza con '{'
    }

    #[tokio::test]
    async fn test_get_open_api_invalid_json() {
        use axum::response::IntoResponse;

        let response = get_open_api_invalid().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "application/json"
        );

        let body = to_bytes(response.into_body(), MAX).await.unwrap();
        assert_eq!(body.as_ref(), b"not a json");
    }
}