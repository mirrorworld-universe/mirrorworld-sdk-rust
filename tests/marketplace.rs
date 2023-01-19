// use core::panicking::panic;
use serde::de::Unexpected::Str;
// use std::borrow::Borrow;
// use serde::de::Unexpected::Str;
// use serde::Serialize;
use mirrorworld_sdk_rust::{marketplace::Marketplace, NetEnv};
use mirrorworld_sdk_rust::marketplace::{MintNftPayload, SolanaCommitment, UpdateNftPayload};


extern crate core;

const KEY: &str = "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU";
const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjEzOTY4NzgsImV4cCI6MTY2Mzk4ODg3OCwianRpIjoiYXV0aDo1OTk2In0.DU6-drhby1g3jF4eQm0OILoYQedDc1x7avY_Kpzn2QU";


#[tokio::test]
async fn test_create_collection() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());
    let name: String = String::from("TEST_ASSERT_0829");
    let symbol: String = String::from("NM");
    let uri: String = String::from("https://market-assets.mirrorworld.fun/gen1/1.json");

    let response = m.create_collection(name.clone(), symbol, uri).await.unwrap();
    // println!("response: {:?}", &response);

    if response.is_none() {
       panic!("response is none")
    }
    assert_eq!(response.unwrap().name, name);

}

#[tokio::test]
async fn test_create_sub_collection() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let name: String = String::from("TEST_ASSERT_0829_2");
    let symbol: String = String::from("NM_1");
    let uri: String = String::from("https://market-assets.mirrorworld.fun/gen1/1.json");
    let parent_collection: String = String::from("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe");

    let response = m.create_sub_collection(name, symbol, uri, parent_collection).await.unwrap();

    println!("response: {:?}", response);
    if response.is_none() {
        panic!("response is none")
    }

    assert_eq!(response.unwrap().collection, Some("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe".to_string()))
}

// test mint nft
#[tokio::test]
async fn test_mint_nft() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let payload: MintNftPayload = MintNftPayload{
        name: String::from("TEST_ASSERT_4"),
        symbol: String::from("NM_1"),
        url: String::from("https://market-assets.mirrorworld.fun/gen1/3.json"),
        collection_mint: String::from("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe")
    };

    let response = m.mint_nft(payload).await.unwrap();

    println!("response: {:?}", &response);
    if response.is_none() {
        panic!("response is none")
    }
    assert_eq!(response.unwrap().collection, "BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe".to_string())
}

// test transfer nft to another wallet
#[tokio::test]
async fn test_transfer_nft() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mint_address = "B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG";
    let to_wallet_address = "H7eoMZiYnX1BdKi5apQSCJLUriL9jbgc8vV9WEar27Ma";

    let response = m.transfer_nft(mint_address.to_string(), to_wallet_address.to_string()).await.unwrap();

    println!("response: {:?}", response);
    if !response.is_none() {
        assert_eq!(response.unwrap().mint_address, "B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string())
    } else {
        println!("err")
    }
}

#[tokio::test]
async fn test_fetch_nfts_by_mint_addresses() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mut address = Vec::new();
    address.push("B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string());
    // address.push("B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string());

    let response = m.fetch_nfts_by_mint_address(address, 10, 1).await.unwrap();

    println!("response: {:#?}", response);
    if response.is_none() {
        panic!("reaponse id none");
    }

    assert_eq!(response.unwrap().nfts.len(), 1)
}

#[tokio::test]
async fn test_list_nft() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());
    let mint_address: String = String::from("CYpEG4e88FCWfBoWfSTdNQ6vTJqgnHJPb7sLAY7Rb3M8");
    let price: f64 = 0.05;
    let auction_house: String = String::from("");

    let response = m.list_nft(mint_address, price, auction_house).await.unwrap();

    println!("response: {:?}", response);
    if response.is_none() {
        panic!("response is none");
    }
    assert_eq!(response.unwrap().mint_address, "CYpEG4e88FCWfBoWfSTdNQ6vTJqgnHJPb7sLAY7Rb3M8".to_string())
}

#[tokio::test]
async fn test_buy_nft() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mint_address: String = String::from("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe");
    let price: f64 = 0.05;
    let auction_house: String = String::from("");

    let response = m.buy_nft(mint_address, price, auction_house).await.unwrap();

    println!("response: {:?}", response);
    if response.is_none() {
        panic!("response ios none")
    }

    assert_eq!(response.unwrap().mint_address, "BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe".to_string())
}


#[tokio::test]
async fn test_update_nft_listing() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mint_address: String = String::from("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe");
    let price: f64 = 0.5;
    let auction_house: String = String::from("");

    let response = m.update_nft_listing(mint_address, price, auction_house).await.unwrap();

    println!("response: {:?}", response);
    if response.is_none() {
        panic!("response ios none")
    }

    assert_eq!(response.unwrap().mint_address, "BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe".to_string())
}

#[tokio::test]
async fn test_cancel_nft_listing() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mint_address: String = String::from("CYpEG4e88FCWfBoWfSTdNQ6vTJqgnHJPb7sLAY7Rb3M8");
    let price: f64 = 0.05;
    let auction_house: String = String::from("");

    let response = m.cancel_nft_listing(mint_address, price, auction_house).await.unwrap();

    println!("response: {:?}", response);
    if response.is_none() {
        panic!("response ios none")
    }

    assert_eq!(response.unwrap().mint_address, "CYpEG4e88FCWfBoWfSTdNQ6vTJqgnHJPb7sLAY7Rb3M8".to_string())
}

// test failed
#[tokio::test]
async fn test_fetch_nfts_by_creator_addresses() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::MAINNET, TOKEN.to_string());

    let mut address = Vec::new();
    address.push("GCeY1zY2QFz1iYekbsX1jQjtJnjyxWXtBhxAJPrvG3Bg".to_string());
    // address.push("B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string());

    let response = m.fetch_nfts_by_creator_address(address, 10, 1).await.unwrap();

    println!("response: {:#?}", response);
    if response.is_none() {
        panic!("reaponse id none");
    }

    assert_eq!(response.unwrap().nfts.len(), 1)
}

// test failed
#[tokio::test]
async fn test_fetch_nfts_by_update_authorities() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::MAINNET, TOKEN.to_string());

    let mut address = Vec::new();
    address.push("4eMGGR6qyvhrSSrHJBjaYkXZpM5kNwbzRQq9q89NfvPC".to_string());
    // address.push("B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string());

    let response = m.fetch_nfts_by_update_authorities(address, 10, 1).await.unwrap();

    println!("response: {:#?}", response);
    if response.is_none() {
        panic!("reaponse id none");
    }

    assert_eq!(response.unwrap().nfts.len(), 1)
}

#[tokio::test]
async fn test_fetch_nfts_by_owner_address() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mut address = Vec::new();
    address.push("H7eoMZiYnX1BdKi5apQSCJLUriL9jbgc8vV9WEar27Ma".to_string());
    // address.push("B2hsVWTFhdz25wNsUrdHpmhTHubLV3wNpiPezGASrggG".to_string());

    let response = m.fetch_nfts_by_owner_addresses(address, 10, 0).await.unwrap();

    // println!("response: {:#?}", response);
    if response.is_none() {
        panic!("reaponse id none");
    }

    assert_eq!(response.unwrap().nfts.len(), 1)
}

#[tokio::test]
async fn test_update_nft() {
    let m = Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

    let mint_address = String::from("BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe");

    let payload = UpdateNftPayload{
        mint_address,
        name: String::from("TEST_ASSERT_4"),
        update_authority: String::from("H7eoMZiYnX1BdKi5apQSCJLUriL9jbgc8vV9WEar27Ma"),
        symbol: String::from("NM_2"),
        url: String::from("https://market-assets.mirrorworld.fun/gen1/3.json"),
        seller_fee_basis_points: 200,
        confirmation: SolanaCommitment::confirmed.to_string(),
    };
    let response = m.update_nft(payload).await.unwrap();
    if response.is_none() {
        panic!("response error");
    }
    assert_eq!(response.unwrap().mint_address, "BPZFHm6GCpSjZ4VfvjwmoDTm6vsuJFoWy82uNqVDfXUe".to_string())
}