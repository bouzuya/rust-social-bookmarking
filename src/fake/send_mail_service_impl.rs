use crate::entity::credential::Credential;
use crate::entity::user::User;
use crate::service::send_mail_service::SendMailService;

pub struct SendMailServiceImpl;

impl SendMailServiceImpl {
    pub fn new() -> Self {
        SendMailServiceImpl
    }
}

impl SendMailService for SendMailServiceImpl {
    fn send_create_user_mail(&self, credential: &Credential) {
        let verification = credential.verification().unwrap();
        println!("send create user mail");
        println!("  to: {:?}", credential.mail_address());
        println!(
            "  body: {:?} https://example.com/create-user/?secret={:?}",
            verification.expired_at(),
            verification.secret()
        );
    }

    fn send_update_password_mail(&self, _: &Credential) {
        todo!()
    }

    fn send_user_verified_mail(&self, user: &User, credential: &Credential) {
        println!("send user verified mail");
        println!("to: {:?}", credential.mail_address());
        println!(
            "body: https://example.com/sign-in?mail_address={:?}",
            credential.mail_address()
        );
        println!("      https://example.com/users/{:?}", user.key());
    }

    fn send_verify_mail_address_mail(&self, _: &Credential) {
        todo!()
    }
}
