use mirrorworld_sdk_rust::wallet::{Wallet};
use mirrorworld_sdk_rust::{NetEnv, set_config, set_network};

const TEST_KEY: &str = "your api key";
const TEST_TOKEN: &str = "your access token";
const SECRET_KEY: &str = "your secret key";


#[tokio::test]
async fn test_transfer_sol() {
    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string(), SECRET_KEY.to_string());
    let result = wallet.transfer_sol(("C64RkD2jnvrFF8mi9FUBwfhNHuuiPuRMzCLRSWcyJKUG", "2")).await.unwrap();

    println!("result:{:?}", result);
    if !result.is_none() {
        assert_ne!(result.unwrap().tx_signature, "");
    }
}

#[tokio::test]
async fn test_transfer_spltoken() {
    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string(), SECRET_KEY.to_string());
    let result = wallet.transfer_spltoken((
        "C64RkD2jnvrFF8mi9FUBwfhNHuuiPuRMzCLRSWcyJKUG",
        "100",
        "BvrranCddgP1VZs7qnKt8wRdGDuM7MxYPCytDmT76E51",
        "9",
    )).await.unwrap();

    println!("result:{:?}", result);
    if !result.is_none() {
        assert_ne!(result.unwrap().tx_signature, "");
    }
}

#[tokio::test]
async fn test_get_tokens() {

    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string(), SECRET_KEY.to_string());

    let result = wallet.get_tokens().await.unwrap();

    println!("response:{:?}", result);
    assert_eq!(result.unwrap().tokens.is_empty(), false)
}

#[tokio::test]
async fn test_get_transactions() {

    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string(), SECRET_KEY.to_string());
    let result = wallet.get_transactions().await.unwrap();
    println!("response:{:?}", result);

    assert_eq!(result.unwrap().transactions.is_empty(), false)
}