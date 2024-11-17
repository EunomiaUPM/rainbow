// @generated automatically by Diesel CLI.

diesel::table! {
    transfer_callbacks (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        provider_pid -> Nullable<Uuid>,
        consumer_pid -> Nullable<Uuid>,
        data_address -> Nullable<Jsonb>,
    }
}
