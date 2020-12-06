use crate::repository::{CredentialRepository, UserRepository};
use crate::service::SessionService;
use crate::{entity::UserKey, repository::BookmarkRepository};
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct DeleteUserUseCase {
    session_service: Arc<dyn SessionService>,
    bookmark_repository: Arc<dyn BookmarkRepository>,
    credential_repository: Arc<dyn CredentialRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl DeleteUserUseCase {
    pub fn new(
        session_service: Arc<dyn SessionService>,
        bookmark_repository: Arc<dyn BookmarkRepository>,
        credential_repository: Arc<dyn CredentialRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            session_service,
            bookmark_repository,
            credential_repository,
            user_repository,
        }
    }

    pub fn delete_user(&self, user_key: &UserKey) -> Result<()> {
        match self.session_service.get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) if &current_user.key() != user_key => Err(anyhow!("forbidden")),
            Some(current_user) => {
                self.bookmark_repository
                    .delete_by_user_id(&current_user.id())?;
                self.credential_repository
                    .delete_by_user_id(&current_user.id())?;
                self.user_repository.delete(&current_user.id())?;
                self.session_service.set_current_user(None)
            }
        }
    }
}
