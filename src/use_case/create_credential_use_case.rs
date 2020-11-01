use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};
use anyhow::Result;

pub trait UseCreateCredentialUseCase {
    type CreateCredentialUseCase: CreateCredentialUseCase;
    fn create_credential_use_case(&self) -> &Self::CreateCredentialUseCase;
}

pub trait CreateCredentialUseCase: UseCredentialRepository + UseSendMailService {
    fn create_credential(&self, mail_address: MailAddress, password: Password) -> Result<()> {
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
            .create(mail_address, password)?;
        self.send_mail_service().send_create_user_mail(&credential);
        Ok(())
    }
}

impl<T: UseCredentialRepository + UseSendMailService> CreateCredentialUseCase for T {}
