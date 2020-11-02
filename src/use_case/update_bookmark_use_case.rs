use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_key::BookmarkKey;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use anyhow::Result;

pub trait UseUpdateBookmarkUseCase {
    type UpdateBookmarkUseCase: UpdateBookmarkUseCase;
    fn update_bookmark_use_case(&self) -> &Self::UpdateBookmarkUseCase;
}

pub trait UpdateBookmarkUseCase {
    fn update_bookmark(
        &self,
        _: BookmarkKey,
        _: BookmarkUrl,
        _: BookmarkTitle,
        _: BookmarkComment,
    ) -> Result<()> {
        todo!()
    }
}

impl<T> UpdateBookmarkUseCase for T {}
