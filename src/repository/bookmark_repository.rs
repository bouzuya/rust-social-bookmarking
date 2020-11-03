use crate::entity::bookmark::Bookmark;
use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_id::BookmarkId;
use crate::entity::bookmark_key::BookmarkKey;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;
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

  fn delete(&self, _: &BookmarkId) -> Result<()>;

  fn find_by_key(&self, _: &BookmarkKey) -> Result<Option<Bookmark>>;

  fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Bookmark>>;

  fn save(&self, bookmark: &Bookmark) -> Result<()>;
}
