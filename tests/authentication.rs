use mirrorworld_sdk_rust::authentication::{
    fetch_user, get_nft_details, get_token, get_transactions, login, login_google,
    login_with_email, signup_email, LoginParam, LoginWithEmailParam, LoginWithEmailRes, Response,
};
use std::error::Error;

#[test]
fn test_login_with_email() {
    let res: Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> = login_with_email({
        LoginWithEmailParam {
            email: "liu_yangchina@126.com",
            code: "933620",
            password: "123456",
        }
    });
    let Response = if let Ok(Some(Response)) = res {
        Response
    } else {
        todo!()
    };
    let code = if let Some(code) = Response.code { code } else { todo!() };
    assert_eq!(code, 100006);
}

#[test]
fn test_signup_email() {
    let res: Result<Option<Response<LoginWithEmailRes>>, Box<dyn Error>> =
        signup_email("liu_yangchina@126.com");
    let Response = if let Ok(Some(Response)) = res {
        Response
    } else {
        todo!()
    };
    let code = if let Some(code) = Response.code { code } else { todo!() };
    assert_eq!(code, 100005);
}

#[test]
fn test_signup() {
    let result = login({
        LoginParam {
            email: "liu_yangchina@126.com",
            password: "123456",
        }
    });
    let Response = if let Ok(Some(Response)) = result {
        Response
    } else {
        todo!()
    };
    let code = if let Some(code) = Response.code { code } else { todo!() };
    assert_eq!(code, 0);
}

#[test]
fn test_fetch_user() {
    let result = fetch_user();
    let Response = if let Ok(Some(Response)) = result {
        Response
    } else {
        todo!()
    };
    let code = if let Some(code) = Response.code { code } else { todo!() };
    assert_eq!(code, 0);
}
