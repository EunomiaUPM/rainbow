use axum::body::Body;
use axum::response::Response;
use reqwest::Response as ReqwestResponse;
pub async fn forward_response(reqwest_response: ReqwestResponse) -> Response {
    let status = reqwest_response.status();
    let headers = reqwest_response.headers().clone();
    let body_stream = reqwest_response.bytes_stream();
    let body = Body::from_stream(body_stream);
    let mut response = Response::builder().status(status);
    let response_headers = response.headers_mut().unwrap();
    for (key, value) in headers.iter() {
        response_headers.insert(key, value.clone());
    }

    response.body(body).unwrap()
}
