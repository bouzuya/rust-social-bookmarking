use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::repository::bookmark_repository::BookmarkRepository;
use crate::session_service::SessionService;
use anyhow::{anyhow, Result};

pub struct CreateBookmarkUseCase<B: BookmarkRepository, S: SessionService> {
    bookmark_repository: B,
    session_service: S,
}

impl<B: BookmarkRepository, S: SessionService> CreateBookmarkUseCase<B, S> {
    pub fn new(bookmark_repository: B, session_service: S) -> Self {
        Self {
            bookmark_repository,
            session_service,
        }
    }

    pub fn create_bookmark(
        &self,
        url: BookmarkUrl,
        title: BookmarkTitle,
        comment: BookmarkComment,
    ) -> Result<()> {
        return match self.session_service.get_current_user()? {
            None => Err(anyhow!("no current user")),
            Some(current_user) => {
                self.bookmark_repository
                    .create(current_user.id, url, title, comment)?;
                Ok(())
            }
        };
    }
}
