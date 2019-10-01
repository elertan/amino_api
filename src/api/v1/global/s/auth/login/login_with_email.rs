use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::models::api_info::ApiInfo;
use crate::api::v1::models::api_response::ApiResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginWithEmailParams<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginWithEmailResult {
    #[serde(flatten)]
    pub api_info: ApiInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginWithEmailPostData {
    pub secret: String,
    #[serde(rename = "clientType")]
    pub client_type: u8,
    #[serde(rename = "systemPushEnabled")]
    pub system_push_enabled: u8,
    pub timestamp: u64,
    pub locale: String,
    pub action: String,
    #[serde(rename = "bundleID")]
    pub bundle_id: String,
    pub timezone: i32,
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub email: String,
    pub v: u8,
    #[serde(rename = "clientCallbackURL")]
    pub client_callback_url: String,
}

pub fn login_with_email<'a>(
    api_instance: &mut ApiInstance,
    params: &'a LoginWithEmailParams,
) -> Result<ApiResponse<LoginWithEmailResult>, failure::Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_should_fail() {
        let mut api_instance = ApiInstance::new();
        let params = LoginWithEmailParams {
            email: "test_email_blabladasdaw1231@adasdaasdaerew.com",
            password: "asdnasdbahsnjdasbdas",
        };
        let result = login_with_email(&mut api_instance, &params);
        assert!(result.is_ok());
    }
}
