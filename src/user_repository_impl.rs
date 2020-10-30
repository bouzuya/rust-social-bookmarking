use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::user::User;
use crate::user_dao::UserDao;
use crate::user_repository::UserRepository;

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
}
