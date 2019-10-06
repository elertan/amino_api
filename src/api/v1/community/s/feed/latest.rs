use crate::api::v1::api_instance::ApiInstance;
use crate::api::v1::community::community::Community;
use crate::api::v1::models::api_response::ApiResponse;
use crate::api::v1::models::blog::Blog;
use crate::api::v1::models::paging_info::PagingInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchLatestPostsParams<'a> {
    size: u32,
    paging_token: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchLatestPostsResult {
    paging: PagingInfo,
    #[serde(rename="blogList")]
    blog_list: Vec<Blog>,
}

pub async fn fetch_latest_posts<'a>(
    api: &ApiInstance,
    community: &Community,
    params: &FetchLatestPostsParams<'a>,
) -> Result<ApiResponse<FetchLatestPostsResult>, failure::Error> {
    let client = &api.client;
    let mut partial_url = format!("{}/s/feed/blog-all?pagingType=t&size={}", community.get_url_identifier(), params.size);
    if params.paging_token.is_some() {
        partial_url += format!("&pageToken={}", params.paging_token.unwrap()).as_str();
    }
    let url = api.base_url.create_full_url(partial_url.as_str());

    let result = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_posts_without_paging_token_should_work() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance =  rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);

        let result: Result<ApiResponse<FetchLatestPostsResult>, failure::Error> = rt.block_on(
            fetch_latest_posts(&api, &community, &FetchLatestPostsParams {
                size: 25,
                paging_token: None
            })
        );

        dbg!(&result);
        assert!(result.is_ok());
    }

    #[test]
    fn fetch_posts_with_paging_token_should_work() {
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        crate::helpers::testing::load_test_env();
        let api: ApiInstance =  rt.block_on(crate::helpers::testing::get_authorized_v1_api_instance());
        let community = Community::from_id(3);

        let result: Result<ApiResponse<FetchLatestPostsResult>, failure::Error> = rt.block_on(
            fetch_latest_posts(&api, &community, &FetchLatestPostsParams {
                size: 25,
                paging_token: None
            })
        );

        dbg!(&result);
        assert!(result.is_ok());

        let paging_token = result.unwrap().result.paging.next_page_token.unwrap();

        let next_result: Result<ApiResponse<FetchLatestPostsResult>, failure::Error> = rt.block_on(
            fetch_latest_posts(&api, &community, &FetchLatestPostsParams {
                size: 25,
                paging_token: Some(paging_token.as_str())
            })
        );

        dbg!(&next_result);
        assert!(next_result.is_ok());
    }
}
