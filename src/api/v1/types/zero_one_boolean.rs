#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZeroOneBoolean(u8);

impl ZeroOneBoolean {
    pub fn from_bool(value: bool) -> Self {
        Self(match value {
            true => 1,
            false => 0
        })
    }

    pub fn as_bool(&self) -> bool {
        self.0 == 1
    }
}

