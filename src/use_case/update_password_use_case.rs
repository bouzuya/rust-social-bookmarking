use crate::entity::Password;
use crate::repository::{CredentialRepository, UseCredentialRepository};
use crate::service::{SessionService, UseSessionService};
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
                    None => unreachable!(),
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
