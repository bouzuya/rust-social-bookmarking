use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_key::BookmarkKey;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::repository::{BookmarkRepository, UseBookmarkRepository};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseUpdateBookmarkUseCase {
    type UpdateBookmarkUseCase: UpdateBookmarkUseCase;
    fn update_bookmark_use_case(&self) -> &Self::UpdateBookmarkUseCase;
}

pub trait UpdateBookmarkUseCase: UseBookmarkRepository + UseSessionService {
    fn update_bookmark(
        &self,
        bookmark_key: BookmarkKey,
        bookmark_url: BookmarkUrl,
        bookmark_title: BookmarkTitle,
        bookmark_comment: BookmarkComment,
    ) -> Result<()> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => match self.bookmark_repository().find_by_key(&bookmark_key)? {
                None => Err(anyhow!("no bookmark")),
                Some(bookmark) if bookmark.user_id() != current_user.id() => {
                    Err(anyhow!("forbidden"))
                }
                Some(bookmark) => {
                    let updated =
                        bookmark.update(bookmark_url, bookmark_title, bookmark_comment)?;
                    self.bookmark_repository().save(&updated)
                }
            },
        }
    }
}

impl<T: UseBookmarkRepository + UseSessionService> UpdateBookmarkUseCase for T {}
