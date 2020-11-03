use crate::entity::credential_id::CredentialId;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;
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
    fn find_by_user_key(&self, _: &UserKey) -> Result<Option<User>>;
}
