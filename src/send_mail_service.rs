use crate::user::User;

pub trait SendMailService {
    fn send_verify_user_mail(&self, user: User);
}
