use crate::api::v1::models::account::AccountExtensions;
use crate::api::v1::types::zero_one_boolean::ZeroOneBoolean;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserRole {
    User = 0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub status: u32,
    #[serde(rename = "moodSticker")]
    pub mood_sticker: Option<serde_json::Value>,
    #[serde(rename = "itemsCount")]
    pub items_count: u32,
    #[serde(rename = "consecutiveCheckInDays")]
    pub consecutive_check_in_days: Option<serde_json::Value>,
    pub uid: Uuid,
    #[serde(rename = "modifiedTime")]
    pub modified_time: Option<DateTime<Utc>>,
    #[serde(rename = "followingStatus")]
    pub following_status: u32,
    #[serde(rename = "onlineStatus")]
    pub online_status: ZeroOneBoolean,
    #[serde(rename = "accountMembershipStatus")]
    pub account_membership_status: ZeroOneBoolean,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "postsCount")]
    pub posts_count: u32,
    pub race: Option<serde_json::Value>,
    #[serde(rename = "membersCount")]
    pub members_count: u32,
    pub nickname: String,
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<serde_json::Value>>,
    pub reputation: u32,
    #[serde(rename = "isNicknameVerified")]
    pub is_nickname_verified: bool,
    pub mood: Option<serde_json::Value>,
    pub level: u32,
    pub gender: Option<serde_json::Value>,
    #[serde(rename = "notificationSubscriptionStatus")]
    pub notification_subscription_status: u8,
    #[serde(rename = "pushEnabled")]
    pub push_enabled: bool,
    #[serde(rename = "membershipStatus")]
    pub membership_status: u8,
    pub content: Option<serde_json::Value>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<DateTime<Utc>>,
    #[serde(rename = "joinedCount")]
    pub joined_count: u32,
    pub role: UserRole,
    #[serde(rename = "commentsCount")]
    pub comments_count: u32,
    #[serde(rename = "aminoId")]
    pub amino_id: String,
    #[serde(rename = "ndcId")]
    pub ndc_id: u32,
    #[serde(rename = "createdTime")]
    pub created_time: DateTime<Utc>,
    pub extensions: Option<AccountExtensions>,
    pub icon: String,
    #[serde(rename = "storiesCount")]
    pub stories_count: u32,
    pub age: Option<u32>,
    #[serde(rename = "blogsCount")]
    pub blogs_count: u32,
}
