use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::repository::user_repository::{UseUserRepository, UserRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};
use anyhow::{anyhow, Result};

pub trait UseCreateUserUseCase {
    type CreateUserUseCase: CreateUserUseCase;
    fn create_user_use_case(&self) -> &Self::CreateUserUseCase;
}

pub trait CreateUserUseCase:
    UseCredentialRepository + UseUserRepository + UseSendMailService
{
    fn create_user(&self, verify_user_secret: VerifyUserSecret) -> Result<()> {
        match self
            .credential_repository()
            .find_by_verify_user_secret(&verify_user_secret)?
        {
            None => Err(anyhow!("invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("invalid secret"))
                } else {
                    let verified = credential.verify(&verify_user_secret)?;
                    let user = self.user_repository().create(&credential)?;
                    self.credential_repository().save(verified)?;

                    self.send_mail_service()
                        .send_user_verified_mail(&user, &credential);
                    Ok(())
                }
            }
        }
    }
}

impl<T: UseCredentialRepository + UseUserRepository + UseSendMailService> CreateUserUseCase for T {}
