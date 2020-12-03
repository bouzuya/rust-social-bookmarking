use crate::entity::MailAddress;
use crate::repository::CredentialRepository;
use crate::service::{SendMailService, SessionService};
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub struct UpdateMailAddressUseCase {
    credential_repository: Arc<dyn CredentialRepository>,
    session_service: Arc<dyn SessionService>,
    send_mail_service: Arc<dyn SendMailService>,
}

impl UpdateMailAddressUseCase {
    pub fn new(
        credential_repository: Arc<dyn CredentialRepository>,
        session_service: Arc<dyn SessionService>,
        send_mail_service: Arc<dyn SendMailService>,
    ) -> Self {
        Self {
            credential_repository,
            session_service,
            send_mail_service,
        }
    }

    pub fn update_mail_address(&self, mail_address: &MailAddress) -> Result<()> {
        match self.session_service.get_current_user()? {
            None => Err(anyhow!("unauthorized")),
            Some(current_user) => {
                let credentials = self
                    .credential_repository
                    .find_by_user_id(&current_user.id())?;
                match credentials
                    .into_iter()
                    .filter(|c| c.verification().is_none())
                    .nth(0)
                {
                    None => unreachable!(),
                    Some(verified) => {
                        let new_credential = self.credential_repository.create(
                            current_user.id(),
                            &mail_address,
                            &verified.password(),
                        )?;
                        self.send_mail_service
                            .send_verify_mail_address_mail(&new_credential);
                        Ok(())
                    }
                }
            }
        }
    }
}
