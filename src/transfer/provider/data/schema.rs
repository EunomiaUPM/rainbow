// @generated automatically by Diesel CLI.

diesel::table! {
    transfer_processes (id) {
        id -> Uuid,
        provider_pid -> Uuid,
        consumer_pid -> Uuid,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transfer_messages (id) {
        id -> Uuid,
        transfer_process_id -> Uuid,
        created_at -> Timestamp,
        message_type -> Varchar
    }
}

diesel::table! {
    transfer_message_fields (id) {
        id -> Uuid,
        transfer_message_id -> Uuid,
        key -> Varchar,
        value -> Varchar,
        parent -> Nullable<Uuid>
    }
}

diesel::joinable!(transfer_messages -> transfer_processes(id));
diesel::joinable!(transfer_message_fields -> transfer_messages(id));
diesel::allow_tables_to_appear_in_same_query!(
    transfer_processes,
    transfer_messages,
    transfer_message_fields
);

