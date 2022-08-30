
use mirrorworld_sdk_rust::authentication::{ 
  login_with_email, 
  signup_email, 
  login, 
  login_google, 
  fetch_user,
  get_token,
  get_transactions,
  get_nft_details,
  LoginWithEmailParam
};
#[test]
fn login_with_email_test() {
    #[tokio::main]
    async fn test(){
      let result:Result<reqwest::Response, reqwest::Error> = login_with_email(
        {LoginWithEmailParam{
           email: "liu_yangchina@126.com".to_string(),
           code: "279933".to_string(),
           password: "123456".to_string()
       }});
       println!("{:?}", result);
       "sueccesful";
    }
    // assert_eq!(4,test());
}