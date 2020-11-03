use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifyUserSecret(String);

impl VerifyUserSecret {
    pub fn generate() -> Self {
        // TODO: generate secret
        VerifyUserSecret("1234567890".repeat(25) + "abcde")
    }
}

impl TryFrom<&str> for VerifyUserSecret {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() == 255 && s.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(VerifyUserSecret(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for VerifyUserSecret {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<VerifyUserSecret> for String {
    fn from(user_key: VerifyUserSecret) -> Self {
        user_key.0
    }
}
