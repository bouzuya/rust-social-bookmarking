use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookmarkTitle(String);

impl TryFrom<&str> for BookmarkTitle {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() <= 255 {
            Ok(BookmarkTitle(s.to_owned()))
        } else {
            Err("Too long")
        }
    }
}

impl FromStr for BookmarkTitle {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<BookmarkTitle> for String {
    fn from(bookmark_title: BookmarkTitle) -> Self {
        bookmark_title.0
    }
}
