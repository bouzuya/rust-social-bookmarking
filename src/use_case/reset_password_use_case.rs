use crate::entity::MailAddress;
use crate::repository::CredentialRepository;
use crate::service::SendMailService;
use anyhow::Result;
use std::sync::Arc;

pub struct ResetPasswordUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    send_mail_service: Arc<dyn SendMailService>,
}

impl ResetPasswordUseCase {
    pub fn new(
        credential_repository: Arc<dyn CredentialRepository>,
        send_mail_service: Arc<dyn SendMailService>,
    ) -> Self {
        Self {
            credential_repository,
            send_mail_service,
        }
    }

    pub fn reset_password(&self, mail_address: &MailAddress) -> Result<()> {
        match self
            .credential_repository
            .find_by_mail_address(mail_address)?
        {
            None => Ok(()),
            Some(credential) => match credential.reset_password() {
                Err(_) => Ok(()),
                Ok(updated) => {
                    self.credential_repository.save(&updated)?;

                    self.send_mail_service.send_update_password_mail(&updated);

                    Ok(())
                }
            },
        }
    }
}
