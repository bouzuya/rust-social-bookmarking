use crate::entity::{BookmarkComment, BookmarkTitle, BookmarkUrl};
use crate::repository::BookmarkRepository;
use crate::service::SessionService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct CreateBookmarkUseCase {
    bookmark_repository: Arc<dyn BookmarkRepository>,
    session_service: Arc<dyn SessionService>,
}

impl CreateBookmarkUseCase {
    pub fn new(
        bookmark_repository: Arc<dyn BookmarkRepository>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
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
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                self.bookmark_repository
                    .create(current_user.id(), url, title, comment)?;
                Ok(())
            }
        };
    }
}
