use crate::entities::data_plane_process::DataPlaneProcessEntitiesTrait;
use crate::entities::transfer_events::TransferEventEntitiesTrait;
use crate::errors::error_adapter::CustomToResponse;
use crate::http::common::parse_urn;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;

#[derive(Clone)]
pub struct TransferEventsRouter {
    data_plane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
    transfer_event_entity: Arc<dyn TransferEventEntitiesTrait>,
}

impl FromRef<TransferEventsRouter> for Arc<dyn DataPlaneProcessEntitiesTrait> {
    fn from_ref(state: &TransferEventsRouter) -> Self {
        state.data_plane_process_entity.clone()
    }
}

impl FromRef<TransferEventsRouter> for Arc<dyn TransferEventEntitiesTrait> {
    fn from_ref(state: &TransferEventsRouter) -> Self {
        state.transfer_event_entity.clone()
    }
}

impl TransferEventsRouter {
    pub fn new(
        data_plane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
        transfer_event_entity: Arc<dyn TransferEventEntitiesTrait>,
    ) -> Self {
        Self { data_plane_process_entity, transfer_event_entity }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/data-plane/:data_plane_id", get(Self::handle_get_data_plane_by_id))
            .route("/:transfer_id", get(Self::handle_get_by_id))
            .with_state(self)
    }
    async fn handle_get_data_plane_by_id(
        State(state): State<TransferEventsRouter>,
        Path(data_plane_id): Path<String>,
    ) -> impl IntoResponse {
        let data_plane_id = match parse_urn(&data_plane_id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.transfer_event_entity.get_transfer_events_by_process_id(&data_plane_id).await {
            Ok(dataplane_session) => (StatusCode::OK, Json(dataplane_session)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn handle_get_by_id(
        State(state): State<TransferEventsRouter>,
        Path(transfer_id): Path<String>,
    ) -> impl IntoResponse {
        let transfer_id = match parse_urn(&transfer_id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.transfer_event_entity.get_transfer_event_by_id(&transfer_id).await {
            Ok(transfer_event) => match transfer_event {
                Some(transfer_event) => (StatusCode::OK, Json(transfer_event)).into_response(),
                None => {
                    let err = CommonErrors::missing_resource_new(
                        transfer_id.to_string().as_str(),
                        "Transfer event not found",
                    );
                    tracing::error!("{}", err.log());
                    err.into_response()
                }
            },
            Err(e) => e.to_response(),
        }
    }
}
