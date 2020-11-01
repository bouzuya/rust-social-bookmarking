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
        println!("  to: {}", credential.mail_address().to_string());
        println!(
            "  body: {:?} https://example.com/create-user/?secret={}",
            verification.expired_at(),
            verification.secret().to_string()
        );
    }

    fn send_user_verified_mail(&self, user: &User) {
        println!("send user verified mail");
        println!("to: {}", user.mail_address.to_string());
        println!("body: https://example.com/sign-in");
    }
}
