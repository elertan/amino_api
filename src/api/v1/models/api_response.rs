#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<TResult> {
    pub code: i16,
    pub result: TResult
}
