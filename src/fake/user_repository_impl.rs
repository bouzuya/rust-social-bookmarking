use crate::entity::credential::Credential;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::repository::user_repository::UserRepository;
use anyhow::Result;
use std::convert::TryFrom;

pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create(&self, credential: &Credential) -> Result<User> {
        let user_id = UserId::try_from(1).unwrap();
        let user = User::new(user_id, &credential);
        println!("create user");
        println!("  user_id           : {:?}", user.id());
        println!("  key               : {}", user.key().to_string());
        println!("  credential_id     : {:?}", user.credential_id());
        Ok(user)
    }
}
