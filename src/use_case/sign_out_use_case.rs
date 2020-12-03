use crate::service::SessionService;
use anyhow::Result;
use std::sync::Arc;

pub struct SignOutUseCase {
    session_service: Arc<dyn SessionService>,
}

impl SignOutUseCase {
    pub fn new(session_service: Arc<dyn SessionService>) -> Self {
        Self { session_service }
    }

    pub fn sign_out(&self) -> Result<()> {
        self.session_service.set_current_user(None)
    }
}
