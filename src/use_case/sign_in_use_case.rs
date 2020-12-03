use crate::entity::{MailAddress, Password};
use crate::repository::{CredentialRepository, UserRepository};
use crate::service::SessionService;
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct SignInUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    user_repository: Arc<dyn UserRepository>,
    session_service: Arc<dyn SessionService>,
}

impl SignInUseCase {
    pub fn new(
        credential_repository: Arc<dyn CredentialRepository>,
        user_repository: Arc<dyn UserRepository>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
        Self {
            credential_repository,
            user_repository,
            session_service,
        }
    }

    pub fn sign_in(&self, mail_address: &MailAddress, password: &Password) -> Result<()> {
        match self
            .credential_repository
            .find_by_mail_address(mail_address)?
        {
            None => Err(anyhow!("unauthorized")),
            Some(credential) => {
                if &credential.password() != password {
                    Err(anyhow!("unauthorized"))
                } else {
                    match self
                        .user_repository
                        .find_by_credential_id(&credential.id())?
                    {
                        None => Err(anyhow!("not found")),
                        Some(user) => self.session_service.set_current_user(Some(user)),
                    }
                }
            }
        }
    }
}
