use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user::User;

pub trait UserRepository {
    fn create_user(&self, mail_address: MailAddress, password: Password) -> User;
}
