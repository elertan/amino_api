use uuid::Uuid;
use crate::api::v1::types::zero_one_boolean::ZeroOneBoolean;
use crate::api::v1::models::user::UserRole;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub uid: Uuid,
    #[serde(rename="membersQuota")]
    pub members_quota: u16,
    #[serde(rename="membersSummary")]
    pub members_summary: Vec<MemberSummary>,
    #[serde(rename="threadId")]
    pub thread_id: Uuid,
    pub keywords: serde_json::Value,
    #[serde(rename="membersCount")]
    pub members_count: u16,
    #[serde(rename="strategyInfo")]
    pub strategy_info: Option<String>,
    pub title: Option<String>,
    #[serde(rename="membershipStatus")]
    pub membership_status: ZeroOneBoolean,
    pub content: Option<String>,
    #[serde(rename="needHidden")]
    pub need_hidden: bool,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    #[serde(rename="alertOption")]
    pub alert_option: u8,
    #[serde(rename="lastReadTime")]
    pub last_read_time: DateTime<Utc>,
    pub r#type: Option<u8>,
    pub status: Option<u8>,
    #[serde(rename="publishToGlobal")]
    pub publish_to_global: ZeroOneBoolean,
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<DateTime<Utc>>,
    #[serde(rename="lastMessageSummary")]
    pub last_message_summary: LastMessageSummary,
    pub condition: Option<u8>,
    pub icon: Option<String>,
    #[serde(rename="latestActivityTime")]
    pub latest_activity_time: Option<DateTime<Utc>>,
    pub author: ThreadAuthor,
    pub extensions: serde_json::Value,
    #[serde(rename="ndcId")]
    pub ndc_id: u32,
    #[serde(rename="createdTime")]
    pub created_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberSummary {
    pub status: Option<u8>,
    pub uid: Uuid,
    #[serde(rename="membershipStatus")]
    pub membership_status: Option<ZeroOneBoolean>,
    pub role: UserRole,
    pub nickname: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastMessageSummary {
    pub uid: Uuid,
    #[serde(rename="mediaType")]
    pub media_type: Option<String>,
    pub content: Option<String>,
    #[serde(rename="messageId")]
    pub message_id: Uuid,
    #[serde(rename="createdTime")]
    pub created_time: DateTime<Utc>,
    pub r#type: u16,
    #[serde(rename="mediaValue")]
    pub media_value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadAuthor {
    pub status: u32,
    pub uid: Uuid,
    pub level: u8,
    #[serde(rename="accountMembershipStatus")]
    pub account_membership_status: Option<u8>,
    pub reputation: i32,
    pub role: UserRole,
    pub nickname: String,
    pub icon: Option<String>,
}
