use crate::data::entities::transfer_event;
use crate::data::entities::transfer_event::NewTransferEventModel;
use crate::data::repo_traits::transfer_event_repo::{TransferEventRepo, TransferEventRepoErrors};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::{Urn, UrnBuilder};

pub struct TransferEventRepoForSql {
    db_connection: DatabaseConnection,
}
impl TransferEventRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl TransferEventRepo for TransferEventRepoForSql {
    async fn get_all_transfer_events(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_event::Model>, TransferEventRepoErrors> {
        let events = transfer_event::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match events {
            Ok(events) => Ok(events),
            Err(e) => Err(TransferEventRepoErrors::ErrorFetchingTransferEvent(
                e.into(),
            )),
        }
    }

    async fn get_batch_transfer_events(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<transfer_event::Model>, TransferEventRepoErrors> {
        let ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let events =
            transfer_event::Entity::find().filter(transfer_event::Column::Id.is_in(ids)).all(&self.db_connection).await;
        match events {
            Ok(events) => Ok(events),
            Err(e) => Err(TransferEventRepoErrors::ErrorFetchingTransferEvent(
                e.into(),
            )),
        }
    }

    async fn get_all_transfer_events_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<transfer_event::Model>, TransferEventRepoErrors> {
        let pid = process_id.to_string();
        let fields = transfer_event::Entity::find()
            .filter(transfer_event::Column::DataplaneProcessId.eq(pid))
            .all(&self.db_connection)
            .await;

        match fields {
            Ok(fields) => Ok(fields),
            Err(e) => Err(TransferEventRepoErrors::ErrorFetchingTransferEvent(
                e.into(),
            )),
        }
    }

    async fn get_transfer_event_by_id(
        &self,
        transfer_event: &Urn,
    ) -> anyhow::Result<Option<transfer_event::Model>, TransferEventRepoErrors> {
        let transfer_event_id = transfer_event.to_string();
        let event = transfer_event::Entity::find_by_id(transfer_event_id).one(&self.db_connection).await;
        match event {
            Ok(event) => Ok(event),
            Err(e) => Err(TransferEventRepoErrors::ErrorFetchingTransferEvent(
                e.into(),
            )),
        }
    }

    async fn create_transfer_event(
        &self,
        data_plane_process: &Urn,
        new_transfer_event: &NewTransferEventModel,
    ) -> anyhow::Result<transfer_event::Model, TransferEventRepoErrors> {
        let data_plane_process = data_plane_process.to_string();
        let id = UrnBuilder::new("transfer-event", uuid::Uuid::new_v4().to_string().as_str())
            .build()
            .expect("UrnBuilder failed");

        let model = transfer_event::ActiveModel {
            id: ActiveValue::Set(id.to_string()),
            dataplane_process_id: ActiveValue::Set(data_plane_process.to_string()),
            from: ActiveValue::Set(new_transfer_event.from.to_string()),
            to: ActiveValue::Set(new_transfer_event.to.to_string()),
            payload: ActiveValue::Set(new_transfer_event.payload.clone()),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
        };

        let event = transfer_event::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match event {
            Ok(event) => Ok(event),
            Err(e) => {
                return Err(TransferEventRepoErrors::ErrorCreatingTransferEvent(
                    e.into(),
                ))
            }
        }
    }
}
