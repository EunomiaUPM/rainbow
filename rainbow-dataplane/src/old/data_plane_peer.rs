use crate::core::DataPlanePeerCreationBehavior;
use anyhow::bail;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::dataplane::repo::DATA_PLANE_REPO;
use std::collections::HashMap;
use urn::Urn;

#[derive(Debug)]
pub struct DataPlanePeer {
    pub id: Urn,
    pub role: ConfigRoles,
    pub local_address: Option<String>,
    pub dct_formats: DctFormats,
    pub attributes: HashMap<String, String>,
}

impl DataPlanePeerCreationBehavior for DataPlanePeer {
    fn create_data_plane_peer() -> Self {
        Self::default()
    }

    fn create_data_plane_peer_from_inner(inner: DataPlanePeer) -> Self {
        Self::default()
    }

    fn with_role(mut self, role: ConfigRoles) -> Self {
        self.role = role;
        self
    }

    fn with_local_address(mut self, local_address: String) -> Self {
        self.local_address = Some(local_address);
        self
    }

    fn with_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.attributes.extend(attributes);
        self
    }

    fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    fn delete_attribute(mut self, key: String) -> Self {
        self.attributes.remove(&key);
        self
    }

    fn with_action(mut self, action: FormatAction) -> Self {
        self.dct_formats.action = action;
        self
    }

    fn with_protocol(mut self, protocol: FormatProtocol) -> Self {
        self.dct_formats.protocol = protocol;
        self
    }
}

impl Default for DataPlanePeer {
    fn default() -> DataPlanePeer {
        Self {
            id: get_urn(None),
            role: ConfigRoles::Consumer,
            local_address: None,
            dct_formats: DctFormats {
                protocol: FormatProtocol::NgsiLd,
                action: FormatAction::Pull,
            },
            attributes: HashMap::new(),
        }
    }
}

impl DataPlanePeer {
    pub(crate) async fn load_model_by_id(id: Urn) -> anyhow::Result<Box<Self>> {
        let peer = match DATA_PLANE_REPO.get_data_plane_process_by_id(id.clone()).await? {
            Some(peer) => peer,
            None => bail!("Could not find dataPlaneDataPlan with id {}", id),
        };

        let mut fw_peer = Self {
            id: get_urn_from_string(&peer.id)?,
            role: peer.role.parse()?,
            local_address: Option::from(peer.address),
            dct_formats: DctFormats {
                protocol: peer.dct_action_protocol.parse()?,
                action: peer.dct_action_format.parse()?,
            },
            attributes: Default::default(),
        };

        let attributes = DATA_PLANE_REPO.get_all_data_plane_fields_by_process(id).await?;
        for attr in attributes {
            fw_peer = fw_peer.add_attribute(attr.key.to_string(), attr.value.to_string());
        }

        Ok(Box::new(fw_peer))
    }
}
