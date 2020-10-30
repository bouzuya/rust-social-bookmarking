use crate::bookmark::Bookmark;
use crate::bookmark_comment::BookmarkComment;
use crate::bookmark_repository::BookmarkRepository;
use crate::bookmark_title::BookmarkTitle;
use crate::bookmark_url::BookmarkUrl;
use crate::user_id::UserId;
use anyhow::Result;

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
    let bookmark = Bookmark::new(user_id, url, title, comment);
    println!("  user_id: {}", bookmark.user_id.to_i32());
    println!("  url: {}", bookmark.url.to_string());
    println!("  title: {}", bookmark.title.to_string());
    println!("  comment: {}", bookmark.comment.to_string());
    Ok(bookmark)
  }
}
