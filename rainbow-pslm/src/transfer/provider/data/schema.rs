// @generated automatically by Diesel CLI.

diesel::table! {
    transfer_messages (id) {
        id -> Uuid,
        transfer_process_id -> Uuid,
        created_at -> Timestamp,
        message_type -> Varchar,
        from -> Varchar,
        to -> Varchar,
        content -> Jsonb,
    }
}

diesel::table! {
    transfer_processes (provider_pid) {
        provider_pid -> Uuid,
        consumer_pid -> Uuid,
        agreement_id -> Uuid,
        data_plane_id -> Nullable<Uuid>,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(transfer_messages -> transfer_processes (transfer_process_id));

diesel::allow_tables_to_appear_in_same_query!(
    transfer_messages,
    transfer_processes,
);