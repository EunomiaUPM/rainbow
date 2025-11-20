pub mod api {
    pub mod transfer_messages {
        tonic::include_proto!("transfer_messages");
    }

    pub mod transfer_processes {
        tonic::include_proto!("transfer_processes");
    }

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("transfer_descriptor");
}

pub(crate) mod transfer_messages;
pub(crate) mod transfer_process;
