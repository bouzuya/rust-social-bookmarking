use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};
use anyhow::Result;

pub trait UseSignUpUseCase {
    type SignUpUseCase: SignUpUseCase;
    fn sign_up_use_case(&self) -> &Self::SignUpUseCase;
}

pub trait SignUpUseCase: UseCredentialRepository + UseSendMailService {
    fn sign_up(&self, mail_address: MailAddress, password: Password) -> Result<()> {
        if let Some(credential) = self
            .credential_repository()
            .find_by_mail_address(&mail_address)?
        {
            match credential.verification() {
                None => return Ok(()),
                Some(_) => self.credential_repository().delete(&credential.id())?,
            }
        }
        let credential = self
            .credential_repository()
            .create(&mail_address, &password)?;
        self.send_mail_service().send_create_user_mail(&credential);
        Ok(())
    }
}

impl<T: UseCredentialRepository + UseSendMailService> SignUpUseCase for T {}
