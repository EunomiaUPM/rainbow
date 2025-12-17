pub(crate) mod catalogs;
pub(crate) mod data_services;
pub(crate) mod datasets;
pub(crate) mod distributions;
pub(crate) mod odrl_policies;
pub(crate) mod policy_templates;

pub(crate) mod api {
    pub mod catalog_agent {
        tonic::include_proto!("catalog.v1");
    }

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("catalog_descriptor");
}
