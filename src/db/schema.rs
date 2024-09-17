// @generated automatically by Diesel CLI.

diesel::table! {
    transfer_sessions (id) {
        id -> Uuid,
        provider_pid -> Uuid,
        consumer_pid -> Uuid,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
