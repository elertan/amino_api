use uuid::Uuid;
use crate::api::v1::types::zero_one_boolean::ZeroOneBoolean;
use crate::api::v1::models::user::UserRole;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename="includedSummary")]
    pub included_summary: Option<bool>,
    pub uid: Uuid,
    pub author: MessageAuthor,
    #[serde(rename="threadId")]
    pub thread_id: Uuid,
    #[serde(rename="mediaType")]
    pub media_type: Option<String>,
    pub content: Option<String>,
    #[serde(rename="clientRefId")]
    pub client_ref_id: u32,
    #[serde(rename="messageId")]
    pub message_id: Uuid,
    #[serde(rename="createdTime")]
    pub created_time: DateTime<Utc>,
    pub extensions: serde_json::Value,
    pub r#type: u32,
    #[serde(rename="mediaValue")]
    pub media_value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageAuthor {
    pub status: u32,
    #[serde(rename="isNicknameVerified")]
    pub is_nickname_verified: bool,
    pub uid: Uuid,
    pub level: u8,
    #[serde(rename="accountMembershipStatus")]
    pub account_membership_status: u8,
    #[serde(rename="membershipStatus")]
    pub membership_status: Option<ZeroOneBoolean>,
    pub reputation: i32,
    pub role: UserRole,
    pub nickname: String,
    pub icon: Option<String>,
}