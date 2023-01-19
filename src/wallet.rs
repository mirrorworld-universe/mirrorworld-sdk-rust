use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::option::Option;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::json;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: Option<String>,
    pub data: Option<T>,
    pub code: Option<u32>,
    pub message: Option<String>,
    // pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSpltoken  {
  pub tx_signature: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISolanaToken {
    pub ata: String,
    pub mint: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaTokenInfo {
    pub ata: String,
    pub mint: String,
    pub amount: usize,
    pub decimals: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSolanaTokens {
    pub tokens: Vec<SolanaTokenInfo>,
    pub sol: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactions {
    pub transactions: Vec<TransactionInfo>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {

    #[serde(rename = "blockTime")]
    pub block_time: usize,
    pub slot: usize,
    pub transaction: TransactionDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub signatures: Vec<String>,
}

#[tokio::main]
pub async fn transfer_spltoken(payload:(&str, &str, &str, &str))->
Result<Option<Response<TransferSpltoken>>, Box<dyn Error>> 
{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::get_apikey().parse().unwrap());
  headers.insert("authorization", crate::get_auth().parse().unwrap());
  let mut map = HashMap::new();
  let (to_publickey, amount,token_min, decimals ) = payload;
  map.insert("to_publickey", &to_publickey);
  map.insert("amount", &amount);
  map.insert("token_mint", &token_min);
  map.insert("decimals", &decimals);
  let url_ = format!("/v1/{}/wallet/transfer-token", crate::get_network());
  let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  println!("login_with_email_code_response_is_{:?}",res);
  let p = res.json::<Response<TransferSpltoken>>().await?;
  println!("login_with_email_code_response_is_{:?}",p);
  Ok(Some(p))
}


/**
* Transfer SOL to wallet address
*/
#[tokio::main]
pub async fn transfer_sol(payload:(&str, &str))->
Result<Option<Response<TransferSpltoken>>, Box<dyn Error>> 
{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::get_apikey().parse().unwrap());
  headers.insert("authorization", crate::get_auth().parse().unwrap());
  let mut map = HashMap::new();
  let (to_publickey, amount ) = payload;
  map.insert("to_publickey", &to_publickey);
  map.insert("amount", &amount);
  let url_ = format!("/v1/{}/wallet/transfer-sol", crate::get_network());
  let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  let p = res.json::<Response<TransferSpltoken>>().await?;
  Ok(Some(p))
}

#[tokio::main]
pub async fn get_tokens() -> Result<Option<Response<GetSolanaTokens>>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-api-key", crate::get_apikey().parse().unwrap());
    headers.insert("authorization", crate::get_auth().parse().unwrap());

    println!("{:?}", headers);

    let url_ = format!("/v1/{}/wallet/tokens", crate::get_network());
    let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;

    println!("{:?}", url);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();

    println!("{:?}", res);
    let p = res.json::<Response<GetSolanaTokens>>().await?;

    Ok(Some(p))
}

#[tokio::main]
pub async fn get_transactions() -> Result<Option<Response<GetTransactions>>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-api-key", crate::get_apikey().parse().unwrap());
    headers.insert("authorization", crate::get_auth().parse().unwrap());

    let url_ = format!("/v1/{}/wallet/transactions", crate::get_network());
    let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();

    let p = res.json::<Response<GetTransactions>>().await?;
    Ok(Some(p))

}


