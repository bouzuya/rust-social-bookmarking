use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::repository::user_repository::{UseUserRepository, UserRepository};
use crate::service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseSignInUseCase {
    type SignInUseCase: SignInUseCase;
    fn sign_in_use_case(&self) -> &Self::SignInUseCase;
}

pub trait SignInUseCase: UseCredentialRepository + UseUserRepository + UseSessionService {
    fn sign_in(&self, mail_address: &MailAddress, password: &Password) -> Result<()> {
        match self
            .credential_repository()
            .find_by_mail_address(mail_address)?
        {
            None => Err(anyhow!("unauthorized")),
            Some(credential) => {
                if &credential.password() != password {
                    Err(anyhow!("unauthorized"))
                } else {
                    match self
                        .user_repository()
                        .find_by_credential_id(&credential.id())?
                    {
                        None => Err(anyhow!("not found")),
                        Some(user) => self.session_service().set_current_user(Some(user)),
                    }
                }
            }
        }
    }
}

impl<T: UseCredentialRepository + UseUserRepository + UseSessionService> SignInUseCase for T {}
