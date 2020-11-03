use crate::entity::user::User;
use crate::service::session_service::{SessionService, UseSessionService};
use anyhow::Result;

pub trait UseGetCurrentUserUseCase {
    type GetCurrentUserUseCase: GetCurrentUserUseCase;
    fn get_current_user_use_case(&self) -> &Self::GetCurrentUserUseCase;
}

pub trait GetCurrentUserUseCase: UseSessionService {
    fn get_current_user(&self) -> Result<Option<User>> {
        let current_user = self.session_service().get_current_user()?;
        Ok(current_user)
    }
}

impl<T: UseSessionService> GetCurrentUserUseCase for T {}
