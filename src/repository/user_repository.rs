use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::user::User;
use anyhow::Result;

pub trait UseUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub trait UserRepository {
    fn create(&self, credential: &Credential) -> Result<User>;
    fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>>;
}
