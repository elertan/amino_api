use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::models::api_response::ApiResponse;
use crate::api::v1::community::community::Community;
use chrono::{Utc, DateTime};
use crate::api::v1::models::user::User;

#[derive(Debug, Clone)]
pub struct FetchRecentParams {
    start: Option<u32>,
    size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchRecentResult {
    #[serde(rename="userProfileCount")]
    pub user_profile_count: u32,
    #[serde(rename="userProfileList")]
    pub user_profile_list: Vec<User>,
}

pub async fn fetch_recent(
    api: &ApiInstance,
    community: &Community,
    params: &FetchRecentParams
) -> Result<ApiResponse<FetchRecentResult>, failure::Error> {
    let mut partial_url =
        format!(
            "{}/s/user-profile?size={}&type=recent",
            community.get_url_identifier(),
            params.size
        );
    if params.start.is_some() {
        let start = params.start.unwrap();
//        let stop_time: DateTime<Utc> = Utc::now();
//        let stop_time_string = format!("{:?}", stop_time);// stop_time.to_string();
//        let stop_time_string_encoded = urlencoding::encode(stop_time_string.as_str());
//        let part = format!("&stoptime={}&pagingType=o&start={}", stop_time_string_encoded, start);
        let part = format!("&pagingType=o&start={}", start);
        partial_url += part.as_str();
    }

    let url = api.base_url.create_full_url(&partial_url);
    let client = &api.client;
    let response = client.get(&url)
        .send()
        .await?;
    let response_text = response.text().await?;
    let result = serde_json::from_str(&response_text)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_recent_should_work() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance = rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);


        let result: Result<ApiResponse<FetchRecentResult>, failure::Error> = rt.block_on(
            fetch_recent(&api, &community, &FetchRecentParams {
                size: 25,
                start: None
            })
        );

        assert!(result.is_ok());
    }


    #[test]
    fn fetch_recent_with_some_should_work() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance = rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);

        let result: Result<ApiResponse<FetchRecentResult>, failure::Error> = rt.block_on(
            fetch_recent(&api, &community, &FetchRecentParams {
                size: 25,
                start: Some(25)
            })
        );

        dbg!(&result);
        assert!(result.is_ok());
    }
}