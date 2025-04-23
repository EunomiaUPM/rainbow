use crate::proxy::http::HTTPProxyTrait;
use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use reqwest::Client;
use std::hash::Hash;

pub struct HTTPProxy {
    client: Client,
}

impl HTTPProxy {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl HTTPProxyTrait for HTTPProxy {
    type Service = HTTPProxy;

    fn router(self) -> Router {
        Router::new()
            .route("/data/pull/:data_plane_id", any(Self::pull_data))
            .with_state(self.client)
    }

    async fn pull_data(service: State<Self::Service>, req: Request) -> impl IntoResponse {}

    async fn push_data(service: State<Self::Service>, req: Request) -> impl IntoResponse {
        todo!()
    }
}
