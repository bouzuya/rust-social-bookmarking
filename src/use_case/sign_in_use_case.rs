use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use anyhow::Result;

pub trait UseSignInUseCase {
    type SignInUseCase: SignInUseCase;
    fn sign_in_use_case(&self) -> &Self::SignInUseCase;
}

pub trait SignInUseCase {
    fn sign_in(&self, _: MailAddress, _: Password) -> Result<()> {
        todo!()
    }
}

impl<T> SignInUseCase for T {}
