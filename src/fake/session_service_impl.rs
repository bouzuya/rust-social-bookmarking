use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
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
    let credential_id = CredentialId::try_from(1).unwrap();
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    let credential = Credential::new(credential_id, mail_address, password);
    let user = User::new(user_id, &credential);
    Ok(Some(user))
  }

  fn set_current_user(&self, _: User) -> Result<()> {
    todo!()
  }
}
