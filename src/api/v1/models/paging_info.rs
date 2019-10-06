#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PagingInfo {
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
