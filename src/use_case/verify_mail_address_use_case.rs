use crate::entity::CredentialSecret;
use crate::repository::{CredentialRepository, UseCredentialRepository};
use anyhow::{anyhow, Result};

pub trait UseVerifyMailAddressUseCase {
    type VerifyMailAddressUseCase: VerifyMailAddressUseCase;
    fn verify_mail_address_use_case(&self) -> &Self::VerifyMailAddressUseCase;
}

pub trait VerifyMailAddressUseCase: UseCredentialRepository {
    fn verify_mail_address(&self, secret: &CredentialSecret) -> Result<()> {
        match self.credential_repository().find_by_secret(&secret)? {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let verified = credential.verify(&secret)?;
                    self.credential_repository().save(&verified)?;
                    for c in self
                        .credential_repository()
                        .find_by_user_id(&credential.user_id())?
                    {
                        if c.id() != verified.id() {
                            self.credential_repository().delete(&c.id())?;
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}

impl<T: UseCredentialRepository> VerifyMailAddressUseCase for T {}
