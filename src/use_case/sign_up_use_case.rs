use crate::entity::{MailAddress, Password};
use crate::repository::{CredentialRepository, UserRepository};
use crate::service::SendMailService;
use anyhow::Result;
use std::sync::Arc;

pub struct SignUpUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    user_repository: Arc<dyn UserRepository>,
    send_mail_service: Arc<dyn SendMailService>,
}

impl SignUpUseCase {
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

    pub fn sign_up(&self, mail_address: MailAddress, password: Password) -> Result<()> {
        if let Some(credential) = self
            .credential_repository
            .find_by_mail_address(&mail_address)?
        {
            match credential.verification() {
                None => return Ok(()),
                Some(_) => self.credential_repository.delete(&credential.id())?,
            }
        }
        let user = self.user_repository.create()?;
        let credential = self
            .credential_repository
            .create(user.id(), &mail_address, &password)?;
        self.send_mail_service.send_create_user_mail(&credential);
        Ok(())
    }
}
