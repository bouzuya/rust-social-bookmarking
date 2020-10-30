use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;

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
