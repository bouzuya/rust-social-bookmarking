pub struct MailAddress(String);

impl MailAddress {
    pub fn from_str(s: &str) -> Option<Self> {
        // TODO: validation
        Some(MailAddress(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
