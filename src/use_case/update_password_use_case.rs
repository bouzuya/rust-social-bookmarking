use crate::entity::password::Password;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::service::session_service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseUpdatePasswordUseCase {
    type UpdatePasswordUseCase: UpdatePasswordUseCase;
    fn update_password_use_case(&self) -> &Self::UpdatePasswordUseCase;
}

pub trait UpdatePasswordUseCase: UseCredentialRepository + UseSessionService {
    fn update_password(&self, password: &Password) -> Result<()> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                let credentials = self
                    .credential_repository()
                    .find_by_user_id(&current_user.id())?;
                match credentials
                    .into_iter()
                    .filter(|c| c.verification().is_none())
                    .nth(0)
                {
                    None => Err(anyhow!("no verified credential")),
                    Some(credential) => {
                        let updated = credential.update_password(&password)?;
                        self.credential_repository().save(&updated)
                    }
                }
            }
        }
    }
}

impl<T: UseCredentialRepository + UseSessionService> UpdatePasswordUseCase for T {}
