use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
use crate::schema::users;
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

#[derive(Insertable)]
#[table_name = "users"]
struct UserRow {
  id: i32,
  key: String,
}

impl From<&User> for UserRow {
  fn from(user: &User) -> Self {
    Self {
      id: user.id().into(),
      key: user.key().into(),
    }
  }
}

impl UserRepository for PgUserRepository {
  fn create(&self, user: &User) -> Result<()> {
    diesel::insert_into(users::table)
      .values(UserRow::from(user))
      .execute(&self.connection)
      .map(|_| ())
      .map_err(anyhow::Error::msg)
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
  fn test_create_user_id() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url));
    let repository = PgUserRepository::new(connection);
    let id1 = repository.create_id().expect("id");
    let id2 = repository.create_id().expect("id");
    assert_ne!(id1, id2);
  }

  #[test]
  fn test_create_user() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url));
    let repository = PgUserRepository::new(connection);
    let id = repository.create_id().expect("id");
    let user = User::new(id);
    repository.create(&user).expect("user");
  }
}
