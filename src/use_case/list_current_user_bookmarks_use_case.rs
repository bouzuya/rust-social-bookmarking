use crate::entity::Bookmark;
use crate::repository::BookmarkRepository;
use crate::service::SessionService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct ListCurrentUserBookmarksUseCase {
    bookmark_repository: Arc<dyn BookmarkRepository>,
    session_service: Arc<dyn SessionService>,
}

impl ListCurrentUserBookmarksUseCase {
    pub fn new(
        bookmark_repository: Arc<dyn BookmarkRepository>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
        Self {
            bookmark_repository,
            session_service,
        }
    }

    pub fn list_current_user_bookmarks(&self) -> Result<Vec<Bookmark>> {
        match self.session_service.get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => self.bookmark_repository.find_by_user_id(&current_user.id()),
        }
    }
}
