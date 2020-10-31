use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user::User;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::Result;

pub trait UseUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub trait UserRepository {
    fn create(&self, mail_address: MailAddress, password: Password) -> Result<User>;
    fn find_by_verify_user_secret(&self, verify_user_secret: &VerifyUserSecret) -> Option<User>;
    fn save(&self, user: &User) -> bool;
}
