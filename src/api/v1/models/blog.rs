use crate::api::v1::models::media_list_item::MediaListItem;
use uuid::Uuid;
use crate::api::v1::models::tip_info::TipInfo;
use chrono::{DateTime, Utc};

/*
{
		"globalVotesCount": 0,
		"globalVotedValue": 0,
		"votedValue": 0,
		"keywords": "",
		"strategyInfo": "{\"scenarioType\": \"latest-post-feed\", \"objectId\": \"efaef436-54c2-4712-b2f2-a535d8932fd9\", \"reqId\": \"985f9af6-eac0-4ca0-ba39-e8442b9932ba\", \"ndcId\": 3, \"objectSubType\": \"blog\", \"uiPos\": 0, \"objectType\": \"blog\"}",
		"mediaList": [
			[100, "http:\/\/pm1.narvii.com\/7336\/66ebce0f541c48b43cd563c47da9a184a1edc354r1-900-900v2_00.jpg", null]
		],
		"style": 0,
		"totalQuizPlayCount": 0,
		"title": "Unnecessary Thoughts",
		"tipInfo": {
			"tipOptionList": [{
				"value": 2,
				"icon": "http:\/\/static.narvii.com\/monetization\/product\/handful_of_coins_v4.png"
			}, {
				"value": 10,
				"icon": "http:\/\/static.narvii.com\/monetization\/product\/stack_of_coins_v4.png"
			}, {
				"value": 50,
				"icon": "http:\/\/static.narvii.com\/monetization\/product\/tall_stack_of_coins_v4.png"
			}],
			"tipMaxCoin": 500,
			"tippersCount": 0,
			"tippable": true,
			"tipMinCoin": 1,
			"tipCustomOption": {
				"value": null,
				"icon": "http:\/\/static.narvii.com\/monetization\/product\/bag_of_coins_v4.png"
			},
			"tippedCoins": 0
		},
		"contentRating": 0,
		"content": "[C]\u201dI love you\u201d\n[CI]\u201dPlease don\u2019t love someone else\u201d",
		"needHidden": false,
		"guestVotesCount": 0,
		"type": 0,
		"status": 0,
		"globalCommentsCount": 0,
		"modifiedTime": "2019-10-04T21:32:49Z",
		"widgetDisplayInterval": null,
		"totalPollVoteCount": 0,
		"blogId": "efaef436-54c2-4712-b2f2-a535d8932fd9",
		"viewCount": 0,
		"author": {
			"status": 0,
			"isNicknameVerified": false,
			"uid": "0fc48051-48c3-4b56-a5f9-abf01fab172c",
			"level": 11,
			"followingStatus": 0,
			"accountMembershipStatus": 0,
			"isGlobal": false,
			"membershipStatus": 0,
			"reputation": 4050,
			"role": 0,
			"ndcId": 3,
			"nickname": "Hishiro",
			"icon": "http:\/\/pm1.narvii.com\/7298\/1f27abb0d3172b8896820d150644c8c584145079r1-438-438v2_00.jpg"
		},
		"extensions": {
			"style": {
				"backgroundColor": "#000000FF"
			}
		},
		"votesCount": 12,
		"ndcId": 3,
		"createdTime": "2019-10-04T21:32:49Z",
		"endTime": null,
		"commentsCount": 1
	}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
    #[serde(rename = "blogId")]
    pub blog_id: Uuid,
    pub title: Option<String>,
    #[serde(rename = "globalVotesCount")]
    pub global_votes_count: u32,
    #[serde(rename = "globalVotedValue")]
    pub global_voted_value: u32,
    #[serde(rename = "votedValue")]
    pub voted_value: u32,
    #[serde(rename = "strategyInfo")]
    pub strategy_info: Option<String>,
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<MediaListItem>>,
    pub style: u32,
    #[serde(rename = "totalQuizPlayCount")]
    pub total_quiz_play_count: u32,
    #[serde(rename = "tipInfo")]
    pub tip_info: TipInfo,
    #[serde(rename = "contentRating")]
    pub content_rating: u32,
    pub content: Option<String>,
    #[serde(rename="needHidden")]
    pub need_hidden: bool,
    #[serde(rename="guestVotesCount")]
    pub guest_votes_count: u32,
    pub r#type: u32,
    pub status: u32,
    #[serde(rename="globalCommentsCount")]
    pub global_comments_count: u32,
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<DateTime<Utc>>,
    #[serde(rename="widgetDisplayInterval")]
    pub widget_display_interval: Option<serde_json::Value>,
    #[serde(rename="totalPollVoteCount")]
    pub total_poll_vote_count: u32,
    #[serde(rename="viewCount")]
    pub view_count: u32,
    pub author: BlogAuthor,
    pub extensions: serde_json::Value,
    #[serde(rename="votesCount")]
    pub votes_count: u32,
    #[serde(rename="ndcId")]
    pub ndc_id: u32,
    #[serde(rename="createdTime")]
    pub created_time: DateTime<Utc>,
    #[serde(rename="endTime")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(rename="commentsCount")]
    pub comments_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogAuthor {
    pub status: u32,
    #[serde(rename="isNicknameVerified")]
    pub is_nickname_verified: bool,
    pub uid: Uuid,
    pub level: u32,
    #[serde(rename="followingStatus")]
    pub following_status: u32,
    #[serde(rename="accountMembershipStatus")]
    pub account_membership_status: u32,
    #[serde(rename="isGlobal")]
    pub is_global: bool,
    #[serde(rename="membershipStatus")]
    pub membership_status: u32,
    pub reputation: u32,
    pub role: u32,
    #[serde(rename="ndcId")]
    pub ndc_id: u32,
    pub nickname: String,
    pub icon: String,
}
