use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiInfo {
    #[serde(rename = "api:duration")]
    pub duration: String,
    #[serde(rename = "api:message")]
    pub message: String,
    #[serde(rename = "api:statuscode")]
    pub status_code: u32,
    #[serde(rename = "api:timestamp")]
    pub timestamp: DateTime<Utc>,
}
