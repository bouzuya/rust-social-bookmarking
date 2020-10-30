use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user::User;
use crate::user_key::UserKey;
use crate::verify_user_secret::VerifyUserSecret;

pub struct UserDao;

impl UserDao {
    pub fn new() -> Self {
        UserDao
    }

    pub fn insert(&self, user: &User) {
        println!("insert user");
        println!("key               : {}", user.key.to_string());
        println!("mail_address      : {}", user.mail_address.to_string());
        println!("password          : {}", user.password.to_string());
        println!(
            "verify_user_secret: {}",
            user.verify_user_secret
                .clone()
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
    }

    pub fn find_by_verify_user_secret(
        &self,
        verify_user_secret: &VerifyUserSecret,
    ) -> Option<User> {
        println!("find user by verify_user_secret");
        println!("verify_user_secret: {}", verify_user_secret.to_string());
        let key = UserKey::from_str("012345").unwrap();
        let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
        let password = Password::from_str("password").unwrap();
        Some(User::of(
            key,
            mail_address,
            password,
            Some(verify_user_secret.clone()),
        ))
    }
}
