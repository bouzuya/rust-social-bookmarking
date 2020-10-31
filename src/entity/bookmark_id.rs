use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BookmarkId(i32);

impl TryFrom<i32> for BookmarkId {
    type Error = &'static str;
    fn try_from(i: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        if i >= 1 {
            Ok(BookmarkId(i))
        } else {
            Err("BookmarkId >= 1")
        }
    }
}

impl From<BookmarkId> for i32 {
    fn from(id: BookmarkId) -> Self {
        id.0
    }
}
