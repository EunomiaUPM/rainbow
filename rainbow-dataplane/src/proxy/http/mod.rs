pub mod http;

use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::{async_trait, Router};

#[async_trait]
pub trait HTTPProxyTrait: Send + Sync {
    type Service;
    fn router(self) -> Router;
    async fn pull_data(service: State<Self::Service>, req: Request) -> impl IntoResponse;
    async fn push_data(service: State<Self::Service>, req: Request) -> impl IntoResponse;
}