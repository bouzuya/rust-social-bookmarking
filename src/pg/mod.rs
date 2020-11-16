use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
use anyhow::Result;

struct PgUserRepository;

impl UserRepository for PgUserRepository {
  fn create(&self, _: &User) -> Result<()> {
    todo!()
  }

  fn create_id(&self) -> Result<UserId> {
    todo!()
  }

  fn delete(&self, _: &UserId) -> Result<()> {
    todo!()
  }

  fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>> {
    todo!()
  }

  fn find_by_user_key(&self, _: &UserKey) -> Result<Option<User>> {
    todo!()
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn name() {
    unimplemented!();
  }
}
