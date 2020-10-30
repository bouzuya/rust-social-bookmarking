use crate::entity::bookmark::Bookmark;
use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;
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
