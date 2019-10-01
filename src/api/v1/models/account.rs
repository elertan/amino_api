use crate::api::v1::models::zero_one_boolean::ZeroOneBoolean;
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
}
