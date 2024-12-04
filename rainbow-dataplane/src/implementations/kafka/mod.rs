use crate::core::{DataPlanePeer, DataPlanePeerCreationBehavior, PersistModel};
use crate::data::entities::data_plane_process;
use axum::async_trait;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use sea_orm::DbConn;
use std::collections::HashMap;

pub mod implementation;

pub struct KafkaDataPlane {
    pub inner: DataPlanePeer,
}

#[async_trait]
impl PersistModel<data_plane_process::Model> for KafkaDataPlane {
    async fn persist(self, db_connection: &DbConn) -> anyhow::Result<Box<Self>> {
        todo!()
    }
}

impl DataPlanePeerCreationBehavior for KafkaDataPlane {
    fn create_data_plane_peer() -> Self {
        Self { inner: DataPlanePeer::default() }
    }

    fn create_data_plane_peer_from_inner(inner: DataPlanePeer) -> Self {
        Self { inner }
    }

    fn with_role(mut self, role: ConfigRoles) -> Self {
        self.inner.role = role;
        self
    }

    fn with_local_address(mut self, local_address: String) -> Self {
        self.inner.local_address = Some(local_address);
        self
    }

    fn with_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.inner.attributes.extend(attributes);
        self
    }

    fn add_attribute(mut self, key: String, value: String) -> Self {
        self.inner.attributes.insert(key, value);
        self
    }

    fn delete_attribute(mut self, key: String) -> Self {
        self.inner.attributes.remove(&key);
        self
    }

    fn with_action(mut self, action: FormatAction) -> Self {
        self.inner.dct_formats.action = action;
        self
    }

    fn with_protocol(mut self, protocol: FormatProtocol) -> Self {
        self.inner.dct_formats.protocol = protocol;
        self
    }
}
