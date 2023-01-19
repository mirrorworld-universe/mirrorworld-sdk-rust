use mirrorworld_sdk_rust::wallet::{get_tokens, get_transactions, transfer_sol, transfer_spltoken};
use mirrorworld_sdk_rust::{set_config, set_network};

#[test]
fn test_get_nft_details() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    set_network("mainnet");
    let result = transfer_sol(("C64RkD2jnvrFF8mi9FUBwfhNHuuiPuRMzCLRSWcyJKUG", "2"));
    let response = if let Ok(Some(response)) = result {
        response
    } else {
        todo!()
    };
    let code = if let Some(code) = response.code {
        code
    } else {
        todo!()
    };
    assert_eq!(code, 0);
}

#[test]
fn test_transfer_spltoken() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = transfer_spltoken((
        "C64RkD2jnvrFF8mi9FUBwfhNHuuiPuRMzCLRSWcyJKUG",
        "2",
        "5WCTV62V2X7iXdRdre73omXcF5agaYq3vj5kCXp6Mzdc",
        "1",
    ));
    let response = if let Ok(Some(response)) = result {
        response
    } else {
        todo!()
    };
    let status = if let Some(status) = response.status {
        status
    } else {
        todo!()
    };
    assert_eq!(status, "success");
}

#[test]
fn test_get_tokens() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjMzNSwiZXRoX2FkZHJlc3MiOiIweGYxMTA5RDkzMjM3MkMyOTEwODFEYjgyMjNDRDk5NDQ4YzkxMENhZjAiLCJzb2xfYWRkcmVzcyI6IjNFTDFpNXJ5aUNiWlBEMVhjUk42blZDeHJxdHVKTXE4TlFpSE53ajRRejJYIiwiZW1haWwiOiJzdW5ob254QGdtYWlsLmNvbSIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4ZjExMDlEOTMyMzcyQzI5MTA4MURiODIyM0NEOTk0NDhjOTEwQ2FmMCIsInNvbF9hZGRyZXNzIjoiM0VMMWk1cnlpQ2JaUEQxWGNSTjZuVkN4cnF0dUpNcThOUWlITndqNFF6MlgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NzQxMTYzMDUsImV4cCI6MTY3NjcwODMwNSwianRpIjoiYXV0aDo2MzM1In0.3Q3YMZo7ixEZnbgcMB4tEaxQJ8xL-PFe0m22yJ8lYJw",
        "mw_testSpTASagrppVD7VVM4h0Cs9jSv0RA6iufbxf"
    );
    set_network("devnet");
    let result = get_tokens();

    println!("response:{:?}", result);
}

#[test]
fn test_get_transactions() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NjMzNSwiZXRoX2FkZHJlc3MiOiIweGYxMTA5RDkzMjM3MkMyOTEwODFEYjgyMjNDRDk5NDQ4YzkxMENhZjAiLCJzb2xfYWRkcmVzcyI6IjNFTDFpNXJ5aUNiWlBEMVhjUk42blZDeHJxdHVKTXE4TlFpSE53ajRRejJYIiwiZW1haWwiOiJzdW5ob254QGdtYWlsLmNvbSIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4ZjExMDlEOTMyMzcyQzI5MTA4MURiODIyM0NEOTk0NDhjOTEwQ2FmMCIsInNvbF9hZGRyZXNzIjoiM0VMMWk1cnlpQ2JaUEQxWGNSTjZuVkN4cnF0dUpNcThOUWlITndqNFF6MlgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NzQxMTYzMDUsImV4cCI6MTY3NjcwODMwNSwianRpIjoiYXV0aDo2MzM1In0.3Q3YMZo7ixEZnbgcMB4tEaxQJ8xL-PFe0m22yJ8lYJw",
        "mw_testSpTASagrppVD7VVM4h0Cs9jSv0RA6iufbxf"
    );
    set_network("devnet");
    let result = get_transactions();
    println!("response:{:?}", result);
}