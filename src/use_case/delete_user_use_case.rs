use crate::entity::user_key::UserKey;
use crate::repository::{UseUserRepository, UserRepository};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseDeleteUserUseCase {
    type DeleteUserUseCase: DeleteUserUseCase;
    fn delete_user_use_case(&self) -> &Self::DeleteUserUseCase;
}

pub trait DeleteUserUseCase: UseSessionService + UseUserRepository {
    fn delete_user(&self, user_key: &UserKey) -> Result<()> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) if &current_user.key() != user_key => Err(anyhow!("forbidden")),
            Some(current_user) => {
                self.user_repository().delete(&current_user.id())?;
                self.session_service().set_current_user(None)
            }
        }
    }
}

impl<T: UseSessionService + UseUserRepository> DeleteUserUseCase for T {}
