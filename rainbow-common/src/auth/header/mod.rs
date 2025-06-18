use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub token: String,
}

pub async fn extract_request_info(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    debug!("Request info headers middleware");
    // 1. Extract headers
    let headers = request.headers();
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| "".to_string())
        .replace("Bearer ", "");
    // 2. Setup struct
    let request_info = RequestInfo { token };
    // 3. Insert into extensions
    request.extensions_mut().insert(Arc::new(request_info));
    // 4. Bye
    Ok(next.run(request).await)
}