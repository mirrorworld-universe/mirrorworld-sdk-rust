pub const STAGING_REQUEST_URL: &str = "https://api-staging.mirrorworld.fun";
pub const RELEASE_REQUEST_URL: &str = "https://api.mirrorworld.fun";

pub const DEVNET_ENV : &str = "devnet";
pub const MAINNET_ENV: &str = "mainnet";



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

pub mod accessors;
pub mod authentication;
pub mod event;
pub mod marketplace;
pub mod wallet;
