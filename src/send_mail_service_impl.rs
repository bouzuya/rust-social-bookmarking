use crate::entity::user::User;
use crate::send_mail_service::SendMailService;

pub struct SendMailServiceImpl;

impl SendMailServiceImpl {
    pub fn new() -> Self {
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
            user.verify_user_secret.unwrap().to_string()
        );
    }

    fn send_user_verified_mail(&self, user: &User) {
        println!("send user verified mail");
        println!("to: {}", user.mail_address.to_string());
        println!("body: https://example.com/sign-in");
    }
}
