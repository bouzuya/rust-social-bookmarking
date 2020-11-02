use crate::entity::mail_address::MailAddress;
use anyhow::Result;

pub trait UseUpdateMailAddressUseCase {
    type UpdateMailAddressUseCase: UpdateMailAddressUseCase;
    fn update_mail_address_use_case(&self) -> &Self::UpdateMailAddressUseCase;
}

pub trait UpdateMailAddressUseCase {
    fn update_mail_address(&self, _: MailAddress) -> Result<()> {
        todo!()
    }
}

impl<T> UpdateMailAddressUseCase for T {}
