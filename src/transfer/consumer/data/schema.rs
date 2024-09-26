// @generated automatically by Diesel CLI.

diesel::table! {
    transfer_callbacks (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
