// @generated automatically by Diesel CLI.

diesel::table! {
    data_plane_processes (data_plane_id) {
        data_plane_id -> Uuid,
        transfer_process_id -> Uuid,
        agreement_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        state -> Bool,
    }
}

diesel::table! {
    transfer_messages (id) {
        id -> Uuid,
        transfer_process_id -> Uuid,
        created_at -> Timestamp,
        message_type -> Varchar,
        content -> Jsonb,
    }
}

diesel::table! {
    transfer_processes (provider_pid) {
        provider_pid -> Uuid,
        consumer_pid -> Uuid,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(data_plane_processes -> transfer_processes (transfer_process_id));
diesel::joinable!(transfer_messages -> transfer_processes (transfer_process_id));

diesel::allow_tables_to_appear_in_same_query!(
    data_plane_processes,
    transfer_messages,
    transfer_processes,
);
