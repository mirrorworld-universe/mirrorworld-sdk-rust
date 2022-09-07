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

1. test.

```bash
cargo test
```

Installing rustup on Linux or macOS
```bash
https://juejin.cn/post/7108305647287926792
https://doc.rust-lang.org/book/ch01-01-installation.html
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
    login_with_email,
    signup_email,
};
use mirrorworld_sdk_rust::{
    set_config,
    set_apikey,
};
# set global apikey
set_apikey("your apikey"); 
# Completes user signup with email
let res = login_with_email({
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
``` bash
use mirrorworld_sdk_rust::{marketplace::Marketplace, NetEnv};
use mirrorworld_sdk_rust::marketplace::GeneralPayload;
```