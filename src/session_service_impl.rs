use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::session_service::SessionService;
use crate::user::User;
use crate::user_id::UserId;
use anyhow::Result;

pub struct SessionServiceImpl;

impl SessionServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

impl SessionService for SessionServiceImpl {
  fn get_current_user(&self) -> Result<Option<User>> {
    let user_id = UserId::from_i32(1).unwrap();
    let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
    let password = Password::from_str("password").unwrap();
    let user = User::new(user_id, mail_address, password);
    Ok(Some(user))
  }
}
