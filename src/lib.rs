#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;


use reqwest::header::HeaderMap;


pub const STAGING_REQUEST_URL: &str = "https://api.mirrorworld.fun";
pub const RELEASE_REQUEST_URL: &str = "https://api.mirrorworld.fun";

pub const DEVNET_ENV : &str = "devnet";
pub const MAINNET_ENV: &str = "mainnet";


pub struct Config{
    API_KEY: String,
    AUTH: String,
    NET: String,
}
impl Config {
    fn new()->Config {
        Config{
            API_KEY: "".to_string(),
            AUTH: "".to_string(),
            NET: "devnet".to_string(),
        }
    }
}

lazy_static! {
    static ref API_KEY: Mutex<String> = Mutex::new("".to_string());
    static ref AUTH: Mutex<String> = Mutex::new("".to_string());
    static ref CACHE: Mutex<Config> = Mutex::new(Config::new());
}

pub fn setConfig (
    auth: &str, key: &str 
) {
    CACHE.lock().unwrap().API_KEY = key.to_string();
    CACHE.lock().unwrap().AUTH = auth.to_string();
}
pub fn set_network(
    net: &str
) {
    CACHE.lock().unwrap().NET = net.to_string();
}
pub fn get_network() -> String  {
    return CACHE.lock().unwrap().NET.to_string();
}

pub fn getAuth() -> String {
   return CACHE.lock().unwrap().AUTH.to_string();
}

pub fn getAPIKEY() -> String {
    return CACHE.lock().unwrap().API_KEY.to_string();
}

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


pub mod authentication;
pub mod marketplace;
pub mod wallet;

