use crate::entity::{CredentialSecret, Password};
use crate::repository::CredentialRepository;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct UpdatePasswordBySecretUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
}

impl UpdatePasswordBySecretUseCase {
    pub fn new(credential_repository: Arc<dyn CredentialRepository>) -> Self {
        Self {
            credential_repository,
        }
    }

    pub fn update_password_by_secret(
        &self,
        secret: &CredentialSecret,
        password: &Password,
    ) -> Result<()> {
        match self
            .credential_repository
            .find_by_password_reset_secret(&secret)?
        {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let password_reset = credential.password_reset().unwrap();
                if password_reset.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let updated = credential.update_password(password)?;
                    self.credential_repository.save(&updated)
                }
            }
        }
    }
}
