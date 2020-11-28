use crate::entity::{
    Credential, CredentialId, CredentialSecret, CredentialSecretExpiredAt,
    CredentialSecretWithExpiration, CredentialStatus, CredentialVerifiedAt, MailAddress, Password,
    UserId,
};
use crate::repository::CredentialRepository;
use crate::schema::{
    credential, credential_password_reset, credential_verification, credential_verified,
};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::{expression::nullable::Nullable, prelude::*, sql_types::*};
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

    fn columns() -> (
        credential::columns::id,
        credential::columns::user_id,
        credential::columns::mail_address,
        credential::columns::password,
        Nullable<credential_password_reset::columns::secret>,
        Nullable<credential_password_reset::columns::expired_at>,
        Nullable<credential_verification::columns::secret>,
        Nullable<credential_verification::columns::expired_at>,
        Nullable<credential_verified::columns::verified_at>,
    ) {
        (
            credential::id,
            credential::user_id,
            credential::mail_address,
            credential::password,
            credential_password_reset::secret.nullable(),
            credential_password_reset::expired_at.nullable(),
            credential_verification::secret.nullable(),
            credential_verification::expired_at.nullable(),
            credential_verified::verified_at.nullable(),
        )
    }

    fn credential_from_row(
        row: (
            i32,
            i32,
            String,
            String,
            Option<String>,
            Option<NaiveDateTime>,
            Option<String>,
            Option<NaiveDateTime>,
            Option<NaiveDateTime>,
        ),
    ) -> Result<Credential> {
        let (
            id,
            user_id,
            mail_address,
            password,
            password_reset_secret,
            password_reset_expired_at,
            verification_secret,
            verification_expired_at,
            verified_at,
        ) = row;
        let password_reset = match (password_reset_secret, password_reset_expired_at) {
            (None, Some(_)) | (Some(_), None) => Err(anyhow!("invalid database")),
            (None, None) => Ok(None),
            (Some(s), Some(e)) => {
                let secret = s.parse().map_err(anyhow::Error::msg)?;
                let expired_at = CredentialSecretExpiredAt::from(e);
                let secret_with_expiration = CredentialSecretWithExpiration::of(expired_at, secret);
                Ok(Some(secret_with_expiration))
            }
        }?;
        let verification = match (verification_secret, verification_expired_at) {
            (None, Some(_)) | (Some(_), None) => Err(anyhow!("invalid database")),
            (None, None) => Ok(None),
            (Some(s), Some(e)) => {
                let secret = s.parse().map_err(anyhow::Error::msg)?;
                let expired_at = CredentialSecretExpiredAt::from(e);
                let secret_with_expiration = CredentialSecretWithExpiration::of(expired_at, secret);
                Ok(Some(secret_with_expiration))
            }
        }?;
        let status = match (verification, verified_at) {
            (None, None) | (Some(_), Some(_)) => Err(anyhow!("invalid database")),
            (None, Some(at)) => Ok(CredentialStatus::Verified(CredentialVerifiedAt::from(at))),
            (Some(secret_with_expiration), None) => Ok(CredentialStatus::WaitingForVerification(
                secret_with_expiration,
            )),
        }?;
        Ok(Credential::of(
            CredentialId::try_from(id).map_err(anyhow::Error::msg)?,
            UserId::try_from(user_id).map_err(anyhow::Error::msg)?,
            mail_address.parse().map_err(anyhow::Error::msg)?,
            password.parse().map_err(anyhow::Error::msg)?,
            password_reset,
            status,
        ))
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

impl TryFrom<&Credential> for CredentialPasswordResetRow {
    type Error = &'static str;
    fn try_from(
        credential: &Credential,
    ) -> std::result::Result<Self, <Self as TryFrom<&Credential>>::Error> {
        match credential.password_reset() {
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

impl TryFrom<&Credential> for CredentialVerifiedRow {
    type Error = &'static str;
    fn try_from(
        credential: &Credential,
    ) -> std::result::Result<Self, <Self as TryFrom<&Credential>>::Error> {
        match credential.status() {
            CredentialStatus::WaitingForVerification(_) => Err("invalid status"),
            CredentialStatus::Verified(verified_at) => Ok(Self {
                credential_id: credential.id().into(),
                verified_at: verified_at.into(),
            }),
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
            .map_err(anyhow::Error::msg)?;
        if let Some(verification) = CredentialVerificationRow::try_from(&credential).ok() {
            diesel::insert_into(credential_verification::table)
                .values(verification)
                .execute(self.connection.as_ref())
                .map_err(anyhow::Error::msg)?;
        }
        Ok(credential)
    }

    fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Credential>> {
        credential::table
            .left_outer_join(credential_password_reset::table)
            .left_outer_join(credential_verification::table)
            .left_outer_join(credential_verified::table)
            .select(Self::columns())
            .filter(credential::user_id.eq(i32::from(user_id.clone())))
            .get_results(self.connection.as_ref())
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|row| Self::credential_from_row(row).ok())
                    .collect()
            })
            .map_err(anyhow::Error::msg)
    }

    fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>> {
        let found = credential::table
            .left_outer_join(credential_password_reset::table)
            .left_outer_join(credential_verification::table)
            .left_outer_join(credential_verified::table)
            .select(Self::columns())
            .filter(credential::mail_address.eq(mail_address.to_string()))
            .first(self.connection.as_ref())
            .optional()
            .map_err(anyhow::Error::msg)?;
        found.map(Self::credential_from_row).transpose()
    }

    fn find_by_secret(&self, secret: &CredentialSecret) -> Result<Option<Credential>> {
        let found = credential::table
            .left_outer_join(credential_password_reset::table)
            .left_outer_join(credential_verification::table)
            .left_outer_join(credential_verified::table)
            .select(Self::columns())
            .filter(credential_verification::secret.eq(secret.to_string()))
            .first(self.connection.as_ref())
            .optional()
            .map_err(anyhow::Error::msg)?;
        found.map(Self::credential_from_row).transpose()
    }

    fn delete(&self, _: &CredentialId) -> Result<()> {
        todo!()
    }

    fn save(&self, credential: &Credential) -> Result<()> {
        if let Some(row) = CredentialPasswordResetRow::try_from(credential).ok() {
            diesel::delete(credential_password_reset::table)
                .filter(credential_password_reset::credential_id.eq(i32::from(credential.id())))
                .execute(self.connection.as_ref())
                .map(|_| ())
                .map_err(anyhow::Error::msg)?;
            diesel::insert_into(credential_password_reset::table)
                .values(row)
                .execute(self.connection.as_ref())
                .map(|_| ())
                .map_err(anyhow::Error::msg)?;
        }

        if CredentialVerificationRow::try_from(credential)
            .ok()
            .is_none()
        {
            diesel::delete(credential_verification::table)
                .filter(credential_verification::credential_id.eq(i32::from(credential.id())))
                .execute(self.connection.as_ref())
                .map(|_| ())
                .map_err(anyhow::Error::msg)?;
        }

        if let Some(row) = CredentialVerifiedRow::try_from(credential).ok() {
            diesel::delete(credential_verified::table)
                .filter(credential_verified::credential_id.eq(i32::from(credential.id())))
                .execute(self.connection.as_ref())
                .map(|_| ())
                .map_err(anyhow::Error::msg)?;
            diesel::insert_into(credential_verified::table)
                .values(row)
                .execute(self.connection.as_ref())
                .map(|_| ())
                .map_err(anyhow::Error::msg)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::User;
    use crate::pg::pg_user_repository::PgUserRepository;
    use crate::repository::UserRepository;

    #[test]
    fn test_scenario() {
        transaction(|connection| {
            let user = {
                let user_repository = PgUserRepository::new(connection.clone());
                let user_id = user_repository.create_id()?;
                let user = User::new(&user_id);
                user_repository.create(&user)?;
                user
            };
            let repository = PgCredentialRepository::new(connection.clone());

            let created = {
                let mail_address = "m@bouzuya.net".parse().unwrap();
                let password = "password".parse().unwrap();
                let created = repository.create(user.id(), &mail_address, &password)?;

                let found = repository.find_by_mail_address(&created.mail_address())?;
                assert_eq!(found, Some(created.clone()));
                created
            };

            {
                let secret = created.verification().unwrap().secret();
                let found = repository.find_by_secret(&secret)?;

                assert_eq!(found, Some(created.clone()));
            }

            let verified = {
                let secret = created.verification().unwrap().secret();
                let verified = created.verify(&secret)?;
                repository.save(&verified)?;

                let found = repository.find_by_mail_address(&verified.mail_address())?;
                assert_eq!(found, Some(verified.clone()));
                verified
            };

            let reset = {
                let reset = verified.reset_password()?;
                repository.save(&reset)?;

                let found = repository.find_by_mail_address(&reset.mail_address())?;
                assert_eq!(found, Some(reset.clone()));

                reset
            };

            {
                let found = repository.find_by_user_id(&user.id())?;

                assert_eq!(found, vec![reset.clone()]);
            }

            Ok(())
        });
    }

    fn transaction<F>(f: F)
    where
        F: FnOnce(Arc<PgConnection>) -> Result<()>,
    {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let connection = Arc::new(connection);
        connection
            .as_ref()
            .test_transaction::<(), anyhow::Error, _>(|| f(connection.clone()))
    }
}
