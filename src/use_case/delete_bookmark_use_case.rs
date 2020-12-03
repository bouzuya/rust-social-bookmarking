use crate::entity::BookmarkKey;
use crate::repository::BookmarkRepository;
use crate::service::SessionService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct DeleteBookmarkUseCase {
    bookmark_repository: Arc<dyn BookmarkRepository>,
    session_service: Arc<dyn SessionService>,
}

impl DeleteBookmarkUseCase {
    pub fn new(
        bookmark_repository: Arc<dyn BookmarkRepository>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
        Self {
            bookmark_repository,
            session_service,
        }
    }

    pub fn delete_bookmark(&self, bookmark_key: &BookmarkKey) -> Result<()> {
        match self.session_service.get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => match self.bookmark_repository.find_by_key(&bookmark_key)? {
                None => Err(anyhow!("not found")),
                Some(bookmark) if bookmark.user_id() != current_user.id() => {
                    Err(anyhow!("forbidden"))
                }
                Some(bookmark) => self.bookmark_repository.delete(&bookmark.id()),
            },
        }
    }
}
