#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZeroOneBoolean(u8);

impl ZeroOneBoolean {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn as_bool(&self) -> bool {
        self.0 == 1
    }
}

