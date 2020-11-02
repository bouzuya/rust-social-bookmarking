use crate::entity::user_key::UserKey;
use anyhow::Result;

pub trait UseDeleteUserUseCase {
    type DeleteUserUseCase: DeleteUserUseCase;
    fn delete_user_use_case(&self) -> &Self::DeleteUserUseCase;
}

pub trait DeleteUserUseCase {
    fn delete_user(&self, _: UserKey) -> Result<()> {
        todo!()
    }
}

impl<T> DeleteUserUseCase for T {}
