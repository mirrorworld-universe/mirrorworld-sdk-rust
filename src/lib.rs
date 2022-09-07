#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use reqwest::header::HeaderMap;

pub const STAGING_REQUEST_URL: &str = "https://api.mirrorworld.fun";
pub const RELEASE_REQUEST_URL: &str = "https://api.mirrorworld.fun";

pub const DEVNET_ENV: &str = "devnet";
pub const MAINNET_ENV: &str = "mainnet";

pub struct Config {
    api_key: String,
    auth: String,
    net: String,
}
impl Config {
    fn new() -> Config {
        Config {
            api_key: "".to_string(),
            auth: "".to_string(),
            net: "devnet".to_string(),
        }
    }
}

lazy_static! {
    static ref API_KEY: Mutex<String> = Mutex::new("".to_string());
    static ref AUTH: Mutex<String> = Mutex::new("".to_string());
    static ref CACHE: Mutex<Config> = Mutex::new(Config::new());
}

pub fn set_config(auth_str: &str, key_str: &str) {
    CACHE.lock().unwrap().api_key = key_str.to_string();
    CACHE.lock().unwrap().auth = auth_str.to_string();
}

pub fn set_auth(auth_str: &str) {
    CACHE.lock().unwrap().auth = auth_str.to_string();
}
pub fn set_apikey(key_str: &str) {
    CACHE.lock().unwrap().api_key = key_str.to_string();
}
pub fn set_network(net: &str) {
    CACHE.lock().unwrap().net = net.to_string();
}
pub fn get_network() -> String {
    return CACHE.lock().unwrap().net.to_string();
}

pub fn get_auth() -> String {
    return CACHE.lock().unwrap().auth.to_string();
}

pub fn get_apikey() -> String {
    return CACHE.lock().unwrap().api_key.to_string();
}

#[derive(Copy, Clone)]
pub enum NetEnv {
    DEVNET,
    MAINNET,
}

pub fn get_env_name(net: NetEnv) -> String {
    let s = match net {
        NetEnv::DEVNET => DEVNET_ENV.to_string(),
        NetEnv::MAINNET => MAINNET_ENV.to_string(),
        // _ => "".to_string(),
    };
    s
}

pub fn get_basic_url(net: NetEnv) -> String {
    let s = match net {
        NetEnv::DEVNET => STAGING_REQUEST_URL.to_string(),
        NetEnv::MAINNET => RELEASE_REQUEST_URL.to_string(),
        // _ => "".to_string(),
    };
    s
}

pub fn get_request_header(api: String, token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-api-key", api.parse().unwrap());
    let authentication: String = "Bearer ".to_string() + &token;
    headers.insert("authentication", authentication.parse().unwrap());

    headers
}

pub mod authentication;
pub mod marketplace;
pub mod wallet;
