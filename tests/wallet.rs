use mirrorworld_sdk_rust::wallet::{Wallet};
use mirrorworld_sdk_rust::{NetEnv, set_config, set_network};

const TEST_KEY: &str = "mw_testSpTASagrppVD7VVM4h0Cs9jSv0RA6iufbxf";
const TEST_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjMzNSwiZXRoX2FkZHJlc3MiOiIweGYxMTA5RDkzMjM3MkMyOTEwODFEYjgyMjNDRDk5NDQ4YzkxMENhZjAiLCJzb2xfYWRkcmVzcyI6IjNFTDFpNXJ5aUNiWlBEMVhjUk42blZDeHJxdHVKTXE4TlFpSE53ajRRejJYIiwiZW1haWwiOiJzdW5ob254QGdtYWlsLmNvbSIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4ZjExMDlEOTMyMzcyQzI5MTA4MURiODIyM0NEOTk0NDhjOTEwQ2FmMCIsInNvbF9hZGRyZXNzIjoiM0VMMWk1cnlpQ2JaUEQxWGNSTjZuVkN4cnF0dUpNcThOUWlITndqNFF6MlgifSwiY2xpZW50X2lkIjpudWxsLCJ0eXBlIjoiYWNjZXNzX3Rva2VuIiwicmVmcmVzaF90b2tlbl9pZCI6MjI4MjEsImlhdCI6MTY3NTE0NjEyNCwiZXhwIjoxNjc3NzM4MTI0LCJqdGkiOiJhdXRoOjYzMzUifQ.G0MtUmkvBWxLTnbYWv1GDsIiAu_4FKXoZp9DmuMGRrw";


#[tokio::test]
async fn test_transfer_sol() {
    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string());
    let result = wallet.transfer_sol(("C64RkD2jnvrFF8mi9FUBwfhNHuuiPuRMzCLRSWcyJKUG", "2")).await.unwrap();

    println!("result:{:?}", result);
    if !result.is_none() {
        assert_ne!(result.unwrap().tx_signature, "");
    }
}

#[tokio::test]
async fn test_transfer_spltoken() {
    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string());
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

    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string());

    let result = wallet.get_tokens().await.unwrap();

    println!("response:{:?}", result);
    assert_eq!(result.unwrap().tokens.is_empty(), false)
}

#[tokio::test]
async fn test_get_transactions() {

    let wallet = Wallet::new(TEST_KEY.to_string(), NetEnv::DEVNET, TEST_TOKEN.to_string());
    let result = wallet.get_transactions().await.unwrap();
    println!("response:{:?}", result);

    assert_eq!(result.unwrap().transactions.is_empty(), false)
}