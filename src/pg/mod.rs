use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
use anyhow::Result;
use diesel::prelude::*;
use diesel::sql_types::*;
use std::convert::TryFrom;

struct PgUserRepository {
  connection: PgConnection,
}

impl PgUserRepository {
  fn new(connection: PgConnection) -> Self {
    Self { connection }
  }
}

sql_function!(fn nextval(x: Text) -> BigInt);

impl UserRepository for PgUserRepository {
  fn create(&self, _: &User) -> Result<()> {
    todo!()
  }

  fn create_id(&self) -> Result<UserId> {
    let id = diesel::select(nextval("user_id")).get_result::<i64>(&self.connection)?;
    UserId::try_from(id as i32).map_err(anyhow::Error::msg)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_id() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url));
    let repository = PgUserRepository::new(connection);
    let id1 = repository.create_id().expect("id");
    let id2 = repository.create_id().expect("id");
    assert_ne!(id1, id2);
  }
}
