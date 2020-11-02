use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone)]
pub struct BookmarkKey(String);

impl BookmarkKey {
    pub fn generate() -> Self {
        // TODO: generate key
        Self("123456789012".into())
    }

    pub fn to_string(&self) -> String {
        self.0.to_owned()
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
