use std::borrow::Borrow;
use reqwest::header::HeaderMap;
use reqwest::{self, Client, Url};
use std::error::Error;
use std::collections::HashMap;
use futures::future::ok;
use serde::de::Unexpected::Str;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::Value;
use crate::{get_basic_url, get_env_name, get_request_header, NET_ENV};

pub struct Marketplace {
    api_key: String,
    net: NET_ENV,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    status: String,
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
pub struct GeneralPayload {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct NftListing {
    pub id: usize,
    pub type_name: String,
    pub wallet_address: String,
    pub mint_address: String,
    pub price: String,
    pub seller_address: String,
    pub to_wallet_address: String,
    pub signature: String,
    pub status: String,
    pub update_at: String,
    pub create_at: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataAttribute {
    pub trait_type: String,
    pub value: String,
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
pub struct SolanaNFTExtended {
    pub name: String,
    #[serde(rename = "sellerFeeBasisPoints")]
    pub seller_fee_basic_points: usize,
    #[serde(rename = "updateAuthorityAddress")]
    pub update_authority_address: String,
    pub description: String,
    pub image: String,
    #[serde(rename = "externalUrl")]
    pub external_url: String,
    pub creators: Vec<Creator>,
    pub owner: Owner,
    pub attributes: Vec<MetadataAttribute>,
    pub listings: Vec<SolanaNFTListing>,
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
    pub fn new(api_key: String, env: NET_ENV, token: String) -> Marketplace {
        Marketplace { api_key,  net: env, token }
    }

    // Create Verified Collection
     pub async fn create_collection(&self, name: String, symbol: String, metadata_uri: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

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
        
        let response_data = response.json::<Response<CreateCollectionData>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // create verified sub collection
    pub async fn create_sub_collection(&self, name: String, symbol: String, metadata_uri: String, parent_coll: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ =  format!("/v1/{}/solana/mint/sub-collection", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

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
    pub async fn solana_mint_nft(&self, payload: GeneralPayload) -> Result<Option<SolanaMintNftResult>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/mint/nft", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let data = self.general_payload_to_map(payload);
        let response = client.post(url).headers(headers).json(&data).send().await?;
        let response_data = response.json::<Response<SolanaMintNftResult>>().await?;

        Ok(response_data.data)
    }

    // List NFT ion Mirror World Marketplace
    pub async fn listing_nft(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/list", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        Ok(response_data.data)
    }

    // Purchase NFT on Mirror World Marketplace
    pub async fn buy_nft(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/buy", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        Ok(response_data.data)
    }

    // Update NFT Listing on Mirror World Marketplace
    pub async fn update_nft_listing(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/update", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        Ok(response_data.data)
    }

    // Cancel listing NFT on Mirror World Marketplace
    pub async fn cancel_nft_listing(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/cancel", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "price": price,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        Ok(response_data.data)
    }

    // Transfer NFT from holder's wallet to another address
    pub async fn transfer_nft(&self, mint_address: String, to_wallet_address: String) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/transfer", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "to_wallet_address": to_wallet_address,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        println!("response: {:?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Mint Addresses. Returns a detailed payload of all NFTs whose `mintAddresses`
    pub async fn fetch_nfts_by_mint_address(&self, mint_address: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/nft/mints", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTExtended>>().await?;
        println!("{:#?}", response_data);
        Ok(response_data.data)
    }

    // Fetch NFTs By Creator Addresses. Returns a detailed payload of all NFTs whose `creatorAddresses`
    pub async fn fetch_nfts_by_creator_address(&self, creators: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/nft/creators", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "creators": creators,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTExtended>>().await?;

        Ok(response_data.data)
    }

    // Fetch NFTs By Update Authorities Addresses. Returns a detailed payload of all NFTs whose `updateAuthorities`
    pub async fn fetch_nfts_by_update_authorities(&self, update_authorities: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/nft/udpate-authorities", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "update_authorities": update_authorities,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTExtended>>().await?;

        Ok(response_data.data)
    }

    // Fetch NFTs By Owners Addresses. Returns a detailed payload of all NFTs whose `owners`
    pub async fn fetch_nfts_by_owner_addresses(&self, addresses: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/nft/owners", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "owners": addresses,
            "limit": limit,
            "offset": offset,
        })).send().await?;

        let response_data = response.json::<Response<SolanaNFTExtended>>().await?;

        Ok(response_data.data)
    }

    // Fetch Solana NFT Marketplace Activity
    pub async fn fetch_nft_marketplace_activity(&self, mint_address: String) -> Result<Option<SolanaNFTAuctionActivities>, Box<dyn Error>>{
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/activity/{}", get_env_name(self.net), mint_address);
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.get(url).headers(headers).send().await?;

        let response_data = response.json::<Response<SolanaNFTAuctionActivities>>().await?;
        Ok(response_data.data)
    }

    // mint_sub_collection、mint_nft
    fn general_payload_to_map(&self, payload: GeneralPayload) -> HashMap<&str, String> {
        let mut data = HashMap::new();
        data.insert("name", payload.name);
        data.insert("symbol", payload.symbol);
        data.insert("url", payload.url);
        data.insert("collection_mint", payload.collection_mint); // parent collection address
        data
    }
}

