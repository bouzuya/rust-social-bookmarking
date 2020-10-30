use crate::send_mail_service::SendMailService;
use crate::user_repository::UserRepository;
use crate::verify_user_secret::VerifyUserSecret;
use anyhow::{anyhow, Result};

pub struct VerifyUserUseCase<T: SendMailService, U: UserRepository> {
    send_mail_service: T,
    user_repository: U,
}

impl<T: SendMailService, U: UserRepository> VerifyUserUseCase<T, U> {
    pub fn new(send_mail_service: T, user_repository: U) -> Self {
        Self {
            send_mail_service,
            user_repository,
        }
    }

    pub fn verify_user(&self, verify_user_secret: VerifyUserSecret) -> Result<()> {
        match self
            .user_repository
            .find_by_verify_user_secret(&verify_user_secret)
        {
            None => Err(anyhow!("user not found")),
            Some(user) => {
                let verified = user.verify(&verify_user_secret)?;
                if self.user_repository.save(&verified) {
                    self.send_mail_service.send_user_verified_mail(&verified);
                    Ok(())
                } else {
                    Err(anyhow!("save failed"))
                }
            }
        }
    }
}
