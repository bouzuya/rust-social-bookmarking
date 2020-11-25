use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BookmarkKey(String);

impl BookmarkKey {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        Self(format!(
            "{:016}",
            rng.gen_range(0_i64, 999_999_999_999_999_i64)
        ))
    }
}

impl TryFrom<&str> for BookmarkKey {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() == 16 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(BookmarkKey(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for BookmarkKey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<BookmarkKey> for String {
    fn from(bookmark_key: BookmarkKey) -> Self {
        bookmark_key.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut set = std::collections::HashSet::new();
        for _ in 0..100 {
            let key = BookmarkKey::generate();
            assert_eq!(key.0.len(), 16);
            set.insert(key);
        }
        assert_eq!(set.len(), 100);
    }
}
