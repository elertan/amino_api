use crate::api::v1::models::media_list_item::MediaListItem;
use crate::api::v1::types::zero_one_boolean::ZeroOneBoolean;
use crate::api::v1::models::user::User;
use crate::api::v1::types::timestamp::Timestamp;
use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::community::community::Community;
use crate::api::v1::models::api_response::ApiResponse;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProfileParams<'a> {
    pub uid: Uuid,
    pub icon: &'a str,
    pub nickname: &'a str,
    pub content: &'a str,
    pub media_list: Option<Vec<MediaListItem>>,
    pub enabled_chat_invite: bool,
    pub enabled_wall_commenting: bool,
}

impl<'a> Into<UpdateProfilePostData<'a>> for &UpdateProfileParams<'a> {
    fn into(self) -> UpdateProfilePostData<'a> {
        UpdateProfilePostData {
            nickname: self.nickname,
            content: self.content,
            icon: self.icon,
            media_list: self.media_list.clone(),
            timestamp: Timestamp::from_current_time(),
            extensions: UpdateProfileExtensions {
                style: None,
                __disabled_level__: 0,
                cover_animation: None,
                custom_titles: None,
                featured_type: 0,
                hide_user_profile: false,
                is_member_of_team_amino: false,
                privileged_of_chat_invite_request: ZeroOneBoolean::from_bool(self.enabled_chat_invite),
                privileged_of_comment_on_user_profile: ZeroOneBoolean::from_bool(self.enabled_wall_commenting),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProfileResult {
    #[serde(rename = "userProfile")]
    pub user_profile: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProfileExtensions {
    #[serde(rename = "privilegedOfChatInviteRequest")]
    pub privileged_of_chat_invite_request: ZeroOneBoolean,
    #[serde(rename = "privilegedOfCommentOnUserProfile")]
    pub privileged_of_comment_on_user_profile: ZeroOneBoolean,
    pub style: Option<serde_json::Value>,
    #[serde(rename = "__disabledLevel__")]
    pub __disabled_level__: u32,
    #[serde(rename = "coverAnimation")]
    pub cover_animation: Option<serde_json::Value>,
    #[serde(rename = "customTitles")]
    pub custom_titles: Option<serde_json::Value>,
    #[serde(rename = "isMemberOfTeamAmino")]
    pub is_member_of_team_amino: bool,
    #[serde(rename = "hideUserProfile")]
    pub hide_user_profile: bool,
    #[serde(rename = "featuredType")]
    pub featured_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProfilePostData<'a> {
    pub icon: &'a str,
    pub nickname: &'a str,
    pub content: &'a str,
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<MediaListItem>>,
    pub timestamp: Timestamp,
    pub extensions: UpdateProfileExtensions,
}

pub async fn update_profile(api: &ApiInstance, community: &Community, params: &UpdateProfileParams<'_>) -> Result<ApiResponse<UpdateProfileResult>, failure::Error> {
    let client = &api.client;
    let community_id = community.get_url_identifier();
    let partial_url = format!("{}/s/user-profile/{}", community_id.as_str(), &params.uid);
    let url = api.base_url.create_full_url(partial_url.as_str());

    let post_data: UpdateProfilePostData = params.into();
    let json_data = serde_json::to_string(&post_data).unwrap();
    dbg!(json_data);
    let response = client
        .post(&url)
        .json(&post_data)
        .send()
        .await?;
    dbg!(&response);
    let text = response.text().await?;
    dbg!(&text);
    let result = serde_json::from_str(&text).unwrap();
    dbg!(&result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::v1::api_instance::ApiInstance;
    use crate::api::v1::community::community::Community;

    #[test]
    fn update_profile_should_work() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance = rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);

//        let random_data = (0..9).map(|_| Uuid::new_v4())
//            .fold(String::with_capacity(10*36), |acc, x| acc + &x.to_string().replace("-", ""));

        let result: Result<ApiResponse<UpdateProfileResult>, failure::Error> = rt.block_on(
            update_profile(&api, &community, &UpdateProfileParams {
                uid: Uuid::parse_str("52c2c68a-d43d-4119-a4b1-09bccbbf92c7").unwrap(),
                nickname: "amino_api test",
                content: "amino_api test",
                enabled_chat_invite: true,
                enabled_wall_commenting: true,
                icon: "http://pm1.narvii.com/undefined.jpg",
                media_list: None,
            })
        );

        dbg!(&result);
        assert!(result.is_ok());
    }
}
