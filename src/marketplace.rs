use std::borrow::Borrow;
use reqwest::header::HeaderMap;
use reqwest::{self, Client};
use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use crate::{get_basic_url, get_env_name, NET_ENV};

pub struct Marketplace {
    api_key: String,
    net: NET_ENV,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionData {
    mint_address: String,
    url: String,
    update_authority: String,
    creator_address: String,
    name: String,
    symbol: String,
    collection: Option<String>,
    signature: String,
    status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionResp {
    status: String,
    data: Option<CreateCollectionData>,
    code: u32,
    message: Option<String>,
}

impl Marketplace {
    fn new(api_key: String, env: NET_ENV, token: String) -> Marketplace {
        Marketplace { api_key: api_key,  net: env, token: token}
    }

     async fn create_collection(&self, name: String, symbol: String, metadata_uri: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("x-api-key", self.api_key.parse().unwrap());

        let mut authorization = "Bearer ".to_string() + &self.token.to_string();
        headers.insert("authorization", authorization.parse().unwrap());

        let client = Client::new();
        let mut url_ =  format!("/v1/{}/solana/mint/collection", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let mut data = HashMap::new();
        data.insert("name", name);
        data.insert("symbol", symbol);
        data.insert("url", metadata_uri);
        let response = client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await?;
        
        let response_data = response.json::<CreateCollectionResp>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }
}

#[cfg(test)]
mod tests {
    use super::{Marketplace, NET_ENV};

    #[tokio::test]
     async fn test_create_collection() {
        let key = "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU";
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjEzOTY4NzgsImV4cCI6MTY2Mzk4ODg3OCwianRpIjoiYXV0aDo1OTk2In0.DU6-drhby1g3jF4eQm0OILoYQedDc1x7avY_Kpzn2QU";
    
        let m = Marketplace::new(key.to_string(), NET_ENV::DEVNET, token.to_string());

        let response = m.create_collection("TEST_ASSERT_0825_2".to_string(), "NN".to_string(), "https://market-assets.mirrorworld.fun/gen1/1.json".to_string()).await.unwrap();
        // println!("{:#?}", response == None);
        // assert_eq!(response.name, "TEST_ASSERT_0825_1".to_string())
    }
}
