use crate::entity::{Credential, User};

pub trait UseSendMailService {
    type SendMailService: SendMailService;
    fn send_mail_service(&self) -> &Self::SendMailService;
}

pub trait SendMailService {
    fn send_create_user_mail(&self, credential: &Credential);
    fn send_update_password_mail(&self, credential: &Credential);
    fn send_user_verified_mail(&self, user: &User, credential: &Credential);
    fn send_verify_mail_address_mail(&self, credential: &Credential);
}
