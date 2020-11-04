use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
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

    fn delete(&self, _: &UserId) -> Result<()> {
        // TODO: delete all bookmarks
        todo!()
    }

    fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>> {
        todo!();
    }

    fn find_by_user_key(&self, _: &UserKey) -> Result<Option<User>> {
        todo!()
    }
}
