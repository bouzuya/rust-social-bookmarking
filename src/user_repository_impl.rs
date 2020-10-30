use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;
use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::user_repository::UserRepository;

pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create_user(&self, mail_address: MailAddress, password: Password) -> User {
        let user_id = UserId::from_i32(1).unwrap();
        let user = User::new(user_id, mail_address, password);
        println!("create user");
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
        user
    }

    fn find_by_verify_user_secret(&self, verify_user_secret: &VerifyUserSecret) -> Option<User> {
        println!("find user by verify_user_secret");
        println!("verify_user_secret: {}", verify_user_secret.to_string());
        let id = UserId::from_i32(123).unwrap();
        let key = UserKey::from_str("012345").unwrap();
        let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
        let password = Password::from_str("password").unwrap();
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
