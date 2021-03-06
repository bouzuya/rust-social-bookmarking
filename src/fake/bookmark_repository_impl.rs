use crate::entity::{
    Bookmark, BookmarkComment, BookmarkId, BookmarkKey, BookmarkTitle, BookmarkUrl, UserId,
};
use crate::repository::BookmarkRepository;
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
        println!("  bookmark: {:?}", bookmark);
        Ok(bookmark)
    }

    fn delete(&self, _: &BookmarkId) -> Result<()> {
        todo!()
    }

    fn delete_by_user_id(&self, _: &UserId) -> Result<()> {
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
