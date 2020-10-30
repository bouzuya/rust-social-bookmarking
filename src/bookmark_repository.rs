use crate::bookmark::Bookmark;
use crate::bookmark_comment::BookmarkComment;
use crate::bookmark_title::BookmarkTitle;
use crate::bookmark_url::BookmarkUrl;
use crate::user_id::UserId;
use anyhow::Result;

pub trait BookmarkRepository {
  fn create(
    &self,
    user_id: UserId,
    url: BookmarkUrl,
    title: BookmarkTitle,
    comment: BookmarkComment,
  ) -> Result<Bookmark>;
}
