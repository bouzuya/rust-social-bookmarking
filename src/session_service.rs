use crate::user::User;
use anyhow::Result;

pub trait SessionService {
  fn get_current_user(&self) -> Result<Option<User>>;
}
