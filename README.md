## Usage

1. Clone project code.

```bash
git clone git@github.com:mirrorworld-universe/mirrorworld-sdk-rust.git
cd mirrorworld-sdk-rust
```

2. run.

```bash
cargo run 
```

3. test.

```bash
cargo test
```

## Quick Setup
### Install
```bash
# /Cargo.toml
[dependencies]
mirrorworld-sdk-rust = "0.1.5"
```
### Setup
#### authentication && wallet
```bash
use mirrorworld_sdk_rust::{
    fetch_user,
    get_nft_details,
    get_token,
    get_transactions,
    login,
    complete_signup,
    signup_email,
};
use mirrorworld_sdk_rust::{
    set_config,
    set_apikey,
};
use mirrorworld_sdk_rust::wallet::{Wallet};

# set global apikey
set_apikey("your apikey"); 
# Completes user signup with email
let res = complete_signup({
    LoginWithEmailParam {
        email: "your email",
        code: "your email code",
        password: "your password",
    }
});

let response = if let Ok(Some(response)) = res {
    response
} else {
    todo!()
};
# Completes user signup with email
signup_email("email@example.com");

set_config(
        "your token",
        "your apikey"
    );

# GETChecks whether is authenticated or not and returns the user object if true
fetch_user();

# Get wallet tokens.
get_token();

# Fetches the wallet transactions for a user
get_transactions();

# GET Fetch single NFT details
get_nft_details("nft address");


# wallet

let KEY: &str = "your api key";
let TOKEN: &str = "your access token";
let SECRET_KEY: &str = "your secret key"

# init wallet 
let wallet = Wallet::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string(), SECRET_KEY.to_string());

# Transfer Token to another address
let result = wallet.transfer_spltoken((
        "to publickey address",
        "amount"
        "mint address",
        "decimals",
    )).await.unwrap();


# Transfer SOL to another address.
let result = wallet.transfer_sol(("to publickey address", "amount")).await.unwrap();

# get wallet tokens
let result = wallet.get_tokens().await.unwrap();

# get wallet transactions
let result = wallet.get_transactions().await.unwrap();
```

#### marketplace
```bash
use mirrorworld_sdk_rust::{marketplace::Marketplace, NetEnv};
use mirrorworld_sdk_rust::marketplace::GeneralPayload;

let KEY: &str = "your api key";
let TOKEN: &str = "your access token";
let SECRET_KEY: &str = "your secret key";

// init marketplace object, 
let market =  Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string(), SECRET_KEY.to_string());

// create a collection
let name: String = String::from("your collection name");
let symbol: String = String::from("your token symbol name");
let uri: String = String::from("you collection metadata uri")
let response = market.create_collection(name, symbol, uri).await.unwrap();
if response.is_none() {    
    // your code
} else {
    // your code
}

// mint an nft
let payload: GeneralPayload = GeneralPayload{
        name: String::from("your nft name"),
        symbol: "your symbol".to_string(),
        url: "your nft metadta uri".to_string(),
        collection_mint: "your collection".to_string()
    };
let response = market.mint_nft(payload).await.unwrap();

// list an nft
let mint_address: String = String::from("your nft mint address");
let price: f64 = 0.5; // amount in SOL
let auction_house: String = String::from(""); // your auction house address
let response = market.list_nft(mint_address, price, auction_house).await.unwrap();


// buy an nft
let mint_address: String = String::from("your nft mint address");
let price: f64 = 0.5; // amount in SOL 
let response = market.buy_nft(mint_address, price).await.unwrap();

// update nft listing price
let mint_address: String = String::from("your nft mint address");
let price: f64 = 0.5; // amount in SOL
let response = market.update_nft_listing(mint_address, price).await.unwrap();

// cancel listing
let mint_address: String = String::from("your nft mint address");
let price: f64 = 0.5; // amount in SOL
let response = market.cancel_nft_listing(mint_address, price).await.unwrap();

// transfer nft
let mint_address = String::from("your nft mint address");
let to_wallet_address = String::from("to wallet address");
let response = market.transfer_nft(mint_address, to_wallet_address).await.unwrap();

// fetch nfts by mint address
let mut addresses = Vec::new();
addresses.push("nft mint address".to_string());
let limit: usize = 10;
let offset: usize = 1;
let response = market.fetch_nfts_by_mint_address(addresses, limit, offset).await.unwrap();

// fetch nfts by creator address
let mut addresses = Vec::new();
addresses.push("creator address".to_string());
let limit: usize = 10;
let offset: usize = 1;
let response = market.fetch_nfts_by_creator_address(addresses, limit, offset).await.unwrap();

// fetch nfts by update authorities
let mut addresses = Vec::new();
addresses.push("update authorities address".to_string());
let limit: usize = 10;
let offset: usize = 1;
let response = market.fetch_nfts_by_update_authorities(addresses, limit, offset).await.unwrap();

// fetch nfts by owner address 
let mut addresses = Vec::new();
addresses.push("owner address".to_string());
let limit: usize = 10;
let offset: usize = 1;
let response = market.fetch_nfts_by_owner_addresses(addresses, limit, offset).await.unwrap();

// fetch nfts marketplace activity
let address = String::from("nft mint address");
let response = market.fetch_nft_marketplace_activity(address).await.unwrap();
```