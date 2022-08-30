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
pub struct CreateSubCollectionResp {
    status: String,
    data: Option<CreateCollectionData>,
    code: u32,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralPayload {
    name: String,
    symbol: String,
    url: String,
    collection_mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaMintNftResult {
    mint_address: String,
    url: String,
    update_authority: String,
    creator_address: String,
    name: String,
    symbol: String,
    collection: String,
    signature: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftListing {
    id: usize,
    type_name: String,
    wallet_address: String,
    mint_address: String,
    price: String,
    seller_address: String,
    to_wallet_address: String,
    signature: String,
    status: String,
    update_at: String,
    create_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    address: String,
    verified: bool,
    share: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataAttribute {
    trait_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTListing {
    id: usize,
    #[serde(rename = "tradeState")]
    trade_state: String,
    seller: String,
    metadata: String,
    #[serde(rename = "purchaseId")]
    purchase_id: Option<String>,
    price: f32,
    #[serde(rename = "tokenSize")]
    token_size: usize,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "canceledAt")]
    canceled_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTExtended {
    name: String,
    #[serde(rename = "sellerFeeBasisPoints")]
    seller_fee_basic_points: usize,
    #[serde(rename = "updateAuthorityAddress")]
    update_authority_address: String,
    description: String,
    image: String,
    #[serde(rename = "externalUrl")]
    external_url: String,
    creators: Vec<Creator>,
    owner: Owner,
    attributes: Vec<MetadataAttribute>,
    listings: Vec<SolanaNFTListing>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTAuctionActivity {
    id: usize,
    #[serde(rename = "mintAddress")]
    mint_address: String,
    #[serde(rename = "txSignature")]
    tx_signature: String,
    amount: f32,
    #[serde(rename = "receiptType")]
    receipt_type: String,
    #[serde(rename = "tokenPrice")]
    token_price: String,
    #[serde(rename = "blockTimeCreated")]
    block_time_created: String,
    #[serde(rename = "blockTimeCanceled")]
    block_time_canceled: Option<String>,
    #[serde(rename = "tradeState")]
    trade_state: String,
    #[serde(rename = "auctionHouseAddress")]
    auction_house_address: String,
    #[serde(rename = "sellerAddress")]
    seller_address: String,
    #[serde(rename = "buyerAddress")]
    buyer_address: Option<String>,
    metadata: String,
    #[serde(rename = "blockTime")]
    block_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTTransfersEntity {
    id: usize,
    #[serde(rename = "mintAddress")]
    mint_address: String,
    #[serde(rename = "txSignature")]
    tx_signature: String,
    #[serde(rename = "fromWalletAddress")]
    from_wallet_address: Option<String>,
    #[serde(rename = "toWalletAddress")]
    to_wallet_address: String,
    amount: f32,
    #[serde(rename = "blockTime")]
    block_time: String,
    slot: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaNFTAuctionActivities {
    #[serde(rename = "mintAddress")]
    mint_address: String,

    #[serde(rename = "auctionActivities")]
    auction_activities: Vec<SolanaNFTAuctionActivity>,

    #[serde(rename = "tokenTransfers")]
    token_transfers: Vec<SolanaNFTTransfersEntity>,
}

impl Marketplace {
    fn new(api_key: String, env: NET_ENV, token: String) -> Marketplace {
        Marketplace { api_key: api_key,  net: env, token: token}
    }

    // Create Verified Collection
     async fn create_collection(&self, name: String, symbol: String, metadata_uri: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
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
    async fn create_sub_collection(&self, name: String, symbol: String, metadata_uri: String, parent_coll: String) -> Result<Option<CreateCollectionData>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ =  format!("/v1/{}/solana/mint/sub-collection", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let mut data = HashMap::new();
        data.insert("name", name);
        data.insert("symbol", symbol);
        data.insert("url", metadata_uri);
        data.insert("collection_mint", parent_coll); // parent collection address

        let response = client.post(url).headers(headers).json(&data).send().await?;
        let response_data = response.json::<Response<CreateCollectionData>>().await?;
        Ok(response_data.data)
    }

    // Mint NFT into collection
    async fn solana_mint_nft(&self, payload: GeneralPayload) -> Result<Option<SolanaMintNftResult>, Box<dyn Error>> {
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
    async fn listing_nft(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
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
    async fn buy_nft(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
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
    async fn update_nft_listing(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
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
    async fn cancel_nft_listing(&self, mint_address: String, price: f64) -> Result<Option<NftListing>, Box<dyn Error>> {
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
    async fn transfer_nft(&self, mint_address: String, to_wallet_address: String) -> Result<Option<NftListing>, Box<dyn Error>> {
        let headers:HeaderMap = get_request_header(self.api_key.to_string(), self.token.to_string());

        let client = Client::new();
        let mut url_ = format!("/v1/{}/solana/marketplace/transfer", get_env_name(self.net));
        let mut url = get_basic_url(self.net) + &url_;

        let response = client.post(url).headers(headers).json(&serde_json::json!({
            "mint_address": mint_address,
            "to_wallet_address": to_wallet_address,
        })).send().await?;

        let response_data = response.json::<Response<NftListing>>().await?;
        Ok(response_data.data)
    }

    // Fetch NFTs By Mint Addresses. Returns a detailed payload of all NFTs whose `mintAddresses`
    async fn fetch_nfts_by_mint_address(&self, mint_address: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
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

        Ok(response_data.data)
    }

    // Fetch NFTs By Creator Addresses. Returns a detailed payload of all NFTs whose `creatorAddresses`
    async fn fetch_nfts_by_creator_address(&self, creators: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
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
    async fn fetch_nfts_by_update_authorities(&self, update_authorities: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
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
    async fn fetch_nfts_by_owner_addresses(&self, addresses: Vec<String>, limit: usize, offset: usize) -> Result<Option<SolanaNFTExtended>, Box<dyn Error>> {
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
    async fn fetch_nft_marketplace_activity(&self, mint_address: String) -> Result<Option<SolanaNFTAuctionActivities>, Box<dyn Error>>{
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
