use reqwest::Client;

#[derive(Debug)]
pub struct ApiInstance {
    pub client: Client,
}

impl ApiInstance {
    pub fn new() -> Self {
        let client = Client::new();

        Self {
            client
        }
    }
}