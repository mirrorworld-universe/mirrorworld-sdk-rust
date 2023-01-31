use mirrorworld_sdk_rust::wallet::{Wallet};
use mirrorworld_sdk_rust::{NetEnv, set_config, set_network};

const TEST_KEY: &str = "mw_testSpTASagrppVD7VVM4h0Cs9jSv0RA6iufbxf";
const TEST_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTcsImV0aF9hZGRyZXNzIjoiMHhBNzc5MEE0MDFmZjA4OWVDMzkzMjJGN2Q4MkQyMWMzRDNCMDVmRDQ0Iiwic29sX2FkZHJlc3MiOiJIN2VvTVppWW5YMUJkS2k1YXBRU0NKTFVyaUw5amJnYzh2VjlXRWFyMjdNYSIsImVtYWlsIjoic3VuaG9uZ3hpYW5nQHJjdC5zdHVkaW8iLCJ3YWxsZXQiOnsiZXRoX2FkZHJlc3MiOiIweEE3NzkwQTQwMWZmMDg5ZUMzOTMyMkY3ZDgyRDIxYzNEM0IwNWZENDQiLCJzb2xfYWRkcmVzcyI6Im5kV292b0dZdjE3QWM0eGVGNm9GdkNQcjJXcmVEYnB6NVM3amNKTEg5VU0ifSwiY2xpZW50X2lkIjpudWxsLCJ0eXBlIjoiYWNjZXNzX3Rva2VuIiwicmVmcmVzaF90b2tlbl9pZCI6MjI2ODMsImlhdCI6MTY3NTA2NDE2NCwiZXhwIjoxNjc3NjU2MTY0LCJqdGkiOiJhdXRoOjk3In0.wYoeC2QPGAqyf5zyXrPzsxe8opRP_oeiQrJbEEpn3bY";


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
        "2",
        "5WCTV62V2X7iXdRdre73omXcF5agaYq3vj5kCXp6Mzdc",
        "1",
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