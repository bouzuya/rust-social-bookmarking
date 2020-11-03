use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct UserKey(String);

impl UserKey {
    pub fn generate() -> Self {
        // TODO: generate key
        Self("123456789012".into())
    }
}

impl TryFrom<&str> for UserKey {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() == 12 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(UserKey(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for UserKey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<UserKey> for String {
    fn from(user_key: UserKey) -> Self {
        user_key.0
    }
}
