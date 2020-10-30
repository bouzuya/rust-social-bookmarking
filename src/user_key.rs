pub struct UserKey(String);

impl UserKey {
    pub fn generate() -> Self {
        // TODO: generate key
        Self("user-key1".into())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
