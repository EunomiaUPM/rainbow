use crate::transfer::provider::http::client::DATA_PLANE_HTTP_CLIENT;
use reqwest::Client;
use std::ops::Deref;

pub mod control_plane;
pub mod data_plane;
pub mod api;
pub mod protocol_transition_rules;

enum Clients {
    HttpClient,
}

pub fn get_current_data_plane_client() -> &'static Client {
    // TODO rethink for more than a client...
    match true {
        _ => &DATA_PLANE_HTTP_CLIENT,
    }
        .deref()
}
