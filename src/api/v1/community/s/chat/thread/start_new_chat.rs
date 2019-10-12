use uuid::Uuid;
use crate::api::v1::types::timestamp::Timestamp;
use crate::api::v1::models::message::Message;
use crate::api::v1::models::thread::Thread;
use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::models::api_response::ApiResponse;
use crate::api::v1::community::community::Community;

#[derive(Debug, Clone)]
pub struct StartNewChatParams<'a> {
    pub invitee_uids: Vec<Uuid>,
    pub initial_message: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartNewChatPostData {
    pub timestamp: Timestamp,
    pub r#type: u8,
    #[serde(rename="initialMessageContent")]
    pub initial_message_content: String,
    #[serde(rename="inviteeUids")]
    pub invitee_uids: Vec<String>,
}

impl Into<StartNewChatPostData> for &StartNewChatParams<'_> {
    fn into(self) -> StartNewChatPostData {
        StartNewChatPostData {
            timestamp: Timestamp::from_current_time(),
            r#type: 0,
            initial_message_content: self.initial_message.to_string(),
            invitee_uids: self.invitee_uids.iter().map(|invitee_uid| invitee_uid.to_string()).collect()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartNewChatResult {
    pub thread: Thread,
    #[serde(rename="messageList")]
    pub message_list: Vec<Message>,
}

pub async fn start_new_chat(api: &ApiInstance, community: &Community, params: &StartNewChatParams<'_>) -> Result<ApiResponse<StartNewChatResult>, failure::Error> {
    let client = &api.client;
    let community_id = community.get_url_identifier();
    let partial_url = format!("{}/s/chat/thread", community_id.as_str());
    let url = api.base_url.create_full_url(partial_url.as_str());

    let post_data: StartNewChatPostData = params.into();
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
    use std::str::FromStr;

    #[test]
    fn should_work_on_me() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance = rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);

        let elertan_uuid = Uuid::from_str("5f00ff5e-d51a-4d07-b2bf-7629b2626e1d").unwrap();

        let result: Result<ApiResponse<StartNewChatResult>, failure::Error> = rt.block_on(
            start_new_chat(&api, &community, &StartNewChatParams {
                initial_message: "Hi!",
                invitee_uids: vec![elertan_uuid]
            })
        );

        dbg!(&result);
        assert!(result.is_ok());
    }
}