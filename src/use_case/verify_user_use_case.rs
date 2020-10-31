use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::repository::user_repository::{UseUserRepository, UserRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};
use anyhow::{anyhow, Result};

pub trait UseVerifyUserUseCase {
    type VerifyUserUseCase: VerifyUserUseCase;
    fn verify_user_use_case(&self) -> &Self::VerifyUserUseCase;
}

pub trait VerifyUserUseCase: UseUserRepository + UseSendMailService {
    fn verify_user(&self, verify_user_secret: VerifyUserSecret) -> Result<()> {
        match self
            .user_repository()
            .find_by_verify_user_secret(&verify_user_secret)
        {
            None => Err(anyhow!("user not found")),
            Some(user) => {
                let verified = user.verify(&verify_user_secret)?;
                if self.user_repository().save(&verified) {
                    self.send_mail_service().send_user_verified_mail(&verified);
                    Ok(())
                } else {
                    Err(anyhow!("save failed"))
                }
            }
        }
    }
}

impl<T: UseUserRepository + UseSendMailService> VerifyUserUseCase for T {}
