// @generated automatically by Diesel CLI.

diesel::table! {
    contract_agreements (agreement_id) {
        agreement_id -> Uuid,
        dataset_id -> Uuid,
    }
}
