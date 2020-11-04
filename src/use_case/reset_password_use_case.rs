use crate::entity::mail_address::MailAddress;
use crate::repository::{CredentialRepository, UseCredentialRepository};
use crate::service::{SendMailService, UseSendMailService};
use anyhow::Result;

pub trait UseResetPasswordUseCase {
    type ResetPasswordUseCase: ResetPasswordUseCase;
    fn reset_password_use_case(&self) -> &Self::ResetPasswordUseCase;
}

pub trait ResetPasswordUseCase: UseCredentialRepository + UseSendMailService {
    fn reset_password(&self, mail_address: &MailAddress) -> Result<()> {
        match self
            .credential_repository()
            .find_by_mail_address(mail_address)?
        {
            None => Ok(()),
            Some(credential) => match credential.reset_password() {
                Err(_) => Ok(()),
                Ok(updated) => {
                    self.credential_repository().save(&updated)?;

                    self.send_mail_service().send_update_password_mail(&updated);

                    Ok(())
                }
            },
        }
    }
}

impl<T: UseCredentialRepository + UseSendMailService> ResetPasswordUseCase for T {}
