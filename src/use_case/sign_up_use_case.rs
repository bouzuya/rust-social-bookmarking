use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::repository::user_repository::{UseUserRepository, UserRepository};
use crate::service::{SendMailService, UseSendMailService};
use anyhow::Result;

pub trait UseSignUpUseCase {
    type SignUpUseCase: SignUpUseCase;
    fn sign_up_use_case(&self) -> &Self::SignUpUseCase;
}

pub trait SignUpUseCase: UseCredentialRepository + UseUserRepository + UseSendMailService {
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
        let user_id = self.user_repository().create_id()?;
        let credential = self
            .credential_repository()
            .create(user_id, &mail_address, &password)?;
        self.send_mail_service().send_create_user_mail(&credential);
        Ok(())
    }
}

impl<T: UseCredentialRepository + UseUserRepository + UseSendMailService> SignUpUseCase for T {}
