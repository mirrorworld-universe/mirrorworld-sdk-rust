use reqwest::header::HeaderMap;

pub const STAGING_REQUEST_URL: &str = "https://api-staging.mirrorworld.fun";
pub const RELEASE_REQUEST_URL: &str = "https://api.mirrorworld.fun";

pub const DEVNET_ENV : &str = "devnet";
pub const MAINNET_ENV: &str = "mainnet";

pub const X_API_KEY: &str = "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU";
pub const Authorization: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjAxMSwiZXRoX2FkZHJlc3MiOiJFd0JpeWtoeUd3cm9yeWFxSmltblZmcmdhYUdKdlY5eVlLaGluclV6OG5ZUCIsInNvbF9hZGRyZXNzIjoiRXdCaXlraHlHd3JvcnlhcUppbW5WZnJnYWFHSnZWOXlZS2hpbnJVejhuWVAiLCJlbWFpbCI6ImxpdV95YW5nY2hpbmFAMTI2LmNvbSIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4OTFjQ2FFOTU2Mjk3QjFCNTU3ZTJjMzYzZDUzRTZGNkJhMGEwQURFMiIsInNvbF9hZGRyZXNzIjoiRXdCaXlraHlHd3JvcnlhcUppbW5WZnJnYWFHSnZWOXlZS2hpbnJVejhuWVAifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjE5NTcyODYsImV4cCI6MTY2NDU0OTI4NiwianRpIjoiYXV0aDo2MDExIn0.ssaa48nfDzm2e-tDygwY48VUecFj1cKmn0hT5Z1aYFs";



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

// pub fn set_authorization(auth: String) {
//     authorization = auth;
// }
// pub fn get_authorization() -> String {
//     authorization.to_string()
// }

pub mod authentication;
pub mod marketplace;
pub mod wallet;
// pub mod getNftDetailStruct;

