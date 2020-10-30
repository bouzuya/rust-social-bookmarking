use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user_key::UserKey;
use crate::verify_user_secret::VerifyUserSecret;

pub struct User {
    pub key: UserKey,
    pub mail_address: MailAddress,
    pub password: Password,
    pub verify_user_secret: VerifyUserSecret,
}

impl User {
    pub fn new(mail_address: MailAddress, password: Password) -> Self {
        User {
            key: UserKey::generate(),
            mail_address,
            password,
            verify_user_secret: VerifyUserSecret::generate(),
        }
    }
}
