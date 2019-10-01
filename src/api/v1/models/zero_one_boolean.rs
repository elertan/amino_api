#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZeroOneBoolean(u8);

impl Into<bool> for ZeroOneBoolean {
    fn into(self) -> bool {
        self.0 == 1
    }
}
