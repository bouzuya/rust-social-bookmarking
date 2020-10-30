use crate::entity::user::User;

pub trait SendMailService {
    fn send_verify_user_mail(&self, user: User);
    fn send_user_verified_mail(&self, user: &User);
}
