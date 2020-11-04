use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::repository::{BookmarkRepository, UseBookmarkRepository};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseCreateBookmarkUseCase {
    type CreateBookmarkUseCase: CreateBookmarkUseCase;
    fn create_bookmark_use_case(&self) -> &Self::CreateBookmarkUseCase;
}

pub trait CreateBookmarkUseCase: UseBookmarkRepository + UseSessionService {
    fn create_bookmark(
        &self,
        url: BookmarkUrl,
        title: BookmarkTitle,
        comment: BookmarkComment,
    ) -> Result<()> {
        return match self.session_service().get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                self.bookmark_repository()
                    .create(current_user.id(), url, title, comment)?;
                Ok(())
            }
        };
    }
}

impl<T: UseBookmarkRepository + UseSessionService> CreateBookmarkUseCase for T {}
