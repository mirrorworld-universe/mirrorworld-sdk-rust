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
mirrorworld-sdk-rust = "0.1.0"
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
signup_email("liu_yangchina@126.com");

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

# Transfer Token to another address
transfer_spltoken((
        "sol address",
        amount
        "sol address",
        amount
    ));

# Transfer SOL to another address.
transfer_sol((
    "sol address", 
    amount
));
```

#### marketplace
```bash
use mirrorworld_sdk_rust::{marketplace::Marketplace, NetEnv};
use mirrorworld_sdk_rust::marketplace::GeneralPayload;

let KEY: &str = "your api key";
let TOKEN: &str = "your access token";

// init marketplace object, 
let market =  Marketplace::new(KEY.to_string(), NetEnv::DEVNET, TOKEN.to_string());

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

// create a sub collection
let name: String = String::from("your sub collection");
let symbol: String = String::from("you sub collection token symbol");
let uri: String = String::from("your sub collection metadata uri");
let parent_collection: String = String::from("your parent collection"); // you can got this from the above api
let response = market.create_sub_collection(name, symbol, uri, parent_collection).await.unwrap();

// mint an nft
let payload: GeneralPayload = GeneralPayload{
        name: String::from("your nft name"),
        symbol: "your symbol".to_string(),
        url: "your nft metadta uri".to_string(),
        collection_mint: "your collection".to_string()
    };
let response = market.solana_mint_nft(payload).await.unwrap();

// list an nft
let mint_address: String = String::from("your nft mint address");
let price: f64 = 0.5; // amount in SOL
let response = market.listing_nft(mint_address, price).await.unwrap();


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