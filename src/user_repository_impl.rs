use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user::User;
use crate::user_dao::UserDao;
use crate::user_repository::UserRepository;
use crate::verify_user_secret::VerifyUserSecret;

pub struct UserRepositoryImpl {
    user_dao: UserDao,
}

impl UserRepositoryImpl {
    pub fn new(user_dao: UserDao) -> Self {
        UserRepositoryImpl { user_dao }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create_user(&self, mail_address: MailAddress, password: Password) -> User {
        let user = User::new(mail_address, password);
        self.user_dao.insert(&user);
        user
    }

    fn find_by_verify_user_secret(&self, verify_user_secret: &VerifyUserSecret) -> Option<User> {
        self.user_dao
            .find_by_verify_user_secret(&verify_user_secret)
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
