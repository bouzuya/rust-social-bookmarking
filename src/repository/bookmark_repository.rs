use crate::entity::bookmark::Bookmark;
use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;
use anyhow::Result;

pub trait UseBookmarkRepository {
  type BookmarkRepository: BookmarkRepository;
  fn bookmark_repository(&self) -> &Self::BookmarkRepository;
}

pub trait BookmarkRepository {
  fn create(
    &self,
    user_id: UserId,
    url: BookmarkUrl,
    title: BookmarkTitle,
    comment: BookmarkComment,
  ) -> Result<Bookmark>;

  fn find_by_user_key(&self, _: &UserKey) -> Result<Vec<Bookmark>>;
}
