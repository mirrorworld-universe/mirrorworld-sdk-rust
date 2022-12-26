// use std::borrow::Borrow;
use reqwest::header::HeaderMap;
use reqwest::{
    self, 
    Client, 
    // Url
};
use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::fmt::format;
// use std::fs::OpenOptions;
// use futures::future::ok;
// use serde::de::Unexpected::Str;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::json;
use crate::{get_basic_url, get_env_name, get_request_header, NetEnv};

pub struct Marketplace {
    api_key: String,
    net: NetEnv,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    status: Option<String>,
    data: Option<T>,
    code: u32,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionData {
    pub mint_address: String,
    pub url: String,
    pub update_authority: String,
    pub creator_address: String,
    pub name: String,
    pub symbol: String,
    pub collection: Option<String>,
    pub signature: String,
    pub status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MintNftPayload {
   pub name: String,
   pub symbol: String,
   pub url: String,
   pub collection_mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaMintNftResult {
    pub mint_address: String,
    pub url: String,
    pub update_authority: String,
    pub creator_address: String,
    pub name: String,
    pub symbol: String,
    pub collection: String,
    pub signature: String,
    pub status: String,
}

pub enum SolanaCommitment {
    confirmed,
    finalized,
}

impl fmt::Display for SolanaCommitment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolanaCommitment::confirmed => write!(f, "confirmed"),
            SolanaCommitment::finalized => write!(f, "finalized"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNftPayload {
    pub mint_address: String,
    pub name: String,
    pub update_authority: String,
    pub symbol: String,
    pub url: String,
    pub seller_fee_basis_points: usize,
    pub confirmation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftDetail {
    pub id: usize,
    #[serde(rename = "type")]
    pub type_name: String,
    pub wallet_address: String,
    pub mint_address: String,
    pub price: String,
    pub seller_address: String,
    pub to_wallet_address: Option<String>,
    pub signature: String,
    pub status: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTMintResult {
    pub mint_address: String,
    pub url: String,
    pub update_authority: String,
    pub creator_address: String,
    pub name: String,
    pub symbol: String,
    pub collection: String,
    pub signature: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaUpdateNftResult {
    pub mint_address: String,
    pub url: String,
    pub update_authority: String,
    pub creator_address: String,
    pub name: String,
    pub symbol: String,
    pub seller_fee_basis_points: usize,
    pub signature: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    pub address: String,
    pub verified: bool,
    pub share: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub address: String,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub enum Values {
//     #[serde(rename = "value")]
//     Text(String),
//     #[serde(rename = "value")]
//     Number(i32),
// }

fn string_or_number<'de, D>(de: D) -> Result<Value, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let helper: Value = Deserialize::deserialize(de)?;

    match helper {
        Value::Number(n) => {
            println!("{:#?}", n.as_f64().unwrap().to_string());
            Ok(Value::Number(n))
        }
        Value::String(s) => Ok(json!(s)),
        _ => Ok(json!(null)),
    }}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataAttribute {
    pub trait_type: String,
    #[serde(deserialize_with = "string_or_number")]
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTListing {
    pub id: usize,
    #[serde(rename = "tradeState")]
    pub trade_state: String,
    pub seller: String,
    pub metadata: String,
    #[serde(rename = "purchaseId")]
    pub purchase_id: Option<String>,
    pub price: f32,
    #[serde(rename = "tokenSize")]
    pub token_size: usize,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "canceledAt")]
    pub canceled_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTs {
    pub nfts: Vec<SolanaNFTExtended>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTExtended {
    pub name: String,
    #[serde(rename = "sellerFeeBasisPoints")]
    pub seller_fee_basic_points: usize,
    #[serde(rename = "updateAuthorityAddress")]
    pub update_authority_address: String,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
    pub creators: Vec<Creator>,
    pub owner: Option<Owner>,
    pub attributes: Option<Vec<MetadataAttribute>>,
    pub listings: Option<Vec<SolanaNFTListing>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTAuctionActivity {
    pub id: usize,
    #[serde(rename = "mintAddress")]
    pub mint_address: String,
    #[serde(rename = "txSignature")]
    pub tx_signature: String,
    pub amount: f32,
    #[serde(rename = "receiptType")]
    pub receipt_type: String,
    #[serde(rename = "tokenPrice")]
    pub token_price: String,
    #[serde(rename = "blockTimeCreated")]
    pub block_time_created: String,
    #[serde(rename = "blockTimeCanceled")]
    pub block_time_canceled: Option<String>,
    #[serde(rename = "tradeState")]
    pub trade_state: String,
    #[serde(rename = "auctionHouseAddress")]
    pub auction_house_address: String,
    #[serde(rename = "sellerAddress")]
    pub seller_address: String,
    #[serde(rename = "buyerAddress")]
    pub buyer_address: Option<String>,
    pub metadata: String,
    #[serde(rename = "blockTime")]
    pub block_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTTransfersEntity {
    pub id: usize,
    #[serde(rename = "mintAddress")]
    pub mint_address: String,
    #[serde(rename = "txSignature")]
    pub tx_signature: String,
    #[serde(rename = "fromWalletAddress")]
    pub from_wallet_address: Option<String>,
    #[serde(rename = "toWalletAddress")]
    pub to_wallet_address: String,
    pub amount: f32,
    #[serde(rename = "blockTime")]
    pub block_time: String,
    pub slot: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTAuctionActivities {
    #[serde(rename = "mintAddress")]
    pub mint_address: String,

    #[serde(rename = "auctionActivities")]
    pub auction_activities: Vec<SolanaNFTAuctionActivity>,

    #[serde(rename = "tokenTransfers")]
    pub token_transfers: Vec<SolanaNFTTransfersEntity>,
}

impl Marketplace {
    pub fn new(api_key: String, env: NetEnv, token: String) -> Marketplace {
        Marketplace { api_key,  net: env, token }
    }

    // Create Verified Collection
     pub async fn create_collection(&self, name: String, symbol: String, metadata_uri: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let  url_ =  format!("/v1/{}/solana/mint/collection", get_env_name(self.net));
        let  url = get_basic_url(self.net) + &url_;

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
        
        let response_data = response.json::<Response<CreateCollectionData>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // create verified sub collection
    pub async fn create_sub_collection(&self, name: String, symbol: String, metadata_uri: String, parent_coll: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ =  format!("/v1/{}/solana/mint/sub-collection", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let mut data = HashMap::new();
        data.insert("name", name);
        data.insert("symbol", symbol);
        data.insert("url", metadata_uri);
        data.insert("collection_mint", parent_coll); // parent collection address
        data.insert("confirmation", "confirmed".to_string());

        let response = client.post(url).headers(headers).json(&data).send().await?;
        let response_data = response.json::<Response<CreateCollectionData>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // Mint NFT into collection
    pub async fn mint_nft(&self, payload: MintNftPayload) -> Result<Option<SolanaMintNftResult>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/mint/nft", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let data = self.general_payload_to_map(payload);
        let response = client.post(url).headers(headers).json(&data).send().await?;
        let response_data = response.json::<Response<SolanaMintNftResult>>().await?;

        Ok(response_data.data)
    }

    pub async fn update_nft(&self, payload: UpdateNftPayload) -> Result<Option<SolanaUpdateNftResult>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/mint/update", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&payload).send().await?;

        let response_data = response.json::<Response<SolanaUpdateNftResult>>().await?;
        println!("{:?}", response_data);
        Ok(response_data.data)

    }

    // List NFT ion Mirror World Marketplace
    pub async fn list_nft(&self, mint_address: String, price: f64, auction_house: String) -> Result<Option<NftDetail>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/marketplace/list", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
            "auction_house": auction_house,
        })).send().await?;

        let response_data = response.json::<Response<NftDetail>>().await?;
        println!("{:?}", response_data);
        Ok(response_data.data)
    }

    // Purchase NFT on Mirror World Marketplace
    pub async fn buy_nft(&self, mint_address: String, price: f64, auction_house: String) -> Result<Option<NftDetail>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/marketplace/buy", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
            "auction_house": auction_house,
        })).send().await?;

        let response_data = response.json::<Response<NftDetail>>().await?;
        println!("{:?}", response_data);
        Ok(response_data.data)
    }

    // Update NFT Listing on Mirror World Marketplace
    pub async fn update_nft_listing(&self, mint_address: String, price: f64, auction_house: String) -> Result<Option<NftDetail>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/marketplace/update", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
            "auction_house": auction_house,
        })).send().await?;

        let response_data = response.json::<Response<NftDetail>>().await?;
        Ok(response_data.data)
    }

    // Cancel listing NFT on Mirror World Marketplace
    pub async fn cancel_nft_listing(&self, mint_address: String, price: f64, auction_house: String) -> Result<Option<NftDetail>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/marketplace/cancel", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
            "auction_house": auction_house,
        })).send().await?;

        let response_data = response.json::<Response<NftDetail>>().await?;
        println!("{:?}", response_data);
        Ok(response_data.data)
    }

    // Transfer NFT from holder's wallet to another address
    pub async fn transfer_nft(&self, mint_address: String, to_wallet_address: String) -> Result<Option<NftDetail>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/marketplace/transfer", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "to_wallet_address": to_wallet_address,
        })).send().await?;

        let response_data = response.json::<Response<NftDetail>>().await?;
        println!("response: {:?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Mint Addresses. Returns a detailed payload of all NFTs whose `mintAddresses`
    pub async fn fetch_nfts_by_mint_address(&self, mint_address: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTs>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/nft/mints", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_addresses": mint_address,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTs>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Creator Addresses. Returns a detailed payload of all NFTs whose `creatorAddresses`
    pub async fn fetch_nfts_by_creator_address(&self, creators: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTs>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/nft/creators", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "creators": creators,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTs>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Update Authorities Addresses. Returns a detailed payload of all NFTs whose `updateAuthorities`
    pub async fn fetch_nfts_by_update_authorities(&self, update_authorities: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTs>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/nft/udpate-authorities", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "update_authorities": update_authorities,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTs>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Owners Addresses. Returns a detailed payload of all NFTs whose `owners`
    pub async fn fetch_nfts_by_owner_addresses(&self, addresses: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTs>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let url_ = format!("/v1/{}/solana/nft/owners", get_env_name(self.net));
        let url = get_basic_url(self.net) + &url_;
        println!("{}", url);

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "owners": addresses,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTs>>().await?;
        println!("{:#?}", response_data.data);
        Ok(response_data.data)
    }

    // mint_sub_collection、mint_nft
    fn general_payload_to_map(&self, payload: MintNftPayload) -> HashMap<&str, String> {
        let mut data = HashMap::new();
        data.insert("name", payload.name);
        data.insert("symbol", payload.symbol);
        data.insert("url", payload.url);
        data.insert("collection_mint", payload.collection_mint); // parent collection address
        data
    }
}

