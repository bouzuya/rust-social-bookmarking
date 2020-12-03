use crate::entity::Password;
use crate::repository::CredentialRepository;
use crate::service::SessionService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct UpdatePasswordUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    session_service: Arc<dyn SessionService>,
}

impl UpdatePasswordUseCase {
    pub fn new(
        credential_repository: Arc<dyn CredentialRepository>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
        Self {
            credential_repository,
            session_service,
        }
    }

    pub fn update_password(&self, password: &Password) -> Result<()> {
        match self.session_service.get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                let credentials = self
                    .credential_repository
                    .find_by_user_id(&current_user.id())?;
                match credentials
                    .into_iter()
                    .filter(|c| c.verification().is_none())
                    .nth(0)
                {
                    None => unreachable!(),
                    Some(credential) => {
                        let updated = credential.update_password(&password)?;
                        self.credential_repository.save(&updated)
                    }
                }
            }
        }
    }
}
