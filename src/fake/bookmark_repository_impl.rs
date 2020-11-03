use crate::entity::bookmark::Bookmark;
use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_id::BookmarkId;
use crate::entity::bookmark_key::BookmarkKey;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;
use crate::repository::bookmark_repository::BookmarkRepository;
use anyhow::Result;
use std::convert::TryFrom;

pub struct BookmarkRepositoryImpl;

impl BookmarkRepositoryImpl {
  pub fn new() -> Self {
    Self
  }
}

impl BookmarkRepository for BookmarkRepositoryImpl {
  fn create(
    &self,
    user_id: UserId,
    url: BookmarkUrl,
    title: BookmarkTitle,
    comment: BookmarkComment,
  ) -> Result<Bookmark> {
    println!("create bookmark");
    let bookmark_id = BookmarkId::try_from(1).unwrap();
    let bookmark = Bookmark::new(bookmark_id, user_id, url, title, comment);
    println!("  id     : {:?}", bookmark.id);
    println!("  user_id: {:?}", bookmark.user_id);
    println!("  url    : {:?}", bookmark.url);
    println!("  title  : {:?}", bookmark.title);
    println!("  comment: {:?}", bookmark.comment);
    Ok(bookmark)
  }

  fn delete(&self, _: &BookmarkId) -> Result<()> {
    todo!()
  }

  fn find_by_key(&self, _: &BookmarkKey) -> Result<Option<Bookmark>> {
    todo!()
  }

  fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Bookmark>> {
    todo!()
  }

  fn save(&self, _: &Bookmark) -> Result<()> {
    todo!()
  }
}
