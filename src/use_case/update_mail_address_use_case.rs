use crate::entity::mail_address::MailAddress;
use crate::repository::credential_repository::{CredentialRepository, UseCredentialRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};
use crate::service::session_service::{SessionService, UseSessionService};
use anyhow::{anyhow, Result};

pub trait UseUpdateMailAddressUseCase {
    type UpdateMailAddressUseCase: UpdateMailAddressUseCase;
    fn update_mail_address_use_case(&self) -> &Self::UpdateMailAddressUseCase;
}

pub trait UpdateMailAddressUseCase:
    UseSessionService + UseCredentialRepository + UseSendMailService
{
    fn update_mail_address(&self, mail_address: &MailAddress) -> Result<()> {
        match self.session_service().get_current_user()? {
            None => Err(anyhow!("no current user")),
            Some(current_user) => {
                match self
                    .credential_repository()
                    .find_by_id(&current_user.credential_id())?
                {
                    None => Err(anyhow!("no credential")),
                    Some(credential) => {
                        let new_credential = self
                            .credential_repository()
                            .create(&mail_address, &credential.password())?;
                        self.send_mail_service()
                            .send_verify_mail_address_mail(&new_credential);
                        Ok(())
                    }
                }
            }
        }
    }
}

impl<T: UseSessionService + UseCredentialRepository + UseSendMailService> UpdateMailAddressUseCase
    for T
{
}
