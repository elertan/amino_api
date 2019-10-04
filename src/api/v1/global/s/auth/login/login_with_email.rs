use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::models::account::Account;
use crate::api::v1::models::api_response::ApiResponse;
use uuid::Uuid;
use crate::api::v1::models::user::User;
use crate::api::v1::types::device_id::DeviceId;
use crate::api::v1::types::timestamp::Timestamp;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginWithEmailParams<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginWithEmailResult {
    pub account: Option<Account>,
    pub auid: Option<Uuid>,
    pub secret: Option<String>,
    pub sid: Option<String>,
    #[serde(rename="userProfile")]
    pub user_profile: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginWithEmailPostData {
    pub secret: String,
    #[serde(rename = "clientType")]
    pub client_type: u8,
    #[serde(rename = "systemPushEnabled")]
    pub system_push_enabled: u8,
    pub timestamp: Timestamp,
    pub locale: String,
    pub action: String,
    #[serde(rename = "bundleID")]
    pub bundle_id: String,
    pub timezone: i32,
    #[serde(rename = "deviceID")]
    pub device_id: DeviceId,
    pub email: String,
    pub v: u8,
    #[serde(rename = "clientCallbackURL")]
    pub client_callback_url: String,
}

impl<'a> From<&LoginWithEmailParams<'a>> for LoginWithEmailPostData {
    fn from(params: &LoginWithEmailParams<'a>) -> Self {
        Self {
            secret: format!("0 {}", params.password),
            client_type: 100,
            system_push_enabled: 0,
            timestamp: Timestamp::from_current_time(),
            locale: "en_US".to_string(),
            action: "normal".to_string(),
            bundle_id: "com.narvii.master".to_string(),
            timezone: -420,
            device_id: DeviceId::from_str("01760e21ca2ce3a5e012738ce519e8cafd51476d3bfd91f86fd3c04292fdf3d700694e4f255fe6fd92"),
            email: String::from(params.email),
            v: 2,
            client_callback_url: r#"narviiapp://default"#.to_string(),
        }
    }
}

pub async fn login_with_email<'a>(
    api_instance: &mut ApiInstance,
    params: &'a LoginWithEmailParams<'a>,
) -> Result<ApiResponse<LoginWithEmailResult>, failure::Error> {
    let url = api_instance.base_url.create_full_url("g/s/auth/login");
    let client = &api_instance.client;

    let post_data: LoginWithEmailPostData = params.into();
    let result = client
        .post(&url)
        .json(&post_data)
        .send()
        .await?
        .json()
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_should_fail() {
        let mut api_instance = ApiInstance::default();
        let params = LoginWithEmailParams {
            email: "test_email_blabladasdaw1231@adasdaasdaerew.com",
            password: "asdnasdbahsnjdasbdas",
        };
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
        let result = rt.block_on(login_with_email(&mut api_instance, &params));
        dbg!(&result);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.api_info.message, "Account does not exist.".to_string());
    }

    #[test]
    fn login_should_work() {
        crate::helpers::testing::load_test_env();
        let email =
            dotenv::var("API_V1_GLOBAL_S_AUTH_LOGIN_LOGIN_WITH_EMAIL_LOGIN_SHOULD_WORK_EMAIL")
                .expect("env incorrect");
        let password =
            dotenv::var("API_V1_GLOBAL_S_AUTH_LOGIN_LOGIN_WITH_EMAIL_LOGIN_SHOULD_WORK_PASSWORD")
                .expect("env incorrect");

        let mut api_instance = ApiInstance::default();
        let params = LoginWithEmailParams {
            email: &email,
            password: &password,
        };
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
        let result = rt.block_on(login_with_email(&mut api_instance, &params));
        dbg!(&result);
        assert!(result.is_ok());
        let data: ApiResponse<LoginWithEmailResult> = result.unwrap();
        assert_eq!(data.api_info.message, "OK".to_string());
    }
}
