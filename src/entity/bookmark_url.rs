use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookmarkUrl(String);

impl TryFrom<&str> for BookmarkUrl {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() <= 2048 && (s.starts_with("http://") || s.starts_with("https://")) {
            Ok(BookmarkUrl(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for BookmarkUrl {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<BookmarkUrl> for String {
    fn from(bookmark_url: BookmarkUrl) -> Self {
        bookmark_url.0
    }
}
