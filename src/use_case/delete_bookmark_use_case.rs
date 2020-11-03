use crate::entity::bookmark_key::BookmarkKey;
use crate::repository::bookmark_repository::{BookmarkRepository, UseBookmarkRepository};
use crate::service::session_service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseDeleteBookmarkUseCase {
    type DeleteBookmarkUseCase: DeleteBookmarkUseCase;
    fn delete_bookmark_use_case(&self) -> &Self::DeleteBookmarkUseCase;
}

pub trait DeleteBookmarkUseCase: UseBookmarkRepository + UseSessionService {
    fn delete_bookmark(&self, bookmark_key: &BookmarkKey) -> Result<()> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("no current user")),
            Some(current_user) => match self.bookmark_repository().find_by_key(&bookmark_key)? {
                None => Err(anyhow!("no bookmark")),
                Some(bookmark) if bookmark.user_id() != current_user.id() => {
                    Err(anyhow!("other user's bookmark"))
                }
                Some(bookmark) => self.bookmark_repository().delete(&bookmark.id()),
            },
        }
    }
}

impl<T: UseBookmarkRepository + UseSessionService> DeleteBookmarkUseCase for T {}
