use crate::entity::{Password, VerifyUserSecret};
use crate::repository::{CredentialRepository, UseCredentialRepository};
use anyhow::{anyhow, Result};

pub trait UseUpdatePasswordBySecretUseCase {
    type UpdatePasswordBySecretUseCase: UpdatePasswordBySecretUseCase;
    fn update_password_by_secret_use_case(&self) -> &Self::UpdatePasswordBySecretUseCase;
}

pub trait UpdatePasswordBySecretUseCase: UseCredentialRepository {
    fn update_password_by_secret(
        &self,
        secret: &VerifyUserSecret,
        password: &Password,
    ) -> Result<()> {
        match self
            .credential_repository()
            .find_by_verify_user_secret(&secret)?
        {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let updated = credential.update_password(password)?;
                    self.credential_repository().save(&updated)
                }
            }
        }
    }
}

impl<T: UseCredentialRepository> UpdatePasswordBySecretUseCase for T {}
