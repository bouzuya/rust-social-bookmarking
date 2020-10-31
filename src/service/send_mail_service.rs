use crate::entity::user::User;

pub trait UseSendMailService {
    type SendMailService: SendMailService;
    fn send_mail_service(&self) -> &Self::SendMailService;
}

pub trait SendMailService {
    fn send_verify_user_mail(&self, user: User);
    fn send_user_verified_mail(&self, user: &User);
}
