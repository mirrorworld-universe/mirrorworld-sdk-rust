use reqwest::Client;
use reqwest::header::HeaderMap;

pub const STAGING_REQUEST_URL: &str = "https://api-staging.mirrorworld.fun";
pub const RELEASE_REQUEST_URL: &str = "https://api.mirrorworld.fun";

pub const DEVNET_ENV : &str = "devnet";
pub const MAINNET_ENV: &str = "mainnet";

pub const X_API_KEY: &str = "";



#[derive(Copy, Clone)]
pub enum NET_ENV {
    DEVNET,
    MAINNET,
}

pub fn get_env_name<>(net: NET_ENV<>) -> String {
    let s = match net {
        NET_ENV::DEVNET => {DEVNET_ENV.to_string()},
        NET_ENV::MAINNET => {MAINNET_ENV.to_string()},
        _ => {"".to_string()},
    };
    s
}

pub fn get_basic_url(net: NET_ENV) -> String {
    let s = match net {
        NET_ENV::DEVNET => {STAGING_REQUEST_URL.to_string()},
        NET_ENV::MAINNET => {RELEASE_REQUEST_URL.to_string()},
        _ => {"".to_string()}
    };
    s
}

pub fn get_request_header(api: String, token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-api-key", api.parse().unwrap());

    let mut authorization: String = "Bearer ".to_string() + &token;
    headers.insert("authorization", authorization.parse().unwrap());

    headers
}

pub mod accessors;
pub mod authentication;
pub mod event;
pub mod marketplace;
pub mod wallet;
