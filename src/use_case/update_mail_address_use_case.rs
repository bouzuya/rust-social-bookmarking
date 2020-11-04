use crate::entity::MailAddress;
use crate::repository::{CredentialRepository, UseCredentialRepository};
use crate::service::{SendMailService, SessionService, UseSendMailService, UseSessionService};
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
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                let credentials = self
                    .credential_repository()
                    .find_by_user_id(&current_user.id())?;
                match credentials
                    .into_iter()
                    .filter(|c| c.verification().is_none())
                    .nth(0)
                {
                    None => unreachable!(),
                    Some(verified) => {
                        let new_credential = self.credential_repository().create(
                            current_user.id(),
                            &mail_address,
                            &verified.password(),
                        )?;
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
