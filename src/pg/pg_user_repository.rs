use crate::entity::{CredentialId, User, UserId, UserKey};
use crate::repository::UserRepository;
use crate::schema::{credential, user};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    sql_types::*,
};
use std::convert::{TryFrom, TryInto};

sql_function!(fn nextval(x: Text) -> BigInt);

pub struct PgUserRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgUserRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    fn columns() -> (user::columns::id, user::columns::key) {
        (user::columns::id, user::columns::key)
    }

    fn from_row(row: (i32, String)) -> Result<User> {
        let (id, key) = row;
        Ok(User::of(
            id.try_into().map_err(anyhow::Error::msg)?,
            key.parse().map_err(anyhow::Error::msg)?,
        ))
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "user"]
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
    fn create(&self) -> Result<User> {
        let connection = self.pool.get()?;
        let id = diesel::select(nextval("user_id")).get_result::<i64>(&connection)?;
        let user_id = UserId::try_from(id as i32).map_err(anyhow::Error::msg)?;
        let user = User::new(&user_id);
        diesel::insert_into(user::table)
            .values(UserRow::from(&user))
            .execute(&connection)
            .map(|_| user)
            .map_err(anyhow::Error::msg)
    }

    fn delete(&self, user_id: &UserId) -> Result<()> {
        let connection = self.pool.get()?;
        diesel::delete(user::table)
            .filter(user::dsl::id.eq(i32::from(user_id.clone())))
            .execute(&connection)
            .map(|_| ())
            .map_err(anyhow::Error::msg)
    }

    fn find_by_credential_id(&self, credential_id: &CredentialId) -> Result<Option<User>> {
        let connection = self.pool.get()?;
        let found = user::table
            .inner_join(credential::table)
            .select(Self::columns())
            .filter(credential::columns::id.eq(i32::from(credential_id.clone())))
            .first(&connection)
            .optional()?;
        found.map(|row| Self::from_row(row)).transpose()
    }

    fn find_by_user_key(&self, user_key: &UserKey) -> Result<Option<User>> {
        let connection = self.pool.get()?;
        let found = user::table
            .filter(user::dsl::key.eq(String::from(user_key.clone())))
            .first(&connection)
            .optional()
            .map_err(anyhow::Error::msg)?;
        found.map(|row| Self::from_row(row)).transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario() {
        transaction(|pool| {
            let repository = PgUserRepository::new(pool);
            let user = repository.create()?;
            assert_eq!(
                repository.find_by_user_key(&user.key())?,
                Some(user.clone())
            );

            let user_key2 = UserKey::generate();
            assert_eq!(repository.find_by_user_key(&user_key2)?, None);

            // TODO: find_by_credential_id

            repository.delete(&user.id())?;
            assert_eq!(repository.find_by_user_key(&user.key())?, None);

            Ok(())
        });
    }

    fn transaction<F>(f: F)
    where
        F: FnOnce(Pool<ConnectionManager<PgConnection>>) -> Result<()>,
    {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect(&format!("Error connecting to {}", database_url));
        let connection = pool.get().expect("connection");
        connection.test_transaction::<(), anyhow::Error, _>(|| f(pool.clone()))
    }
}
