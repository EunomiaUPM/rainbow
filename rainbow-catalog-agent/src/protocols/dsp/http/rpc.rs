use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use axum::extract::{FromRef, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct RpcRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
}

impl FromRef<RpcRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &RpcRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl RpcRouter {
    pub fn new(orchestrator: Arc<dyn OrchestratorTrait>) -> Self {
        Self { orchestrator }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/rpc/setup-catalog-request", get(Self::handle_rpc_catalog_request))
            .route("/rpc/setup-dataset-request", get(Self::handle_rpc_dataset_request))
            .with_state(self)
    }

    async fn handle_rpc_catalog_request(State(state): State<RpcRouter>) -> impl IntoResponse {
        "catalog"
    }
    async fn handle_rpc_dataset_request(State(state): State<RpcRouter>) -> impl IntoResponse {
        "dataset"
    }
}
