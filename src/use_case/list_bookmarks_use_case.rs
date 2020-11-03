use crate::entity::bookmark::Bookmark;
use crate::repository::bookmark_repository::{BookmarkRepository, UseBookmarkRepository};
use crate::service::session_service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseListBookmarksUseCase {
    type ListBookmarksUseCase: ListBookmarksUseCase;
    fn list_bookmarks_use_case(&self) -> &Self::ListBookmarksUseCase;
}

pub trait ListBookmarksUseCase: UseBookmarkRepository + UseSessionService {
    // TODO: list_bookmarks -> list_current_user_bookmarks
    fn list_bookmarks(&self) -> Result<Vec<Bookmark>> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("no current user")),
            Some(current_user) => self
                .bookmark_repository()
                .find_by_user_key(&current_user.key()),
        }
    }
}

impl<T: UseBookmarkRepository + UseSessionService> ListBookmarksUseCase for T {}
