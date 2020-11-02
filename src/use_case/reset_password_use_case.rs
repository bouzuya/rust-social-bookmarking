use crate::entity::mail_address::MailAddress;
use anyhow::Result;

pub trait UseResetPasswordUseCase {
    type ResetPasswordUseCase: ResetPasswordUseCase;
    fn reset_password_use_case(&self) -> &Self::ResetPasswordUseCase;
}

pub trait ResetPasswordUseCase {
    fn reset_password(&self, _: MailAddress) -> Result<()> {
        todo!()
    }
}

impl<T> ResetPasswordUseCase for T {}
