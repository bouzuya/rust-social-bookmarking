use crate::entity::credential::Credential;
use crate::entity::user::User;

pub trait UseSendMailService {
    type SendMailService: SendMailService;
    fn send_mail_service(&self) -> &Self::SendMailService;
}

pub trait SendMailService {
    fn send_create_user_mail(&self, credential: &Credential);
    fn send_user_verified_mail(&self, user: &User);
}
