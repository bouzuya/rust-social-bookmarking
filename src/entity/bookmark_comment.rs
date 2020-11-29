use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookmarkComment(String);

impl TryFrom<&str> for BookmarkComment {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() <= 255 {
            Ok(BookmarkComment(s.to_owned()))
        } else {
            Err("Too long")
        }
    }
}

impl FromStr for BookmarkComment {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<BookmarkComment> for String {
    fn from(bookmark_comment: BookmarkComment) -> Self {
        bookmark_comment.0
    }
}
