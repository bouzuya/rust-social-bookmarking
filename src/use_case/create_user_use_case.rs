use crate::entity::CredentialSecret;
use crate::repository::{CredentialRepository, UserRepository};
use crate::service::SendMailService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct CreateUserUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    user_repository: Arc<dyn UserRepository>,
    send_mail_service: Arc<dyn SendMailService>,
}

impl CreateUserUseCase {
    pub fn new(
        credential_repository: Arc<dyn CredentialRepository>,
        user_repository: Arc<dyn UserRepository>,
        send_mail_service: Arc<dyn SendMailService>,
    ) -> Self {
        Self {
            credential_repository,
            user_repository,
            send_mail_service,
        }
    }

    pub fn create_user(&self, secret: CredentialSecret) -> Result<()> {
        match self.credential_repository.find_by_secret(&secret)? {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let verified = credential.verify(&secret)?;
                    self.credential_repository.save(&verified)?;
                    let user = self
                        .user_repository
                        .find_by_credential_id(&credential.id())?;
                    match user {
                        None => Err(anyhow!("invalid database")),
                        Some(user) => {
                            self.send_mail_service
                                .send_user_verified_mail(&user, &credential);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}
