pub struct Community {
    id: u32,
}

impl Community {
    pub fn from_id(id: u32) -> Self {
        Self { id }
    }

    pub fn get_url_identifier(&self) -> String {
        format!("x{}", self.id)
    }
}
