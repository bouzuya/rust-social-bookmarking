use crate::user::User;

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
            user.verify_user_secret.to_string()
        );
    }
}
