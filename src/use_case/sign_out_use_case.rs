use crate::service::{SessionService, UseSessionService};
use anyhow::Result;

pub trait UseSignOutUseCase {
    type SignOutUseCase: SignOutUseCase;
    fn sign_out_use_case(&self) -> &Self::SignOutUseCase;
}

pub trait SignOutUseCase: UseSessionService {
    fn sign_out(&self) -> Result<()> {
        self.session_service().set_current_user(None)
    }
}

impl<T: UseSessionService> SignOutUseCase for T {}
