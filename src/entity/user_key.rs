#[derive(Clone)]
pub struct UserKey(String);

impl UserKey {
    pub fn generate() -> Self {
        // TODO: generate key
        Self("user-key1".into())
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Some(Self(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
