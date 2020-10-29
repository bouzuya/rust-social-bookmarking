struct MailAddress(String);

impl MailAddress {
    fn from_str(s: &str) -> Option<Self> {
        // TODO: validation
        Some(MailAddress(s.into()))
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

struct Password(String);

impl Password {
    fn from_str(s: &str) -> Option<Self> {
        // TODO: validation
        Some(Password(s.into()))
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

struct VerifyUserSecret(String);

impl VerifyUserSecret {
    fn generate() -> Self {
        // TODO: generate secret
        VerifyUserSecret("verify-user-secret1".into())
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

struct UserKey(String);

impl UserKey {
    fn generate() -> Self {
        // TODO: generate key
        Self("user-key1".into())
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

struct User {
    key: UserKey,
    mail_address: MailAddress,
    password: Password,
    verify_user_secret: VerifyUserSecret,
}

impl User {
    fn new(mail_address: MailAddress, password: Password) -> Self {
        User {
            key: UserKey::generate(),
            mail_address,
            password,
            verify_user_secret: VerifyUserSecret::generate(),
        }
    }
}

trait UserRepository {
    fn create_user(&self, mail_address: MailAddress, password: Password) -> User;
}

struct UserDao;

impl UserDao {
    fn new() -> Self {
        UserDao
    }

    fn insert(&self, user: &User) {
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

struct UserRepositoryImpl {
    user_dao: UserDao,
}

impl UserRepositoryImpl {
    fn new(user_dao: UserDao) -> Self {
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

trait SendMailService {
    fn send_verify_user_mail(&self, user: User);
}

struct SendMailServiceImpl;

impl SendMailServiceImpl {
    fn new() -> Self {
        SendMailServiceImpl
    }
}

impl SendMailService for SendMailServiceImpl {
    fn send_verify_user_mail(&self, user: User) {
        println!("send verify user mail");
        println!("to: {}", user.mail_address.to_string());
        println!(
            "body: https://example.com/verify-user/{}?secret={}",
            user.key.to_string(),
            user.verify_user_secret.to_string()
        );
    }
}

struct CreateUserUseCase<T: SendMailService, U: UserRepository> {
    send_mail_service: T,
    user_repository: U,
}

impl<T: SendMailService, U: UserRepository> CreateUserUseCase<T, U> {
    fn new(send_mail_service: T, user_repository: U) -> Self {
        CreateUserUseCase {
            send_mail_service,
            user_repository,
        }
    }

    fn create_user(&self, mail_address: MailAddress, password: Password) {
        let user = self.user_repository.create_user(mail_address, password);
        self.send_mail_service.send_verify_user_mail(user);
    }
}

fn main() {
    let send_mail_service = SendMailServiceImpl::new();
    let user_dao = UserDao::new();
    let user_repository = UserRepositoryImpl::new(user_dao);
    let create_user_use_case = CreateUserUseCase::new(send_mail_service, user_repository);
    let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
    let password = Password::from_str("password").unwrap();
    create_user_use_case.create_user(mail_address, password);
}
