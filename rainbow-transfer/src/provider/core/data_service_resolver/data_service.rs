use crate::provider::core::data_service_resolver::DataServiceFacadeTrait;
use axum::async_trait;
use rainbow_common::protocol::catalog::dataservice_definition::{DataService, DataServiceDcatDeclaration, DataServiceDctDeclaration};
use urn::Urn;

pub struct DataServiceFacadeImpl {}

impl DataServiceFacadeImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DataServiceFacadeTrait for DataServiceFacadeImpl {
    async fn resolve_data_service_by_agreement_id(&self, agreement_id: Urn) -> anyhow::Result<DataService> {
        Ok(DataService {
            context: "".to_string(),
            _type: "".to_string(),
            id: "".to_string(),
            dcat: DataServiceDcatDeclaration {
                theme: "".to_string(),
                keyword: "".to_string(),
                endpoint_description: "".to_string(),
                endpoint_url: "https://jsonplaceholder.typicode.com/comments".to_string(),
            },
            dct: DataServiceDctDeclaration {
                conforms_to: None,
                creator: None,
                identifier: "".to_string(),
                issued: Default::default(),
                modified: None,
                title: None,
                description: vec![],
            },
            odrl_offer: Default::default(),
            extra_fields: Default::default(),
        })
    }
}
