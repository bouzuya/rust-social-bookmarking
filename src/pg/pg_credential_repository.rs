use crate::entity::{Credential, CredentialId, CredentialSecret, MailAddress, Password, UserId};
use crate::repository::CredentialRepository;
use crate::schema::{
  credential, credential_password_reset, credential_verification, credential_verified,
};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::*;
use std::convert::TryFrom;
use std::sync::Arc;

sql_function!(fn nextval(x: Text) -> BigInt);

pub struct PgCredentialRepository {
  connection: Arc<PgConnection>,
}

impl PgCredentialRepository {
  pub fn new(connection: Arc<PgConnection>) -> Self {
    Self { connection }
  }
}

#[derive(Insertable, Queryable)]
#[table_name = "credential"]
struct CredentialRow {
  id: i32,
  user_id: i32,
  mail_address: String,
  password: String,
}

#[derive(Insertable)]
#[table_name = "credential_password_reset"]
struct CredentialPasswordResetRow {
  credential_id: i32,
  secret: String,
  expired_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "credential_verification"]
struct CredentialVerificationRow {
  credential_id: i32,
  secret: String,
  expired_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "credential_verified"]
struct CredentialVerifiedRow {
  credential_id: i32,
  verified_at: NaiveDateTime,
}

impl From<&Credential> for CredentialRow {
  fn from(credential: &Credential) -> Self {
    Self {
      id: credential.id().into(),
      user_id: credential.user_id().into(),
      mail_address: credential.mail_address().into(),
      password: credential.password().into(),
    }
  }
}

impl CredentialRepository for PgCredentialRepository {
  fn create(
    &self,
    user_id: UserId,
    mail_address: &MailAddress,
    password: &Password,
  ) -> Result<Credential> {
    let id =
      diesel::select(nextval("credential_id")).get_result::<i64>(self.connection.as_ref())?;
    let credential_id = CredentialId::try_from(id as i32).map_err(anyhow::Error::msg)?;
    let credential = Credential::new(credential_id, user_id, mail_address, password);
    diesel::insert_into(credential::table)
      .values(CredentialRow::from(&credential))
      .execute(self.connection.as_ref())
      .map(|_| credential)
      .map_err(anyhow::Error::msg)
  }

  fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Credential>> {
    todo!()
  }

  fn find_by_mail_address(&self, _: &MailAddress) -> Result<Option<Credential>> {
    todo!()
  }

  fn find_by_secret(&self, _: &CredentialSecret) -> Result<Option<Credential>> {
    todo!()
  }

  fn delete(&self, _: &CredentialId) -> Result<()> {
    todo!()
  }

  fn save(&self, _: &Credential) -> Result<()> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::entity::User;
  use crate::pg::pg_user_repository::PgUserRepository;
  use crate::repository::UserRepository;

  #[test]
  fn test_create() {
    let connection = establish_connection();
    connection
      .as_ref()
      .test_transaction::<(), anyhow::Error, _>(|| {
        let user_repository = PgUserRepository::new(connection.clone());
        let user_id = user_repository.create_id()?;
        let user = User::new(&user_id);
        user_repository.create(&user)?;

        let repository = PgCredentialRepository::new(connection.clone());
        let mail_address = "m@bouzuya.net".parse().unwrap();
        let password = "password".parse().unwrap();
        repository.create(user_id, &mail_address, &password)?;
        // TODO: check
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
