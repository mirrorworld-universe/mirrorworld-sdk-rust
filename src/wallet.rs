
use std::collections::HashMap;
use reqwest::header::HeaderMap;
pub struct Wallet;


 /**
   * Transfer SPL token to a recipient
   */
// struct Dog {
//     name: String,
//     age: i8
// }

#[tokio::main]
pub async fn transfer_spltoken(payload:(&str, &str, &str, &str))->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let mut map = HashMap::new();
  let (to_publickey, amount,token_min, decimals ) = payload;
  map.insert("to_publickey", &to_publickey);
  map.insert("amount", &amount);
  map.insert("token_mint", &token_min);
  map.insert("decimals", &decimals);
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/transfer-token".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
}


/**
* Transfer SOL to wallet address
*/
#[tokio::main]
pub async fn transfer_sol(payload:(&str, &str))->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let mut map = HashMap::new();
  let (to_publickey, amount ) = payload;
  map.insert("to_publickey", &to_publickey);
  map.insert("amount", &amount);

  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/transfer-sol".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
}


