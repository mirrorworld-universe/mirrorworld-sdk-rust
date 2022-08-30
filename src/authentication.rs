pub struct Authentication;

use std::collections::HashMap;
use reqwest::header::HeaderMap;

// Completes user signup with email

pub struct LoginWithEmailParam {
   pub email: String,
   pub code: String,
   pub password: String,
}

#[tokio::main]
pub async fn login_with_email(payload: LoginWithEmailParam)->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  let mut map = HashMap::new();
  map.insert("code", payload.code.to_string());
  map.insert("email", payload.email.to_string());
  map.insert("password", payload.password.to_string());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/complete-signup".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
}


// Registers a user with email.

#[tokio::main]
pub async fn signup_email(email: &str)->Result<reqwest::Response, reqwest::Error>{
 let mut headers = HeaderMap::new();
 headers.insert("Content-Type", "application/json".parse().unwrap());
 headers.insert("Accept", "application/json".parse().unwrap());
 headers.insert("x-api-key", "".parse().unwrap());
 let mut map = HashMap::new();
 map.insert("email", email.to_string());
 let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/signup".to_string();
 let client = reqwest::Client::new();
 let res = client
       .post(url)
       .headers(headers)
       .json(&map)
       .send()
       .await.unwrap();
 Ok(res)
}

// Logs in a user with email and password

pub struct LoginParam {
  pub email: String,
  pub password: String,
}

#[tokio::main]
pub async fn login(payload: LoginParam)->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  let mut map = HashMap::new();
  map.insert("email", payload.email.to_string());
  map.insert("password", payload.password.to_string());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/login".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
}

//Logs in a user with Google OAuth

#[tokio::main]
pub async fn login_google(identity_provider_token: String)->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  let mut map = HashMap::new();
  map.insert("identity_provider_token", identity_provider_token.to_string());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/google".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  Ok(res)
 }

 // GETChecks whether is authenticated or not and returns the user object if true

 #[tokio::main]
 pub async fn fetch_user()->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/me".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  Ok(res)
 }

 #[tokio::main]
 pub async fn get_token()->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/tokens".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  Ok(res)
 }


 //Fetches the wallet transactions for a user
  
 #[tokio::main]
 pub async fn get_transactions()->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/transactions".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  Ok(res)
 }


 // GET Fetch single NFT details
 
 #[tokio::main]
 pub async fn get_nft_details(sol_addr: &str)->Result<reqwest::Response, reqwest::Error>{
  let mut headers = HeaderMap::new();
  let  authorization:&str = "Bearer";
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", "".parse().unwrap());
  headers.insert("authorization", authorization.parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/solana/nft/".to_string()+ sol_addr;
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  Ok(res)
 }