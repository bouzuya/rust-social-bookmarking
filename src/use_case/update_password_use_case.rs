use crate::entity::password::Password;
use anyhow::Result;

pub trait UseUpdatePasswordUseCase {
    type UpdatePasswordUseCase: UpdatePasswordUseCase;
    fn update_password_use_case(&self) -> &Self::UpdatePasswordUseCase;
}

pub trait UpdatePasswordUseCase {
    fn update_password(&self, _: Password) -> Result<()> {
        todo!()
    }
}

impl<T> UpdatePasswordUseCase for T {}
