// @generated automatically by Diesel CLI.

diesel::table! {
    protocol_versions (id) {
        id -> Uuid,
        path -> Varchar,
        version -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        lol -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    protocol_versions,
    users,
);
