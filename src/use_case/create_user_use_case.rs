use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::send_mail_service::SendMailService;
use crate::user_repository::UserRepository;

pub struct CreateUserUseCase<T: SendMailService, U: UserRepository> {
    send_mail_service: T,
    user_repository: U,
}

impl<T: SendMailService, U: UserRepository> CreateUserUseCase<T, U> {
    pub fn new(send_mail_service: T, user_repository: U) -> Self {
        CreateUserUseCase {
            send_mail_service,
            user_repository,
        }
    }

    pub fn create_user(&self, mail_address: MailAddress, password: Password) {
        let user = self.user_repository.create_user(mail_address, password);
        self.send_mail_service.send_verify_user_mail(user);
    }
}
