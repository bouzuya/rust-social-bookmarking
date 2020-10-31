use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::user_repository::{UseUserRepository, UserRepository};
use crate::service::send_mail_service::{SendMailService, UseSendMailService};

pub trait UseCreateUserUseCase {
    type CreateUserUseCase: CreateUserUseCase;
    fn create_user_use_case(&self) -> &Self::CreateUserUseCase;
}

pub trait CreateUserUseCase: UseUserRepository + UseSendMailService {
    fn create_user(&self, mail_address: MailAddress, password: Password) {
        let user = self.user_repository().create(mail_address, password);
        self.send_mail_service().send_verify_user_mail(user);
    }
}

impl<T: UseUserRepository + UseSendMailService> CreateUserUseCase for T {}
