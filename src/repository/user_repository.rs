use crate::entity::credential::Credential;
use crate::entity::user::User;
use anyhow::Result;

pub trait UseUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub trait UserRepository {
    fn create(&self, credential: &Credential) -> Result<User>;
}
