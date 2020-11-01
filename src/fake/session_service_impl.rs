use crate::entity::user::User;
use crate::entity::user_id::UserId;
use crate::service::session_service::SessionService;
use anyhow::Result;
use std::convert::TryFrom;

pub struct SessionServiceImpl;

impl SessionServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

impl SessionService for SessionServiceImpl {
  fn get_current_user(&self) -> Result<Option<User>> {
    let user_id = UserId::try_from(1).unwrap();
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    let user = User::new(user_id, mail_address, password);
    Ok(Some(user))
  }
}
