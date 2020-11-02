use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::Result;

pub trait UseVerifyMailAddressUseCase {
    type VerifyMailAddressUseCase: VerifyMailAddressUseCase;
    fn verify_mail_address_use_case(&self) -> &Self::VerifyMailAddressUseCase;
}

pub trait VerifyMailAddressUseCase {
    fn verify_mail_address(&self, _: VerifyUserSecret) -> Result<()> {
        todo!()
    }
}

impl<T> VerifyMailAddressUseCase for T {}
