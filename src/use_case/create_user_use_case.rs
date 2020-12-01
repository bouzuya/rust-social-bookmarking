use crate::entity::CredentialSecret;
use crate::repository::{
    CredentialRepository, UseCredentialRepository, UseUserRepository, UserRepository,
};
use crate::service::{SendMailService, UseSendMailService};
use anyhow::{anyhow, Result};

pub trait UseCreateUserUseCase {
    type CreateUserUseCase: CreateUserUseCase;
    fn create_user_use_case(&self) -> &Self::CreateUserUseCase;
}

pub trait CreateUserUseCase:
    UseCredentialRepository + UseUserRepository + UseSendMailService
{
    fn create_user(&self, secret: CredentialSecret) -> Result<()> {
        match self.credential_repository().find_by_secret(&secret)? {
            None => Err(anyhow!("forbidden: invalid secret")),
            Some(credential) => {
                let verification = credential.verification().unwrap();
                if verification.expired() {
                    Err(anyhow!("forbidden: invalid secret"))
                } else {
                    let verified = credential.verify(&secret)?;
                    self.credential_repository().save(&verified)?;
                    let user = self
                        .user_repository()
                        .find_by_credential_id(&credential.id())?;
                    match user {
                        None => Err(anyhow!("invalid database")),
                        Some(user) => {
                            self.send_mail_service()
                                .send_user_verified_mail(&user, &credential);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}

impl<T: UseCredentialRepository + UseUserRepository + UseSendMailService> CreateUserUseCase for T {}
