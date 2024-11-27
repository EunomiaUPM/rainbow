use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

mod http;
mod core;


pub static SSI_AUTH_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client")
});