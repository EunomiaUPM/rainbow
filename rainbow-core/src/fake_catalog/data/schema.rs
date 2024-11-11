// @generated automatically by Diesel CLI.

diesel::table! {
    dataset_catalogs (dataset_id) {
        dataset_id -> Uuid,
        dataset_endpoint -> Varchar,
    }
}
