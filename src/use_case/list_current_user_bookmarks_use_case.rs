use crate::entity::bookmark::Bookmark;
use crate::repository::bookmark_repository::{BookmarkRepository, UseBookmarkRepository};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseListCurrentUserBookmarksUseCase {
    type ListBookmarksUseCase: ListCurrentUserBookmarksUseCase;
    fn list_current_user_bookmarks_use_case(&self) -> &Self::ListBookmarksUseCase;
}

pub trait ListCurrentUserBookmarksUseCase: UseBookmarkRepository + UseSessionService {
    fn list_current_user_bookmarks(&self) -> Result<Vec<Bookmark>> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => self
                .bookmark_repository()
                .find_by_user_id(&current_user.id()),
        }
    }
}

impl<T: UseBookmarkRepository + UseSessionService> ListCurrentUserBookmarksUseCase for T {}
