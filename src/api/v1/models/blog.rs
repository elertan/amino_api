use crate::api::v1::models::media_list_item::MediaListItem;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
    #[serde(rename = "blogId")]
    pub blog_id: Uuid,
    pub title: Option<String>,
    #[serde(rename = "globalVotesCount")]
    pub global_votes_count: u32,
    #[serde(rename = "globalVotedValue")]
    pub global_voted_value: u32,
    #[serde(rename = "votedValue")]
    pub voted_value: u32,
    #[serde(rename = "strategyInfo")]
    pub strategy_info: Option<String>,
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<MediaListItem>>,
}
