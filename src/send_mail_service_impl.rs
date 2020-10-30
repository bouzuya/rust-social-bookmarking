use crate::send_mail_service::SendMailService;
use crate::user::User;

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
            user.verify_user_secret.to_string()
        );
    }
}
