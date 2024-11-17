use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

// https://chatgpt.com/share/67053458-225c-800f-8070-874516713bcf
pub static DATA_PLANE_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client")
});
