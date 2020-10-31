use crate::entity::user::User;
use anyhow::Result;

pub trait UseSessionService {
  type SessionService: SessionService;
  fn session_service(&self) -> &Self::SessionService;
}

pub trait SessionService {
  fn get_current_user(&self) -> Result<Option<User>>;
}
