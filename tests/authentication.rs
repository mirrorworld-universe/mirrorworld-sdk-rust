use mirrorworld_sdk_rust::authentication::{
    fetch_user,
    get_nft_details,
    get_token,
    get_transactions,
    login,
    // login_google,
    login_with_email,
    signup_email,
    LoginParam,
    LoginWithEmailParam,
    LoginWithEmailRes,
    Response,
};
use mirrorworld_sdk_rust::set_config;
use std::error::Error;

#[test]
fn test_login_with_email() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let res: Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> = login_with_email({
        LoginWithEmailParam {
            email: "liu_yangchina@126.com",
            code: "933620",
            password: "123456",
        }
    });
    let response = if let Ok(Some(response)) = res {
        response
    } else {
        todo!()
    };
    let code = if let Some(code) = response.code {
        code
    } else {
        todo!()
    };
    assert_eq!(code, 100006);
}

#[test]
fn test_signup_email() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let res: Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> =
        signup_email("liu_yangchina@126.com");
    let response = if let Ok(Some(response)) = res {
        response
    } else {
        todo!()
    };
    let code = if let Some(code) = response.code {
        code
    } else {
        todo!()
    };
    assert_eq!(code, 100005);
}

#[test]
fn test_signup() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = login({
        LoginParam {
            email: "liu_yangchina@126.com",
            password: "123456",
        }
    });
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
fn test_fetch_user() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = fetch_user();
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
fn test_get_token() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = get_token();
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
fn test_get_transactions() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = get_transactions();
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
fn test_get_nft_details() {
    set_config(
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6NTk5NiwiZXRoX2FkZHJlc3MiOiJHMWllaTNCV2dSWnNTQjNSaHJzRHdHR2p5QWpyTXpKeUZaWlpuMXQyM3lteCIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgiLCJlbWFpbCI6ImxpdXlhbmdAcmN0LnN0dWRpbyIsIndhbGxldCI6eyJldGhfYWRkcmVzcyI6IjB4QzhjYTREOTY2OURCQWIzRTBERDI4QmZEMzhhYjdBMTgyN2VDNmNjMyIsInNvbF9hZGRyZXNzIjoiRzFpZWkzQldnUlpzU0IzUmhyc0R3R0dqeUFqck16SnlGWlpabjF0MjN5bXgifSwiY2xpZW50X2lkIjpudWxsLCJpYXQiOjE2NjIwMzA3MjYsImV4cCI6MTY2NDYyMjcyNiwianRpIjoiYXV0aDo1OTk2In0.6uZrZZSMtJY72pwXnYCmKlh5JSs7WfBdQAZoGnDWyuc",
        "dvriNJlG7ITz1q0S57ZBOAWaDzA3cfjjcnU"
    );
    let result = get_nft_details("G1iei3BWgRZsSB3RhrsDwGGjyAjrMzJyFZZZn1t23ymx");
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
