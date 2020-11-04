use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use anyhow::{anyhow, Result};

pub trait UseVerifyMailAddressUseCase {
    type VerifyMailAddressUseCase: VerifyMailAddressUseCase;
    fn verify_mail_address_use_case(&self) -> &Self::VerifyMailAddressUseCase;
}

pub trait VerifyMailAddressUseCase: UseCredentialRepository {
    fn verify_mail_address(&self, secret: &VerifyUserSecret) -> Result<()> {
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
                    let verified = credential.verify(&secret)?;
                    self.credential_repository().save(&verified)?;
                    // TODO: delete other credentials
                    credential.user_id();
                    Ok(())
                }
            }
        }
    }
}

impl<T: UseCredentialRepository> VerifyMailAddressUseCase for T {}
