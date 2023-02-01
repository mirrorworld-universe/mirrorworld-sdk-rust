use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::option::Option;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::{json, Map};
use std::error::Error;
use crate::{ActionType, get_basic_url, get_env_name, get_request_header, NetEnv};
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

pub struct Wallet {
    api_key: String,
    net: NetEnv,
    token: String,
    secret_key: String,
}

impl Wallet {
    pub fn new(api_key: String, env: NetEnv, token:String, secretKey: String) -> Wallet {
        Wallet{api_key, net: env, token, secret_key: secretKey}
    }

    pub async fn transfer_spltoken(&self, payload:(&str, &str, &str, &str))-> Result<Option<TransferSpltoken>, Box<dyn Error>>
    {
        let mut headers = get_request_header(self.api_key.to_string(), self.token.to_string());
        let mut approve_headers = get_request_header(self.api_key.to_string(), self.secret_key.to_string());

        let mut data = Map::new();
        let (to_publickey, amount,token_min, decimals ) = payload;
        data.insert("to_publickey".to_string(), Value::from(to_publickey));
        data.insert("amount".to_string(), Value::from(amount));
        data.insert("token_mint".to_string(), Value::from(token_min));
        data.insert("decimals".to_string(), Value::from(decimals));

        let action_token = approve_token(
            ActionType::TRANSFER_SPL_TOKEN,
            approve_headers.clone(),
            self.net,
            data.clone()
        ).await.unwrap();
        if action_token.is_none() {
            println!("auction token is none")
        }
        let x_token = action_token.unwrap().authorization_token.to_string();
        headers.insert("x-authorization-token", x_token.parse().unwrap());

        let url_ = format!("/v1/{}/wallet/transfer-token", get_env_name(self.net));
        let url:String = get_basic_url(self.net) + &url_;

        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await.unwrap();
        let resp = res.json::<Response<TransferSpltoken>>().await?;
        Ok(resp.data)
    }

    pub async fn transfer_sol(&self, payload:(&str, &str))-> Result<Option<TransferSpltoken>, Box<dyn Error>>
    {
        let mut headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());
        let mut approve_headers = get_request_header(self.api_key.to_string(), self.secret_key.to_string());

        let mut data = Map::new();
        let (to_publickey, amount ) = payload;
        data.insert("to_publickey".to_string(), Value::from(to_publickey));
        data.insert("amount".to_string(), Value::from(amount));

        let action_token = approve_token(
            ActionType::TRANSFER_SOL,
            approve_headers.clone(),
            self.net,
            data.clone()
        ).await.unwrap();
        if action_token.is_none() {
            panic!("action is none")
        }

        let x_token = action_token.unwrap().authorization_token.to_string();
        headers.insert("x-authorization-token", x_token.parse().unwrap());

        let url_ = format!("/v1/{}/wallet/transfer-sol", get_env_name(self.net));
        let url:String = get_basic_url(self.net) + &url_;
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await.unwrap();
        let resp = res.json::<Response<TransferSpltoken>>().await?;
        Ok(resp.data)
    }

    pub async fn get_tokens(&self) -> Result<Option<GetSolanaTokens>, Box<dyn Error>> {
        let mut headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let url_ = format!("/v1/{}/wallet/tokens", get_env_name(self.net));
        let url:String = get_basic_url(self.net) + &url_;

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await.unwrap();

        let resp = res.json::<Response<GetSolanaTokens>>().await?;

        Ok(resp.data)
    }

    pub async fn get_transactions(&self) -> Result<Option<GetTransactions>, Box<dyn Error>> {
        let mut headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let url_ = format!("/v1/{}/wallet/transactions", get_env_name(self.net));
        let url:String = get_basic_url(self.net) + &url_;
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await.unwrap();

        let resp = res.json::<Response<GetTransactions>>().await?;
        Ok(resp.data)
    }
}