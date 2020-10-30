use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user::User;
use crate::verify_user_secret::VerifyUserSecret;

pub trait UserRepository {
    fn create_user(&self, mail_address: MailAddress, password: Password) -> User;
    fn find_by_verify_user_secret(&self, verify_user_secret: &VerifyUserSecret) -> Option<User>;
    fn save(&self, user: &User) -> bool;
}
