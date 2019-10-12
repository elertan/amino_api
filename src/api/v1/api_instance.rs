use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct BaseUrl(&'static str);

impl<'a> BaseUrl {
    pub fn new(url: &'static str) -> Self {
        BaseUrl(url)
    }

    pub fn create_full_url(&self, path: &'a str) -> String {
        format!("{}{}", self.0, path)
    }
}

#[derive(Debug)]
pub struct ApiInstance {
    pub client: Client,
    pub base_url: BaseUrl,
}

pub struct LoginData {
    pub device_id: String,
    pub sid: String,
    pub auid: String,
}

impl ApiInstance {
    pub fn with_login_data(login_data: &LoginData) -> Result<Self, failure::Error> {
        let mut header_map = HeaderMap::new();
        let sid_header_value = format!("sid={}", login_data.sid);
        header_map.append(HeaderName::from_str("ndcauth")?, HeaderValue::from_str(sid_header_value.as_str())?);
        header_map.append(HeaderName::from_str("auid")?, HeaderValue::from_str(login_data.auid.as_str())?);
        header_map.append(HeaderName::from_str("ndcdeviceid")?, HeaderValue::from_str(login_data.device_id.as_str())?);
        header_map.append(HeaderName::from_str("ndclang")?, HeaderValue::from_str("en")?);

        let client = Client::builder()
            .default_headers(header_map)
            .build()?;

        Ok(Self {
            client,
            base_url: BaseUrl("https://service.narvii.com/api/v1/"),
        })
    }
}

impl Default for ApiInstance {
    fn default() -> Self {
        let client = Client::new();

        Self {
            client,
            base_url: BaseUrl("https://service.narvii.com/api/v1/"),
        }
    }
}
