use crate::entity::user::User;
use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::repository::{CredentialRepository, UseCredentialRepository};
use crate::repository::{UseUserRepository, UserRepository};
use crate::service::{SendMailService, UseSendMailService};
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
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let verified = credential.verify(&verify_user_secret)?;
                    let user = User::new(verified.user_id());
                    self.user_repository().create(&user)?;
                    self.credential_repository().save(&verified)?;

                    self.send_mail_service()
                        .send_user_verified_mail(&user, &credential);
                    Ok(())
                }
            }
        }
    }
}

impl<T: UseCredentialRepository + UseUserRepository + UseSendMailService> CreateUserUseCase for T {}
