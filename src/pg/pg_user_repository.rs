use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
use crate::schema::users;
use anyhow::Result;
use diesel::prelude::*;
use diesel::sql_types::*;
use std::convert::TryFrom;
use std::sync::Arc;

sql_function!(fn nextval(x: Text) -> BigInt);

pub struct PgUserRepository {
  connection: Arc<PgConnection>,
}

impl PgUserRepository {
  pub fn new(connection: Arc<PgConnection>) -> Self {
    Self { connection }
  }
}

#[derive(Insertable, Queryable)]
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
      .execute(self.connection.as_ref())
      .map(|_| ())
      .map_err(anyhow::Error::msg)
  }

  fn create_id(&self) -> Result<UserId> {
    let id = diesel::select(nextval("user_id")).get_result::<i64>(self.connection.as_ref())?;
    UserId::try_from(id as i32).map_err(anyhow::Error::msg)
  }

  fn delete(&self, user_id: &UserId) -> Result<()> {
    diesel::delete(users::table)
      .filter(users::dsl::id.eq(i32::from(user_id.clone())))
      .execute(self.connection.as_ref())
      .map(|_| ())
      .map_err(anyhow::Error::msg)
  }

  fn find_by_credential_id(&self, _: &CredentialId) -> Result<Option<User>> {
    todo!()
  }

  fn find_by_user_key(&self, user_key: &UserKey) -> Result<Option<User>> {
    users::dsl::users
      .filter(users::dsl::key.eq(String::from(user_key.clone())))
      .first(self.connection.as_ref())
      .optional()
      .map(|result: Option<UserRow>| {
        result.map(|row| {
          User::of(
            UserId::try_from(row.id).unwrap(),
            UserKey::try_from(row.key.as_ref()).unwrap(),
          )
        })
      })
      .map_err(anyhow::Error::msg)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_id() {
    let connection = establish_connection();
    connection
      .as_ref()
      .test_transaction::<(), anyhow::Error, _>(|| {
        let repository = PgUserRepository::new(connection.clone());
        let id1 = repository.create_id()?;
        let id2 = repository.create_id()?;
        assert_ne!(id1, id2);
        Ok(())
      });
  }

  #[test]
  fn test_create() {
    let connection = establish_connection();
    connection
      .as_ref()
      .test_transaction::<(), anyhow::Error, _>(|| {
        let repository = PgUserRepository::new(connection.clone());
        let id = repository.create_id()?;
        let user = User::new(id);
        repository.create(&user)?;
        assert_eq!(repository.find_by_user_key(&user.key())?, Some(user));
        Ok(())
      });
  }

  #[test]
  fn test_delete() {
    let connection = establish_connection();
    connection
      .as_ref()
      .test_transaction::<(), anyhow::Error, _>(|| {
        let repository = PgUserRepository::new(connection.clone());
        let id = repository.create_id()?;
        let user = User::new(id);
        repository.create(&user)?;
        let user_key1 = user.key();
        assert_eq!(repository.find_by_user_key(&user_key1)?, Some(&user));

        repository.delete(&user.id())?;
        assert_eq!(repository.find_by_user_key(&user_key1)?, None);

        Ok(())
      });
  }

  #[test]
  fn test_find_by_user_key() {
    let connection = establish_connection();
    connection
      .as_ref()
      .test_transaction::<(), anyhow::Error, _>(|| {
        let repository = PgUserRepository::new(connection.clone());
        let id = repository.create_id()?;
        let user = User::new(id);
        repository.create(&user)?;
        let user_key1 = user.key();
        assert_eq!(repository.find_by_user_key(&user_key1)?, Some(user));

        let user_key2 = UserKey::generate();
        assert_eq!(repository.find_by_user_key(&user_key2)?, None);
        Ok(())
      });
  }

  fn establish_connection() -> Arc<PgConnection> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url));
    Arc::new(connection)
  }
}
