use crate::entity::bookmark_key::BookmarkKey;
use anyhow::Result;

pub trait UseDeleteBookmarkUseCase {
    type DeleteBookmarkUseCase: DeleteBookmarkUseCase;
    fn delete_bookmark_use_case(&self) -> &Self::DeleteBookmarkUseCase;
}

pub trait DeleteBookmarkUseCase {
    fn delete_bookmark(&self, _: BookmarkKey) -> Result<()> {
        todo!()
    }
}

impl<T> DeleteBookmarkUseCase for T {}
