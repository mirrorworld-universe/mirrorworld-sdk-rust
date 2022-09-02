use std::collections::HashMap;
use std::option::Option;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;


#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: Option<String>,
    pub data: Option<T>,
    pub code: Option<u32>,
    pub message: Option<String>,
}

// Completes user signup with email

pub struct LoginWithEmailParam <'a> {
   pub email: &'a str,
   pub code: &'a str,
   pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginWithEmailRes {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: Option<LoginWithEmailUserAttribute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginWithEmailUserAttribute {
      pub allow_spend: bool,
      pub id: usize,
      pub eth_address: Option<String>,
      pub sol_address: Option<String>,
      pub email: String,
      pub email_verified: bool,
      pub is_subaccount: bool,
      pub username: String,
      pub main_user_id: Option<String>,
      pub wallet: LoginWithEmailWalletAttribute,
      #[serde(rename = "createdAt")]
      pub created_at: String,
      #[serde(rename = "updatedAt")]
      pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginWithEmailWalletAttribute {
      pub eth_address: String,
      pub sol_address: String
}

#[tokio::main]
pub async fn login_with_email(payload: LoginWithEmailParam)-> 
Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> 
{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
  let mut map = HashMap::new();
  map.insert("code", payload.code.to_string());
  map.insert("email", payload.email.to_string());
  map.insert("password", payload.password.to_string());
  let url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/complete-signup".to_string();
  let client = reqwest::Client::new();
  let res = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await.unwrap();
  let p = res.json::<Response<LoginWithEmailRes>>().await?;
  println!("login_with_email_code_response_is_{:?}",p);
  Ok(Some(p))
}

// Registers a user with email.

#[tokio::main]
pub async fn signup_email(email: &str)->
Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> 
{
 let mut headers = HeaderMap::new();
 headers.insert("Content-Type", "application/json".parse().unwrap());
 headers.insert("Accept", "application/json".parse().unwrap());
 headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
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
let p = res.json::<Response<LoginWithEmailRes>>().await?;
println!("login_with_email_code_response_is_{:?}",p);
Ok(Some(p))
}

// Logs in a user with email and password

pub struct LoginParam  <'a>{
  pub email: &'a str,
  pub password: &'a str,
}

#[tokio::main]
pub async fn login(payload: LoginParam)->
Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> 
{
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
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
   let p = res.json::<Response<LoginWithEmailRes>>().await?;
   Ok(Some(p))
}

//Logs in a user with Google OAuth
#[tokio::main]
pub async fn login_google(identity_provider_token: String)->
Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> 
{
      let mut headers = HeaderMap::new();
      headers.insert("Content-Type", "application/json".parse().unwrap());
      headers.insert("Accept", "application/json".parse().unwrap());
      headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
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
      let p = res.json::<Response<LoginWithEmailRes>>().await?;
      Ok(Some(p))
}

 // GETChecks whether is authenticated or not and returns the user object if true

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUser {
    pub id: u32,
    pub eth_address: Option<String>,
    pub sol_address: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub wallet: LoginWithEmailWalletAttribute,
}
#[tokio::main]
pub async fn fetch_user()->
Result<Option<Response<FetchUser>>, Box<dyn Error>> 
 {
  let mut headers = HeaderMap::new();
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
  headers.insert("authorization", crate::getAuth().parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/auth/me".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  let p = res.json::<Response<FetchUser>>().await?;
  Ok(Some(p))
 }


#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTokens {
     pub sol: u32,
     pub tokens: Option<Vec<WalletToken>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletToken{
     pub ata: Option<String>,
     pub mint: Option<String>,
     pub amount: Option<u32>,
     pub decimals: Option<u32>
}
#[tokio::main]
pub async fn get_token()->
Result<Option<Response<WalletToken>>, Box<dyn Error>> 
 {
  let mut headers = HeaderMap::new();
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
  headers.insert("authorization", crate::getAuth().parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/tokens".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  let p = res.json::<Response<WalletToken>>().await?;
  println!("{:?}",p.status);
  Ok(Some(p))
 }


 //Fetches the wallet transactions for a user

 #[derive(Debug, Serialize, Deserialize)]
 pub struct Transactions {
      pub count: u32,
      pub next_before: Option<String>,
      pub tokens: Option<Vec<Transaction>>,
 }
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Transaction{
      pub blockTime: Option<String>,
      pub slot: Option<u32>,
      pub meta: Option<Meta>,
      pub transaction: Option<TransactionItem>
 }

 #[derive(Debug, Serialize, Deserialize)]
 pub struct TransactionItem{
      pub message: Message,
      pub signatures: Option<String>,
 }

 
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
      pub accountKeys: Option<Vec<AccountKeysEntity>>,
      pub addressTableLookups: Option<String>,
      pub instructions: Option<Vec<ParsedInstructionEntity>>,
      pub recentBlockhash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountKeysEntity {
      pub pubkey: Option<String>,
      pub signer: Option<bool>,
      pub writable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstructionEntity {
      pub accounts: Option<String>,
      pub data: Option<String>,
      pub programId: Option<String>,
      pub parsed: Option<ParsedInstruction>,
      pub program: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstruction {
      pub info: Info,
      pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
      pub account: Option<String>,
      pub mint: Option<String>,
      pub rentSysvar: Option<String>,
      pub source: Option<String>,
      pub systemProgram: Option<String>,
      pub tokenProgram: Option<String>,
      pub wallet: Option<String>,
      pub amount: Option<String>,
      pub authority: Option<String>,
      pub destination: Option<String>,
      pub lamports: Option<u32>,
      pub tokenAmount: Option<TokenAmount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
      pub amount: Option<String>,
      pub decimals: Option<u32>,
      pub uiAmount: Option<u32>,
      pub uiAmountString: Option<String>,
}

 #[derive(Debug, Serialize, Deserialize)]
 pub struct Meta {
      pub err: Option<Err>,
      pub fee: u32,
      pub innerInstructions: Option<InnerInstructionsEntity>,
      pub loadedAddresses: Option<LoadedAddresses>,
      pub logMessages: Option<String>,
      pub postBalances: Option<u32>,
      pub postTokenBalances: Option<PostTokenBalancesEntity>,
      pub preBalances: Option<u32>,
      pub preTokenBalances: Option<PreTokenBalancesEntityOrPostTokenBalancesEntity>,
      pub status: Option<Status>
    }

#[derive(Debug, Serialize, Deserialize)] 
pub struct Status {
      pub Ok: Option<String>,
      pub Err: Option<InstructionError>,
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct InstructionError{
      pub InstructionError: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct PreTokenBalancesEntityOrPostTokenBalancesEntity {
      pub accountIndex: u32,
      pub mint: Option<String>,
      pub owner: Option<String>,
      pub programId: Option<String>,
      pub uiTokenAmount: UiTokenAmountOrTokenAmount,
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct UiTokenAmountOrTokenAmount {
      pub amount: Option<String>,
      pub decimals: Option<u32>,
      pub uiAmount: Option<u32>,
      pub uiAmountString: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedAddresses {
      pub readonly: Option<Vec<String>>,
      pub writable: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTokenBalancesEntity {
      pub accountIndex: Option<u32>,
      pub mint: Option<String>,
      pub owner: Option<String>,
      pub programId: Option<String>,
      pub uiTokenAmount: UiTokenAmount,
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct UiTokenAmount {
      pub amount: String,
      pub decimals: u32,
      pub uiAmount: Option<u32>,
      pub uiAmountString: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstructionsEntity {
      pub index: u32,
      pub instructions: Option<Vec<InstructionsEntity>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionsEntity {
      pub parsed: Option<Parsed>,
      pub program: Option<String>,
      pub programId: Option<String>,
      pub accounts: Option<Vec<String>>,
      pub data: Option<String>,
    }
    
#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
      pub info: ParsedInfo,
      pub r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInfo {
      pub destination: Option<String>,
      pub lamports: Option<u32>,
      pub source: Option<String>,
      pub account: Option<String>,
      pub mint: Option<String>,
      pub rentSysvar: Option<String>,
      pub systemProgram: Option<String>,
      pub tokenProgram: Option<String>,
      pub wallet: Option<String>,
      pub newAccount: Option<String>,
      pub owner: Option<String>,
      pub space: Option<u32>,
      pub amount: Option<String>,
      pub mintAuthority: Option<String>,
      pub authorityType: Option<String>,
      pub multisigAuthority: Option<String>,
      pub newAuthority: Option<String>,
      pub signers: Option<Vec<String>>,
      pub decimals: Option<u32>,
      pub authority: Option<String>,
      pub delegate: Option<String>,
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct Err {
 InstructionError: Option<String>,
}
#[tokio::main]
pub async fn get_transactions()->
Result<Option<Response<Transactions>>, Box<dyn Error>> 
 {
  let mut headers = HeaderMap::new();
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
  headers.insert("authorization", crate::getAuth().parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/wallet/transactions".to_string();
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  let p = res.json::<Response<Transactions>>().await?;
  println!("{:?}",p.status);
  Ok(Some(p))
 }

 // GET Fetch single NFT details
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ISolanaNFT {
   pub name: Option<String>,
   pub sellerFeeBasisPoints: Option<u32>,
   pub updateAuthorityAddress: Option<String>,
   pub description: Option<String>,
   pub image: Option<String>,
   pub externalUrl: Option<String>,
   pub creators: Option<Vec<Creator>>,
   pub owner: Owner,
   pub attributes: Option<Vec<MetadataAttribute>>,
   pub listings: Option<Vec<String>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Creator {
   pub address: Option<String>,
   pub verified: Option<bool>,
   pub share: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Owner {
   pub address: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct MetadataAttribute {
   pub trait_type: Option<String>,
   pub value: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct CreateVerifiedCollectionPayload {
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub metadataUri: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ICreateVerifiedCollectionPayload {
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub metadataUri: Option<String>,
   pub  url: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct CreateVerifiedSubCollectionPayload{
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub metadataUri: Option<String>,
   pub parentCollection: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ICreateVerifiedSubCollectionPayload{
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub metadataUri: Option<String>,
   pub url: Option<String>,
   pub collection_mint: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct IMintNFTPayload {
   pub collection_mint: Option<String>,
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub url: Option<String>,
 }
 
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct MintNFTPayload{
   pub metadataUri: Option<String>,
   pub collection: Option<String>,
   pub collection_mint: Option<String>,
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub url: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct IListNFTPayload {
   pub mint_address: Option<String>,
   pub price: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ListNFTPayload {
   pub mint_address: Option<String>,
   pub price: Option<u32>,
   pub mintAddress: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct IUpdateListingPayload {
   pub mint_address: Option<String>,
   pub price: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct UpdateListingPayload {
   pub mint_address: Option<String>,
   pub price: Option<u32>,
   pub mintAddress: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct IBuyNFTPayload {
   pub mint_address: String,
   pub price: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct BuyNFTPayload  {
   pub mint_address: String,
   pub price: Option<u32>,
   pub mintAddress: String,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ICancelNFTPayload {
   pub mint_address: String,
   pub price: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct CancelListingPayload{
   pub mint_address: String,
   pub price: Option<u32>,
   pub mintAddress: String,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ITransferNFTPayload {
   pub mint_address: String,
   pub to_wallet_address: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct TransferNFTPayload {
   pub mintAddress: String,
   pub recipientAddress: String,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct IVerifiedCollection {
   pub mint_address: String,
   pub url: String,
   pub update_authority: String,
   pub creator_address: String,
   pub name: String,
   pub symbol: String,
   pub collection: Option<String>,
   pub signature: String,
   pub status: String,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct QueryNFTsBase {
   pub limit: u32,
   pub offset: u32,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct QueryNFTsByMintAddressesPayload{
   pub limit: Option<u32>,
   pub offset: Option<u32>,
   pub mintAddresses: Option<Vec<String>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct QueryNFTsByCreatorsPayload {
   pub limit: Option<u32>,
   pub offset: Option<u32>,
   pub creatorAddresses: Option<Vec<String>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct QueryNFTsByUpdateAuthoritiesPayload {
   pub limit: Option<u32>,
   pub offset: Option<u32>,
   pub updateAuthorities: Option<Vec<String>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct QueryNFTsByOwnersPayload
  {
   pub limit: Option<u32>,
   pub offset: Option<u32>,
   pub owners: Option<Vec<String>>,
 }
 
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct SolanaNFTExtended {
   pub name: Option<String>,
   pub sellerFeeBasisPoints: Option<u32>,
   pub updateAuthorityAddress: Option<String>,
   pub description: Option<String>,
   pub image: Option<String>,
   pub externalUrl: Option<String>,
   pub creators: Option<Vec<Creator>>,
   pub owner: Owner,
   pub attributes: Option<Vec<MetadataAttributes>>,
   pub listings: Option<Vec<SolanaNFTListing>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct MetadataAttributes {
   pub trait_type: Option<String>,
   pub value: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct SolanaNFTListing {
   pub id: Option<u32>,
   pub tradeState: Option<String>,
   pub seller: Option<String>,
   pub metadata: Option<String>,
   pub purchaseId: Option<String>,
   pub price: Option<u32>,
   pub tokenSize: Option<u32>,
   pub createdAt: Option<String>,
   pub canceledAt: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct SolanaNFTAuctionActivitiesPayload {
   pub mintAddress: Option<String>,
   pub auctionActivities: Option<Vec<SolanaNFTAuctionActivity>>,
   pub tokenTransfers: Option<Vec<SolanaNFTTransfersEntity>>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct SolanaNFTAuctionActivity {
   pub id: Option<u32>,
   pub mintAddress: Option<String>,
   pub txSignature: Option<String>,
   pub amount: Option<u32>,
   pub receiptType: Option<String>,
   pub tokenPrice: Option<String>,
   pub blockTimeCreated: Option<String>,
   pub blockTimeCanceled: Option<String>,
   pub tradeState: Option<String>,
   pub auctionHouseAddress: Option<String>,
   pub sellerAddress: Option<String>,
   pub buyerAddress: Option<String>,
   pub metadata: Option<String>,
   pub blockTime: Option<String>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct SolanaNFTTransfersEntity {
   pub id: Option<u32>,
   pub mintAddress: Option<String>,
   pub txSignature: Option<String>,
   pub fromWalletAddress: Option<String>,
   pub toWalletAddress: Option<String>,
   pub amount: Option<u32>,
   pub blockTime: Option<String>,
   pub slot: Option<u32>,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct ISolanaNFTMintResult {
   pub mint_address: Option<String>,
   pub url: Option<String>,
   pub update_authority: Option<String>,
   pub creator_address: Option<String>,
   pub name: Option<String>,
   pub symbol: Option<String>,
   pub collection: Option<String>,
   pub signature: Option<String>,
   pub status: Option<String>,
 }
 #[tokio::main]
 pub async fn get_nft_details(sol_addr: &str)->
 Result<Option<Response<ISolanaNFT>>, Box<dyn Error>> 
 {
  let mut headers = HeaderMap::new();
  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("x-api-key", crate::getAPIKEY().parse().unwrap());
  headers.insert("authorization", crate::getAuth().parse().unwrap());
  let  url:String = crate::STAGING_REQUEST_URL.to_string() + &"/v1/solana/nft/".to_string()+ sol_addr;
  let client = reqwest::Client::new();
  let res = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
  let p = res.json::<Response<ISolanaNFT>>().await?;
  println!("{:?}",p.status);
  Ok(Some(p))
 }