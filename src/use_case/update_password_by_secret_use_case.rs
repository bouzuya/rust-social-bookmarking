use crate::entity::password::Password;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::Result;

pub trait UseUpdatePasswordBySecretUseCase {
    type UpdatePasswordBySecretUseCase: UpdatePasswordBySecretUseCase;
    fn update_password_by_secret_use_case(&self) -> &Self::UpdatePasswordBySecretUseCase;
}

pub trait UpdatePasswordBySecretUseCase {
    fn update_password_by_secret(&self, _: Password, _: VerifyUserSecret) -> Result<()> {
        todo!()
    }
}

impl<T> UpdatePasswordBySecretUseCase for T {}
