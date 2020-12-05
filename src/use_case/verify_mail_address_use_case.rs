use crate::entity::CredentialSecret;
use crate::repository::CredentialRepository;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct VerifyMailAddressUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
}

impl VerifyMailAddressUseCase {
    pub fn new(credential_repository: Arc<dyn CredentialRepository>) -> Self {
        Self {
            credential_repository,
        }
    }

    pub fn verify_mail_address(&self, secret: &CredentialSecret) -> Result<()> {
        match self
            .credential_repository
            .find_by_verification_secret(&secret)?
        {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let verified = credential.verify(&secret)?;
                    self.credential_repository.save(&verified)?;
                    for c in self
                        .credential_repository
                        .find_by_user_id(&credential.user_id())?
                    {
                        if c.id() != verified.id() {
                            self.credential_repository.delete(&c.id())?;
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}
