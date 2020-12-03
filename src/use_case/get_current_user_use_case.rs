use crate::entity::User;
use crate::service::SessionService;
use anyhow::Result;
use std::sync::Arc;

pub struct GetCurrentUserUseCase {
    session_service: Arc<dyn SessionService>,
}

impl GetCurrentUserUseCase {
    pub fn new(session_service: Arc<dyn SessionService>) -> Self {
        Self { session_service }
    }

    pub fn get_current_user(&self) -> Result<Option<User>> {
        self.session_service.get_current_user()
    }
}
