#[derive(Clone)]
pub struct Password(String);

impl Password {
    pub fn from_str(s: &str) -> Option<Self> {
        // TODO: validation
        Some(Password(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
