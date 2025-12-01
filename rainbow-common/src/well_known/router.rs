use crate::well_known::dspace_version::WellKnownDSpaceVersionService;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

#[derive(Clone)]
pub struct WellKnownRouter {
    pub dspace_version_service: WellKnownDSpaceVersionService,
}

impl FromRef<WellKnownRouter> for WellKnownDSpaceVersionService {
    fn from_ref(state: &WellKnownRouter) -> Self {
        state.dspace_version_service.clone()
    }
}

impl WellKnownRouter {
    pub fn new(dspace_version_service: WellKnownDSpaceVersionService) -> WellKnownRouter {
        WellKnownRouter { dspace_version_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/.well-known/dspace-version",
                get(Self::handle_get_well_known_version),
            )
            .with_state(self)
    }

    async fn handle_get_well_known_version(State(state): State<WellKnownRouter>) -> impl IntoResponse {
        let response = state.dspace_version_service.get_dspace_version().unwrap();
        (StatusCode::OK, Json(response)).into_response()
    }
}
