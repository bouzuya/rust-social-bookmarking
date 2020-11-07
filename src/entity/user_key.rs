use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UserKey(String);

impl UserKey {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        Self(format!("{}", rng.gen_range(0_i64, 999_999_999_999_i64)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut set = std::collections::HashSet::new();
        for _ in 0..100 {
            set.insert(UserKey::generate());
        }
        assert_eq!(set.len(), 100);
    }
}
