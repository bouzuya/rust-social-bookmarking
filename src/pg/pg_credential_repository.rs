use crate::entity::{
    Credential, CredentialId, CredentialSecret, CredentialSecretExpiredAt,
    CredentialSecretWithExpiration, CredentialStatus, MailAddress, Password, UserId,
};
use crate::repository::CredentialRepository;
use crate::schema::{
    credential, credential_password_reset, credential_verification, credential_verified,
};
use anyhow::{anyhow, Result};
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

impl TryFrom<&Credential> for CredentialVerificationRow {
    type Error = &'static str;
    fn try_from(
        credential: &Credential,
    ) -> std::result::Result<Self, <Self as TryFrom<&Credential>>::Error> {
        match credential.verification() {
            None => Err("invalid status"),
            Some(v) => Ok(Self {
                credential_id: credential.id().into(),
                secret: v.secret().into(),
                expired_at: v.expired_at().into(),
            }),
        }
    }
}

#[derive(Insertable)]
#[table_name = "credential_verified"]
struct CredentialVerifiedRow {
    credential_id: i32,
    verified_at: NaiveDateTime,
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
            .map_err(anyhow::Error::msg)?;
        if let Some(verification) = CredentialVerificationRow::try_from(&credential).ok() {
            diesel::insert_into(credential_verification::table)
                .values(verification)
                .execute(self.connection.as_ref())
                .map_err(anyhow::Error::msg)?;
        }
        Ok(credential)
    }

    fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Credential>> {
        todo!()
    }

    fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>> {
        let found: Option<(
            i32,
            i32,
            String,
            String,
            Option<String>,
            Option<NaiveDateTime>,
        )> = credential::table
            .left_outer_join(credential_password_reset::table)
            .left_outer_join(credential_verification::table)
            .left_outer_join(credential_verified::table)
            .select((
                credential::id,
                credential::user_id,
                credential::mail_address,
                credential::password,
                credential_verification::secret.nullable(),
                credential_verification::expired_at.nullable(),
            ))
            .filter(credential::mail_address.eq(mail_address.to_string()))
            .first(self.connection.as_ref())
            .optional()
            .map_err(anyhow::Error::msg)?;
        match found {
            None => Ok(None),
            Some((id, user_id, mail_address, password, secret, expired_at)) => {
                let v = match (secret, expired_at) {
                    (None, Some(_)) | (Some(_), None) => Err(anyhow!("invalid database")),
                    (None, None) => Ok(None),
                    (Some(s), Some(e)) => {
                        let secret = s.parse().map_err(anyhow::Error::msg)?;
                        let expired_at = CredentialSecretExpiredAt::from(e);
                        let secret_with_expiration =
                            CredentialSecretWithExpiration::of(expired_at, secret);
                        Ok(Some(secret_with_expiration))
                    }
                }?;
                Ok(Some(Credential::of(
                    CredentialId::try_from(id).map_err(anyhow::Error::msg)?,
                    UserId::try_from(user_id).map_err(anyhow::Error::msg)?,
                    mail_address.parse().map_err(anyhow::Error::msg)?,
                    password.parse().map_err(anyhow::Error::msg)?,
                    None,
                    // TODO: other status
                    match v {
                        Some(secret_with_expiration) => {
                            CredentialStatus::WaitingForVerification(secret_with_expiration)
                        }
                        None => CredentialStatus::Verified,
                    },
                )))
            }
        }
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
                let user = create_user(&connection)?;
                let repository = PgCredentialRepository::new(connection.clone());
                let mail_address = "m@bouzuya.net".parse().unwrap();
                let password = "password".parse().unwrap();
                let created = repository.create(user.id(), &mail_address, &password)?;
                let found = repository.find_by_mail_address(&mail_address)?;
                assert_eq!(found, Some(created));
                Ok(())
            });
    }

    #[test]
    fn test_find_by_mail_address() {
        // TODO
    }

    fn create_user(connection: &Arc<PgConnection>) -> Result<User> {
        let user_repository = PgUserRepository::new(connection.clone());
        let user_id = user_repository.create_id()?;
        let user = User::new(&user_id);
        user_repository.create(&user)?;
        Ok(user)
    }

    fn establish_connection() -> Arc<PgConnection> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Arc::new(connection)
    }
}
