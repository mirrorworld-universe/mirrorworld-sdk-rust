use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::option::Option;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::{json, Map};
use std::error::Error;
use crate::{ActionType, NetEnv};
use crate::marketplace::approve_token;

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
    let mut data = Map::new();
    let (to_publickey, amount,token_min, decimals ) = payload;
    data.insert("to_publickey".to_string(), Value::from(to_publickey));
    data.insert("amount".to_string(), Value::from(amount));
    data.insert("token_mint".to_string(), Value::from(token_min));
    data.insert("decimals".to_string(), Value::from(decimals));

    let action_token = approve_token(
        ActionType::TRANSFER_SPL_TOKEN,
        headers.clone(),
        NetEnv::DEVNET,
        data.clone()
    ).await.unwrap();
    if action_token.is_none() {
        panic!("action is none")
    }
    let x_token = action_token.unwrap().authorization_token.to_string();
    headers.insert("x-authorization-token", x_token.parse().unwrap());


    let url_ = format!("/v1/{}/wallet/transfer-token", crate::get_network());
    let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .json(&data)
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
    let mut data = Map::new();

    let (to_publickey, amount ) = payload;
    data.insert("to_publickey".to_string(), Value::from(to_publickey));
    data.insert("amount".to_string(), Value::from(amount));
    println!("data: {:?}", data.clone());
    let action_token = approve_token(
        ActionType::TRANSFER_SOL,
        headers.clone(),
        NetEnv::DEVNET,
        data.clone()
    ).await.unwrap();
    if action_token.is_none() {
        panic!("action is none")
    }

    let x_token = action_token.unwrap().authorization_token.to_string();
    headers.insert("x-authorization-token", x_token.parse().unwrap());
    println!("headers: {:?}", headers.clone());

    let url_ = format!("/v1/{}/wallet/transfer-sol", crate::get_network());
    let url:String = crate::STAGING_REQUEST_URL.to_string() + &url_;
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .json(&data)
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


