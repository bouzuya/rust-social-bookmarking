#[derive(Clone, PartialEq, Eq)]
pub struct VerifyUserSecret(String);

impl VerifyUserSecret {
    pub fn generate() -> Self {
        // TODO: generate secret
        VerifyUserSecret("verify-user-secret1".into())
    }

    pub fn from_str(s: &str) -> Option<Self> {
        // TODO: validate
        Some(Self(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
