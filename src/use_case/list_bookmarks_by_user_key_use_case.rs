use crate::entity::{Bookmark, UserKey};
use crate::repository::{
    BookmarkRepository, UseBookmarkRepository, UseUserRepository, UserRepository,
};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseListBookmarksByUserKeyUseCase {
    type ListBookmarksByUserKeyUseCase: ListBookmarksByUserKeyUseCase;
    fn list_bookmarks_by_user_key_use_case(&self) -> &Self::ListBookmarksByUserKeyUseCase;
}

pub trait ListBookmarksByUserKeyUseCase:
    UseBookmarkRepository + UseUserRepository + UseSessionService
{
    fn list_bookmarks_by_user_key(&self, user_key: &UserKey) -> Result<Vec<Bookmark>> {
        let (user_id, all) = match self.user_repository().find_by_user_key(user_key)? {
            None => return Err(anyhow!("not found")),
            Some(user) => match self.session_service().get_current_user()? {
                None => (user.id(), false),
                Some(current_user) if &current_user.key() != user_key => (user.id(), false),
                Some(current_user) => (current_user.id(), true),
            },
        };
        Ok(self
            .bookmark_repository()
            .find_by_user_id(&user_id)?
            .into_iter()
            .filter(|b| all || b.is_public())
            .collect::<Vec<Bookmark>>())
    }
}

impl<T: UseBookmarkRepository + UseUserRepository + UseSessionService> ListBookmarksByUserKeyUseCase
    for T
{
}
