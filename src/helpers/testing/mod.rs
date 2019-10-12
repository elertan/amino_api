use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::global::s::auth::login::login_with_email::{login_with_email, LoginWithEmailParams};

pub fn load_test_env() {
    dotenv::from_filename("tests.env")
        .unwrap_or_else(|_| panic!("tests.env file has not being created or couldn't be loaded"));
}

pub async fn get_authorized_v1_api_instance() -> ApiInstance {
    let email =
        dotenv::var("API_V1_LOGIN_EMAIL")
            .expect("env incorrect");
    let password =
        dotenv::var("API_V1_LOGIN_PASSWORD")
            .expect("env incorrect");

    let result = login_with_email(&LoginWithEmailParams {
        email: &email,
        password: &password
    }).await.unwrap_or_else(|_| panic!("Could not get authorized api instance, login failed."));

    result.1
}
