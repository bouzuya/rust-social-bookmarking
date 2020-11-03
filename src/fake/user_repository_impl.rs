use crate::entity::credential_id::CredentialId;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::repository::user_repository::UserRepository;
use anyhow::Result;

pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create(&self, _: &User) -> Result<()> {
        todo!()
    }

    fn create_id(&self) -> Result<UserId> {
        todo!()
    }

    fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>> {
        todo!();
    }
}
