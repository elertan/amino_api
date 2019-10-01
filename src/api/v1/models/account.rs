use crate::api::v1::models::user::UserRole;
use crate::api::v1::models::zero_one_boolean::ZeroOneBoolean;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(u8)]
pub enum AccountStatus {
    Unknown = 0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountAdvancedSettings {
    #[serde(rename = "amplitudeAnalyticsEnabled")]
    pub amplitude_analytics_enabled: ZeroOneBoolean,
    #[serde(rename = "amplitudeAppId")]
    pub amplitude_app_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    #[serde(rename = "lastClientType")]
    last_client_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountExtensions {
    #[serde(rename = "adsLevel")]
    pub ads_level: u8,
    #[serde(rename = "adsEnabled")]
    pub ads_enabled: bool,
    #[serde(rename = "deviceInfo")]
    pub device_info: DeviceInfo,
    #[serde(rename = "adsFlags")]
    pub ads_flags: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub status: AccountStatus,
    pub uid: Uuid,
    #[serde(rename = "phoneNumberActivation")]
    pub phone_number_activation: ZeroOneBoolean,
    #[serde(rename = "emailActivation")]
    pub email_activation: ZeroOneBoolean,
    #[serde(rename = "facebookID")]
    pub facebook_id: Option<String>,
    #[serde(rename = "advancedSettings")]
    pub advanced_settings: AccountAdvancedSettings,
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<serde_json::Value>>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<DateTime<Utc>>,
    pub role: UserRole,
    #[serde(rename = "aminoId")]
    pub amino_id: String,
    pub latitude: Option<i64>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "modifiedTime")]
    pub modified_time: Option<DateTime<Utc>>,
    #[serde(rename = "twitterID")]
    pub twitter_id: Option<String>,
    pub activation: ZeroOneBoolean,
    pub membership: Option<serde_json::Value>,
    pub address: Option<serde_json::Value>,
    pub nickname: String,
    #[serde(rename = "googleID")]
    pub google_id: Option<String>,
    pub icon: String,
    #[serde(rename = "securityLevel")]
    pub security_level: u8,
    pub gender: Option<serde_json::Value>,
    pub longitude: Option<i64>,
    pub extensions: AccountExtensions,
    #[serde(rename = "aminoIdEditable")]
    pub amino_id_editable: bool,
    #[serde(rename = "createdTime")]
    pub created_time: DateTime<Utc>,
}
