use crate::bookmark_comment::BookmarkComment;
use crate::bookmark_title::BookmarkTitle;
use crate::bookmark_url::BookmarkUrl;
use crate::user_id::UserId;

pub struct Bookmark {
  pub user_id: UserId,
  pub url: BookmarkUrl,
  pub comment: BookmarkComment,
  pub title: BookmarkTitle,
}

impl Bookmark {
  pub fn new(
    user_id: UserId,
    url: BookmarkUrl,
    title: BookmarkTitle,
    comment: BookmarkComment,
  ) -> Self {
    Self {
      user_id,
      url,
      title,
      comment,
    }
  }
}
