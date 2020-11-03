use crate::entity::user::User;
use crate::service::session_service::SessionService;
use anyhow::Result;

pub struct SessionServiceImpl;

impl SessionServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

impl SessionService for SessionServiceImpl {
  fn get_current_user(&self) -> Result<Option<User>> {
    todo!()
  }

  fn set_current_user(&self, _: Option<User>) -> Result<()> {
    todo!()
  }
}
