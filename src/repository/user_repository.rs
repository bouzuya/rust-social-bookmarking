use crate::entity::credential_id::CredentialId;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use anyhow::Result;

pub trait UseUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub trait UserRepository {
    fn create(&self, user: &User) -> Result<()>;
    fn create_id(&self) -> Result<UserId>;
    fn delete(&self, user_id: &UserId) -> Result<()>;
    fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>>;
}
