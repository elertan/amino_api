use crate::api::v1::models::api_info::ApiInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<TResult> {
    #[serde(flatten)]
    pub api_info: ApiInfo,
    #[serde(flatten)]
    pub result: TResult,
}
