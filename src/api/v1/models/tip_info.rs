#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TipInfo {
    #[serde(rename = "tipMaxCoin")]
    pub tip_max_coin: f32,
    #[serde(rename = "tippersCount")]
    pub tippers_count: f32,
    pub tippable: bool,
    #[serde(rename = "tipMinCoin")]
    pub tip_min_coin: f32,
    #[serde(rename = "tipCustomOption")]
    pub tip_custom_option: TipOption,
    #[serde(rename = "tippedCoins")]
    pub tipped_coins: f32,
    #[serde(rename = "tipOptionList")]
    pub tip_option_list: Vec<TipOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TipOption {
    pub value: Option<f32>,
    pub icon: String,
}