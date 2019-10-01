use reqwest::r#async::Client;

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

impl Default for ApiInstance {
    fn default() -> Self {
        let client = Client::new();

        Self {
            client,
            base_url: BaseUrl("https://service.narvii.com/api/v1/"),
        }
    }
}
