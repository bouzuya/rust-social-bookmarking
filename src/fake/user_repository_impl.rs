use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::entity::verify_user_secret::VerifyUserSecret;
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
    fn create(&self, mail_address: MailAddress, password: Password) -> Result<User> {
        let user_id = UserId::try_from(1).unwrap();
        let user = User::new(user_id, mail_address, password);
        println!("create user");
        println!("  user_id           : {:?}", user.id);
        println!("  key               : {}", user.key.to_string());
        println!("  mail_address      : {}", user.mail_address.to_string());
        println!("  password          : {}", user.password.to_string());
        println!(
            "  verify_user_secret: {}",
            user.verify_user_secret
                .clone()
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        Ok(user)
    }

    fn find_by_verify_user_secret(&self, verify_user_secret: &VerifyUserSecret) -> Option<User> {
        println!("find user by verify_user_secret");
        println!("verify_user_secret: {}", verify_user_secret.to_string());
        let id = UserId::try_from(1).unwrap();
        let key = "123456789012".parse().unwrap();
        let mail_address = "m@bouzuya.net".parse().unwrap();
        let password = "password".parse().unwrap();
        Some(User::of(
            id,
            key,
            mail_address,
            password,
            Some(verify_user_secret.clone()),
        ))
    }

    fn save(&self, user: &User) -> bool {
        println!("save user");
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
        true
    }
}
