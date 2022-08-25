
use std::collections::HashMap;
pub struct Wallet;

 /**
   * Transfer SPL token to a recipient
   */
#[tokio::main]
pub async fn transfer_spltoken(payload:(&str, &str, &str, &str))->Result<reqwest::Response, reqwest::Error>{
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
  let mut map = HashMap::new();
  let (to_publickey, amount ) = payload;
  map.insert("to_publickey", &to_publickey);
  map.insert("amount", &amount);

  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/transfer-sol".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
}


